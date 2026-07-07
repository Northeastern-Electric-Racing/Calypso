//! JSON-RPC protocol conformance for `calypso-sim --stream`. Spawns the real
//! binary and checks the wire contract — no broker required.
mod common;

use common::StreamHarness;
use serde_json::json;

#[test]
fn ping_responds_ok() {
    let mut sim = StreamHarness::spawn();
    assert!(sim.ping(), "ping should return {{ok: true}}");
}

#[test]
fn list_topics_is_nonempty_and_well_formed() {
    let mut sim = StreamHarness::spawn();
    let topics = sim.list_topics();
    assert!(
        !topics.is_empty(),
        "the CAN spec should yield at least one simulatable topic"
    );
    for topic in &topics {
        assert!(
            topic.get("name").and_then(|v| v.as_str()).is_some(),
            "topic entry missing string `name`: {topic}"
        );
        assert!(
            topic.get("unit").is_some(),
            "topic entry missing `unit`: {topic}"
        );
    }
}

#[test]
fn publish_returns_a_timestamp() {
    let mut sim = StreamHarness::spawn();
    let result = sim.publish("VCU/CarState/speed", 12.5);
    assert!(
        result["ts_us"].as_u64().unwrap_or(0) > 0,
        "publish should echo a microsecond timestamp, got {result}"
    );
}

#[test]
fn publish_without_a_value_is_invalid_params() {
    let mut sim = StreamHarness::spawn();
    let (code, _) = sim
        .call("publish", json!({"topic": "VCU/CarState/speed"}))
        .expect_err("publish with no value/values must error");
    assert_eq!(code, -32602, "expected Invalid params");
}

#[test]
fn publish_with_both_value_and_values_is_invalid_params() {
    let mut sim = StreamHarness::spawn();
    let (code, _) = sim
        .call(
            "publish",
            json!({"topic": "X", "value": 1.0, "values": [1.0, 2.0]}),
        )
        .expect_err("value + values together must error");
    assert_eq!(code, -32602);
}

#[test]
fn unknown_method_is_method_not_found() {
    let mut sim = StreamHarness::spawn();
    let (code, _) = sim
        .call("frobnicate", json!({}))
        .expect_err("unknown method must error");
    assert_eq!(code, -32601);
}

#[test]
fn wrong_jsonrpc_version_is_invalid_request() {
    let mut sim = StreamHarness::spawn();
    let resp = sim.raw(json!({"jsonrpc": "2.1", "method": "ping", "params": {}}));
    assert_eq!(
        resp["error"]["code"].as_i64(),
        Some(-32600),
        "jsonrpc != 2.0 must be rejected, got {resp}"
    );
}
