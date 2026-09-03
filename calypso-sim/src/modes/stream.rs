use std::sync::LazyLock;

use crate::simulate_data::create_simulated_components;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_util::sync::CancellationToken;

use crate::publish::{Transport, publish_data, resolve_values};

/// JSON-RPC 2.0 over stdio. Reads one request per line from stdin, writes
/// one response per line to stdout. Diagnostics go to stderr.
///
/// Ownership is a startup partition (see [`crate::ownership`]), not a runtime
/// negotiation, so there are no claim/release/silence methods — a stream driver
/// carves its topics out of the mock heartbeat with `--disable-topic`, then just
/// publishes.
///
/// Methods:
/// * `publish` — `{topic, value | values, unit?}` → `{ts_us}`
/// * `list_topics` — `{}` → `{topics: [{name, unit}, ...]}`
/// * `ping` — `{}` → `{ok: true}`
pub async fn run(token: CancellationToken, transport: Transport) -> Result<(), String> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            () = token.cancelled() => break,
            line = reader.next_line() => match line {
                Ok(Some(line)) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let resp = handle_line(&line, &transport).await;
                    write_line(&resp).await;
                }
                Ok(None) => break, // stdin closed
                Err(e) => {
                    eprintln!("stream: stdin read error: {e}");
                    break;
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Request {
    #[serde(default)]
    jsonrpc: Option<String>,
    #[serde(default)]
    id: Option<Value>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    params: Value,
}

const ERR_PARSE: i32 = -32700;
const ERR_INVALID_REQUEST: i32 = -32600;
const ERR_METHOD_NOT_FOUND: i32 = -32601;
const ERR_INVALID_PARAMS: i32 = -32602;
const ERR_INTERNAL: i32 = -32603;

async fn handle_line(line: &str, transport: &Transport) -> Value {
    let request: Request = match serde_json::from_str(line) {
        Ok(r) => r,
        Err(e) => return error(Value::Null, ERR_PARSE, &format!("Parse error: {e}")),
    };

    if let Some(ver) = &request.jsonrpc
        && ver != "2.0"
    {
        return error(
            request.id.unwrap_or(Value::Null),
            ERR_INVALID_REQUEST,
            "jsonrpc version must be \"2.0\"",
        );
    }
    let id = request.id.unwrap_or(Value::Null);

    // A request with no `method` is well-formed JSON but not a valid JSON-RPC
    // call, so it is an Invalid Request (-32600), not a parse error (-32700).
    let Some(method) = request.method else {
        return error(id, ERR_INVALID_REQUEST, "missing `method`");
    };

    match method.as_str() {
        "publish" => handle_publish(id, request.params, transport).await,
        "list_topics" => handle_list_topics(id),
        "ping" => ok(id, json!({"ok": true})),
        other => error(
            id,
            ERR_METHOD_NOT_FOUND,
            &format!("Unknown method: {other}"),
        ),
    }
}

#[derive(Deserialize)]
struct PublishParams {
    topic: String,
    #[serde(default)]
    value: Option<f32>,
    #[serde(default)]
    values: Option<Vec<f32>>,
    #[serde(default)]
    unit: Option<String>,
}

async fn handle_publish(id: Value, params: Value, transport: &Transport) -> Value {
    let p: PublishParams = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &format!("Invalid params: {e}")),
    };

    let values = match resolve_values(p.value, p.values.as_deref()) {
        Ok(vs) => vs,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &e),
    };

    let unit = p.unit.unwrap_or_default();
    match publish_data(transport, &p.topic, &unit, &values).await {
        Ok(ts_us) => ok(id, json!({"ts_us": ts_us})),
        Err(e) => error(id, ERR_INTERNAL, &format!("publish failed: {e}")),
    }
}

/// Topic (name, unit) pairs for `list_topics`, computed once. Building the full
/// component set runs each component's RNG initializer — wasted work for the
/// static name/unit returned here — so cache it rather than rebuilding per call.
static TOPICS: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
    create_simulated_components()
        .into_iter()
        .map(|c| (c.name, c.unit))
        .collect()
});

fn handle_list_topics(id: Value) -> Value {
    let topics: Vec<Value> = TOPICS
        .iter()
        .map(|(name, unit)| json!({"name": name, "unit": unit}))
        .collect();
    ok(id, json!({"topics": topics}))
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "id is moved into the json! payload"
)]
fn ok(id: Value, result: impl serde::Serialize) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "id is moved into the json! payload"
)]
fn error(id: Value, code: i32, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

async fn write_line(value: &Value) {
    let mut s = serde_json::to_string(value).unwrap_or_else(|_| "{}".into());
    s.push('\n');
    // Async stdout so a slow/stalled stream consumer can't block a runtime
    // worker thread (the read side is already async).
    let mut out = tokio::io::stdout();
    let _ = out.write_all(s.as_bytes()).await;
    let _ = out.flush().await;
}
