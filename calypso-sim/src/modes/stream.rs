use crate::simulate_data::create_simulated_components;
use rumqttc::v5::AsyncClient;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_util::sync::CancellationToken;

use crate::publish::publish_data;
use crate::registry::{Owner, SharedRegistry};

/// JSON-RPC 2.0 over stdio. Reads one request per line from stdin, writes
/// one response per line to stdout. Diagnostics go to stderr.
///
/// Methods:
/// * `publish` — `{topic, value | values, unit?}` → `{ts_us}`
/// * `claim` — `{topic}` → `{previous_owner, owner}`
/// * `release` — `{topic}` → `{previous_owner, owner}` (sets owner=auto)
/// * `silence` — `{topic}` → `{previous_owner, owner}`
/// * `status` — `{}` → `{overrides: [{topic, owner}, ...]}`
/// * `list_topics` — `{}` → `{topics: [{name, unit}, ...]}`
/// * `ping` — `{}` → `{ok: true}`
pub async fn run(
    token: CancellationToken,
    client: AsyncClient,
    registry: SharedRegistry,
) -> Result<(), String> {
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
                    let resp = handle_line(&line, &client, &registry).await;
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
    method: String,
    #[serde(default)]
    params: Value,
}

const ERR_PARSE: i32 = -32700;
const ERR_INVALID_REQUEST: i32 = -32600;
const ERR_METHOD_NOT_FOUND: i32 = -32601;
const ERR_INVALID_PARAMS: i32 = -32602;
const ERR_INTERNAL: i32 = -32603;

async fn handle_line(line: &str, client: &AsyncClient, registry: &SharedRegistry) -> Value {
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

    match request.method.as_str() {
        "publish" => handle_publish(id, request.params, client, registry).await,
        "claim" => handle_set(id, request.params, registry, Owner::Stream).await,
        "release" => handle_set(id, request.params, registry, Owner::Auto).await,
        "silence" => handle_set(id, request.params, registry, Owner::Silenced).await,
        "status" => handle_status(id, registry).await,
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

async fn handle_publish(
    id: Value,
    params: Value,
    client: &AsyncClient,
    registry: &SharedRegistry,
) -> Value {
    let p: PublishParams = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &format!("Invalid params: {e}")),
    };

    let values = match (p.value, p.values) {
        (Some(_), Some(_)) => {
            return error(
                id,
                ERR_INVALID_PARAMS,
                "specify `value` or `values`, not both",
            );
        }
        (Some(v), None) => vec![v],
        (None, Some(vs)) if !vs.is_empty() => vs,
        // None/None or None/Some(empty)
        _ => return error(id, ERR_INVALID_PARAMS, "missing `value` or `values`"),
    };

    if registry.read().await.owner(&p.topic) == Owner::Silenced {
        return ok(id, json!({"skipped": "silenced"}));
    }

    let unit = p.unit.unwrap_or_default();
    match publish_data(client, &p.topic, &unit, &values).await {
        Ok(ts_us) => ok(id, json!({"ts_us": ts_us})),
        Err(e) => error(id, ERR_INTERNAL, &format!("publish failed: {e}")),
    }
}

#[derive(Deserialize)]
struct TopicParam {
    topic: String,
}

async fn handle_set(id: Value, params: Value, registry: &SharedRegistry, owner: Owner) -> Value {
    let p: TopicParam = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &format!("Invalid params: {e}")),
    };

    let prev = registry.write().await.set(&p.topic, owner);
    ok(
        id,
        json!({"topic": p.topic, "previous_owner": prev.as_str(), "owner": owner.as_str()}),
    )
}

async fn handle_status(id: Value, registry: &SharedRegistry) -> Value {
    let snap = registry.read().await.snapshot();
    let entries: Vec<_> = snap
        .into_iter()
        .map(|(t, o)| json!({"topic": t, "owner": o.as_str()}))
        .collect();
    ok(id, json!({"overrides": entries}))
}

fn handle_list_topics(id: Value) -> Value {
    let components = create_simulated_components();
    let topics: Vec<_> = components
        .iter()
        .map(|c| json!({"name": c.name, "unit": c.unit}))
        .collect();
    ok(id, json!({"topics": topics}))
}

#[allow(clippy::needless_pass_by_value)]
fn ok(id: Value, result: impl serde::Serialize) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

#[allow(clippy::needless_pass_by_value)]
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
