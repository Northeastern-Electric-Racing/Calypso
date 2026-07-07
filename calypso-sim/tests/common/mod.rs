//! Shared test harness: spawns `calypso-sim --stream` and drives its JSON-RPC
//! stdio protocol synchronously (one request, one response).
//!
//! No MQTT broker is required. `AsyncClient::publish` only enqueues, and the
//! sim's eventloop poller retries a missing broker instead of dropping the
//! queue (see `modes::poll_eventloop`), so every `publish` still returns a
//! `ts_us`. A broker is only needed to observe the *bytes on the wire*, which
//! is covered separately by the `encode_server_data` unit test in
//! `src/publish.rs`.
#![allow(dead_code)] // each test file uses a different subset of these helpers.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

/// A live `calypso-sim --stream` child process plus its stdio handles.
pub struct StreamHarness {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: i64,
}

impl StreamHarness {
    /// Spawn the sim in stream mode with the autonomous heartbeat off.
    pub fn spawn() -> Self {
        Self::spawn_inner(false)
    }

    /// Spawn with `--auto` so the autonomous heartbeat runs alongside the
    /// stream driver (used to exercise ownership isolation).
    pub fn spawn_with_auto() -> Self {
        Self::spawn_inner(true)
    }

    fn spawn_inner(with_auto: bool) -> Self {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_calypso-sim"));
        // Point at a closed port: no broker is needed, and this keeps the sim
        // from touching any real broker a developer happens to be running.
        cmd.arg("-u").arg("127.0.0.1:47654").arg("--stream");
        if with_auto {
            cmd.arg("--auto");
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .expect("failed to spawn calypso-sim binary for the stream test");
        let stdin = child.stdin.take().expect("child stdin");
        let stdout = BufReader::new(child.stdout.take().expect("child stdout"));

        // Drain stderr on a background thread so the child never blocks writing
        // to a full stderr pipe (it logs an MQTT connection error every ~500ms
        // against the closed broker). The thread exits on EOF when we kill it.
        let stderr = child.stderr.take().expect("child stderr");
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            while reader.read_line(&mut line).is_ok_and(|n| n > 0) {
                line.clear();
            }
        });

        StreamHarness {
            child,
            stdin,
            stdout,
            next_id: 1,
        }
    }

    /// Send a raw request object (an `id` is injected) and return the full
    /// response object. Lets negative tests craft malformed wire shapes.
    pub fn raw(&mut self, mut request: Value) -> Value {
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
        let n = self
            .stdout
            .read_line(&mut response)
            .expect("read response line");
        assert!(n > 0, "sim closed stdout before responding");
        serde_json::from_str(response.trim())
            .unwrap_or_else(|e| panic!("non-JSON response {response:?}: {e}"))
    }

    /// Call `method` with `params`, returning the `result` on success or
    /// `(code, message)` on a JSON-RPC error.
    pub fn call(&mut self, method: &str, params: Value) -> Result<Value, (i64, String)> {
        let mut request = json!({"jsonrpc": "2.0", "method": method});
        request["params"] = params; // moves `params` into the request object
        let resp = self.raw(request);
        if let Some(err) = resp.get("error").filter(|e| !e.is_null()) {
            let code = err["code"].as_i64().unwrap_or(0);
            let message = err["message"].as_str().unwrap_or_default().to_string();
            return Err((code, message));
        }
        Ok(resp.get("result").cloned().unwrap_or(Value::Null))
    }

    /// Call `method`, panicking on any JSON-RPC error (for the success path).
    fn expect_ok(&mut self, method: &str, params: Value) -> Value {
        self.call(method, params)
            .unwrap_or_else(|(code, msg)| panic!("{method} failed: [{code}] {msg}"))
    }

    pub fn ping(&mut self) -> bool {
        self.expect_ok("ping", json!({}))["ok"]
            .as_bool()
            .unwrap_or(false)
    }

    pub fn list_topics(&mut self) -> Vec<Value> {
        self.expect_ok("list_topics", json!({}))["topics"]
            .as_array()
            .cloned()
            .unwrap_or_default()
    }

    pub fn publish(&mut self, topic: &str, value: f64) -> Value {
        self.expect_ok("publish", json!({"topic": topic, "value": value}))
    }

    pub fn publish_unit(&mut self, topic: &str, value: f64, unit: &str) -> Value {
        self.expect_ok(
            "publish",
            json!({"topic": topic, "value": value, "unit": unit}),
        )
    }

    pub fn claim(&mut self, topic: &str) -> Value {
        self.expect_ok("claim", json!({"topic": topic}))
    }

    pub fn release(&mut self, topic: &str) -> Value {
        self.expect_ok("release", json!({"topic": topic}))
    }

    pub fn silence(&mut self, topic: &str) -> Value {
        self.expect_ok("silence", json!({"topic": topic}))
    }

    pub fn status(&mut self) -> Vec<Value> {
        self.expect_ok("status", json!({}))["overrides"]
            .as_array()
            .cloned()
            .unwrap_or_default()
    }

    /// The owner string for `topic` per the current `status`, or `None` when
    /// the topic carries no override (i.e. still `auto`).
    pub fn owner_of(&mut self, topic: &str) -> Option<String> {
        self.status()
            .into_iter()
            .find(|o| o["topic"] == json!(topic))
            .map(|o| o["owner"].as_str().unwrap_or_default().to_string())
    }
}

impl Drop for StreamHarness {
    fn drop(&mut self) {
        // The test is done with the child; kill and reap it so no sim process
        // (or its stderr-drain thread) lingers past the test.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
