use std::sync::LazyLock;

use crate::simulate_data::create_simulated_components;

use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio_util::sync::CancellationToken;

use crate::filter::{FilterMode, FilterTx};
use crate::publish::{Transport, publish_data, resolve_values};
use crate::runtime_topic::{self, SimCommand, SimCommandTx, TopicSpec};

/// JSON-RPC 2.0 over stdio. Reads one request per line from stdin, writes
/// one response per line to stdout. Diagnostics go to stderr.
///
/// Nothing arbitrates between this driver and the mock heartbeat, so there are
/// no claim/release/silence methods. To keep the heartbeat off the topics this
/// driver publishes, mute them with `--disable-topic` at startup or `set_filter`
/// while running (see [`crate::filter`]).
///
/// Methods:
/// * `publish` — `{topic, value | values, unit?}` → `{ts_us}`
/// * `list_topics` — `{}` → `{topics: [{name, unit}, ...]}`
/// * `set_filter` — `{mode: "disable"|"enable"|"clear", patterns?: [regex]}` →
///   `{filter}`; retunes which topics the mock heartbeat drives, live
/// * `add_topic` — `{topic, unit?, sim_freq, sim}` → `{topic}`; starts
///   simulating a topic that was not compiled in
/// * `remove_topic` — `{topic}` → `{topic, removed}`; stops simulating one
/// * `ping` — `{}` → `{ok: true}`
pub async fn run(
    token: CancellationToken,
    transport: Transport,
    filter_tx: FilterTx,
    cmd_tx: SimCommandTx,
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
                    let resp = handle_line(&line, &transport, &filter_tx, &cmd_tx).await;
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

async fn handle_line(
    line: &str,
    transport: &Transport,
    filter_tx: &FilterTx,
    cmd_tx: &SimCommandTx,
) -> Value {
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
        "list_topics" => handle_list_topics(id, cmd_tx).await,
        "set_filter" => handle_set_filter(id, request.params, filter_tx),
        "add_topic" => handle_add_topic(id, request.params, cmd_tx).await,
        "remove_topic" => handle_remove_topic(id, request.params, cmd_tx).await,
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

#[derive(Deserialize)]
struct FilterParams {
    mode: String,
    #[serde(default)]
    patterns: Vec<String>,
}

/// Replace the mock heartbeat's topic filter while the sim runs. Declarative:
/// each call states the whole resulting filter rather than accumulating, so a
/// driver never has to track what it set before.
fn handle_set_filter(id: Value, params: Value, filter_tx: &FilterTx) -> Value {
    let p: FilterParams = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &format!("Invalid params: {e}")),
    };

    match FilterMode::from_command(&p.mode, &p.patterns) {
        Ok(filter) => {
            let described = filter.describe();
            // The mock task may not be running (`--stream` without `--mock`);
            // the filter is still recorded, so this is not an error.
            let _ = filter_tx.send(filter);
            ok(id, json!({"filter": described}))
        }
        Err(e) => error(id, ERR_INVALID_PARAMS, &e),
    }
}

/// Start simulating a topic the binary was not built with. The heartbeat owns
/// the component list, so the outcome — including whether the name was already
/// taken — comes back from it rather than being decided here.
async fn handle_add_topic(id: Value, params: Value, cmd_tx: &SimCommandTx) -> Value {
    let spec: TopicSpec = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &format!("Invalid params: {e}")),
    };
    let topic = spec.topic.clone();
    let component = match runtime_topic::build(spec) {
        Ok(c) => c,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &e),
    };

    let added = runtime_topic::request(cmd_tx, |reply| SimCommand::Add {
        component: Box::new(component),
        reply,
    })
    .await;
    match added {
        Err(e) => error(id, ERR_INTERNAL, &e),
        Ok(Err(e)) => error(id, ERR_INVALID_PARAMS, &e),
        Ok(Ok(())) => ok(id, json!({"topic": topic})),
    }
}

/// Stop simulating a topic. Reports how many components went, which is not
/// always one: the in-topic placeholder names repeat in the generated set.
/// Removing a name that is not simulated is a no-op, not an error.
async fn handle_remove_topic(id: Value, params: Value, cmd_tx: &SimCommandTx) -> Value {
    #[derive(Deserialize)]
    struct RemoveParams {
        topic: String,
    }

    let p: RemoveParams = match serde_json::from_value(params) {
        Ok(v) => v,
        Err(e) => return error(id, ERR_INVALID_PARAMS, &format!("Invalid params: {e}")),
    };

    let removed = runtime_topic::request(cmd_tx, |reply| SimCommand::Remove {
        name: p.topic.clone(),
        reply,
    })
    .await;
    match removed {
        Ok(removed) => ok(id, json!({"topic": p.topic, "removed": removed})),
        Err(e) => error(id, ERR_INTERNAL, &e),
    }
}

/// The compiled topic set, computed once. Building the full component set runs
/// each component's RNG initializer — wasted work for the static name/unit
/// returned here — so cache it rather than rebuilding per call.
///
/// Only a fallback: once topics can be added and removed at runtime this is no
/// longer the live set, so `list_topics` asks the heartbeat first.
static COMPILED_TOPICS: LazyLock<Vec<(String, String)>> = LazyLock::new(|| {
    create_simulated_components()
        .into_iter()
        .map(|c| (c.name, c.unit))
        .collect()
});

async fn handle_list_topics(id: Value, cmd_tx: &SimCommandTx) -> Value {
    fn to_json(topics: &[(String, String)]) -> Vec<Value> {
        topics
            .iter()
            .map(|(name, unit)| json!({"name": name, "unit": unit}))
            .collect()
    }

    // Falls back to the compiled set when the heartbeat is not running: nothing
    // can have been added in that case, so the two agree. The fallback reads the
    // cache in place — cloning it would undo the point of caching it.
    let topics = match runtime_topic::request(cmd_tx, |reply| SimCommand::List { reply }).await {
        Ok(live) => to_json(&live),
        Err(_) => to_json(&COMPILED_TOPICS),
    };
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
