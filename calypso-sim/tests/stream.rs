//! Integration coverage for `calypso-sim --stream`: spawns the real binary and
//! drives its JSON-RPC-over-stdio protocol.
//!
//! No broker required. `AsyncClient::publish` only enqueues, and the sim's
//! eventloop poller retries a missing broker instead of dropping the queue (see
//! `modes::poll_eventloop`), so every `publish` still returns a `ts_us`.
//! Observing the actual bytes on the wire needs a live broker (Siren, in the
//! Docker compose stack) and is intentionally out of scope — see `README.md`.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

/// A live `calypso-sim --stream` child process plus its stdio handles.
struct Sim {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: i64,
}

impl Sim {
    /// Spawn the sim in stream mode with the mock heartbeat off.
    fn spawn() -> Self {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_calypso-sim"));
        // A closed port: no broker is needed, and this keeps the sim off any
        // real broker a developer happens to be running.
        cmd.arg("-u").arg("127.0.0.1:47654").arg("--stream");
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().expect("failed to spawn calypso-sim binary");
        let stdin = child.stdin.take().expect("child stdin");
        let stdout = BufReader::new(child.stdout.take().expect("child stdout"));

        // Drain stderr on a background thread so the child never blocks on a full
        // pipe (it logs an MQTT connection error ~every 500ms against the closed
        // broker). The thread exits on EOF when the child is killed.
        let stderr = child.stderr.take().expect("child stderr");
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            while reader.read_line(&mut line).is_ok_and(|n| n > 0) {
                line.clear();
            }
        });

        Sim {
            child,
            stdin,
            stdout,
            next_id: 1,
        }
    }

    /// Send a request object (an `id` is injected) and return the full response.
    fn raw(&mut self, mut request: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        request["id"] = json!(id);

        let mut line = serde_json::to_string(&request).expect("serialize request");
        line.push('\n');
        self.stdin
            .write_all(line.as_bytes())
            .expect("write request");
        self.stdin.flush().expect("flush request");

        let mut response = String::new();
        let n = self.stdout.read_line(&mut response).expect("read response");
        assert!(n > 0, "sim closed stdout before responding");
        serde_json::from_str(response.trim())
            .unwrap_or_else(|e| panic!("non-JSON response {response:?}: {e}"))
    }

    /// Call `method` with `params`: `Ok(result)` or `Err((code, message))`.
    fn call(&mut self, method: &str, params: Value) -> Result<Value, (i64, String)> {
        let mut request = json!({"jsonrpc": "2.0", "method": method});
        request["params"] = params; // moves `params` into the request
        let resp = self.raw(request);
        if let Some(err) = resp.get("error").filter(|e| !e.is_null()) {
            return Err((
                err["code"].as_i64().unwrap_or(0),
                err["message"].as_str().unwrap_or_default().to_string(),
            ));
        }
        Ok(resp.get("result").cloned().unwrap_or(Value::Null))
    }

    /// Call `method`, panicking on any JSON-RPC error (success path).
    fn ok(&mut self, method: &str, params: Value) -> Value {
        self.call(method, params)
            .unwrap_or_else(|(code, msg)| panic!("{method} failed: [{code}] {msg}"))
    }
}

impl Drop for Sim {
    fn drop(&mut self) {
        // Kill and reap so no sim process (or its stderr thread) outlives the test.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn list_topics_is_nonempty_and_well_formed() {
    let mut sim = Sim::spawn();
    let result = sim.ok("list_topics", json!({}));
    let topics = result["topics"].as_array().expect("topics array");
    assert!(
        !topics.is_empty(),
        "the spec should yield simulatable topics"
    );
    for t in topics {
        assert!(
            t["name"].as_str().is_some(),
            "topic missing string name: {t}"
        );
        assert!(t.get("unit").is_some(), "topic missing unit: {t}");
    }
}

#[test]
fn publish_requires_exactly_one_of_value_or_values() {
    let mut sim = Sim::spawn();
    // Neither present -> invalid params.
    let (code, _) = sim
        .call("publish", json!({"topic": "T"}))
        .expect_err("publish with no value/values must error");
    assert_eq!(code, -32602, "expected Invalid params");
    // Both present -> invalid params.
    let (code, _) = sim
        .call(
            "publish",
            json!({"topic": "T", "value": 1.0, "values": [1.0, 2.0]}),
        )
        .expect_err("value + values together must error");
    assert_eq!(code, -32602);
    // Exactly one -> accepted, returns a timestamp (publish only enqueues, so no
    // broker is needed for this to succeed).
    assert!(
        sim.ok("publish", json!({"topic": "T", "value": 1.0}))["ts_us"]
            .as_u64()
            .unwrap_or(0)
            > 0,
        "a well-formed publish must return a ts_us"
    );
}

#[test]
fn malformed_requests_are_rejected_with_standard_codes() {
    let mut sim = Sim::spawn();
    // Unknown method -> Method not found.
    let (code, _) = sim
        .call("frobnicate", json!({}))
        .expect_err("unknown method must error");
    assert_eq!(code, -32601, "unknown method -> method not found");
    // A jsonrpc version other than "2.0" -> Invalid request.
    let resp = sim.raw(json!({"jsonrpc": "2.1", "method": "ping", "params": {}}));
    assert_eq!(
        resp["error"]["code"].as_i64(),
        Some(-32600),
        "jsonrpc != 2.0 must be rejected, got {resp}"
    );
    // A request missing `method` is valid JSON but not a valid JSON-RPC call.
    let resp = sim.raw(json!({"jsonrpc": "2.0", "params": {}}));
    assert_eq!(
        resp["error"]["code"].as_i64(),
        Some(-32600),
        "missing method must be rejected as invalid request, got {resp}"
    );
}
