# calypso-sim

Standalone MQTT simulation tool. Publishes simulated messages into the same broker the main `calypso` decoder uses, for testing UIs and dependent services without a live CAN bus.

`calypso-sim` is its own crate (separate from `calypso`); build and run it from this directory.

## Build

```
cd calypso-sim
cargo build --release
```

## Input modes

| Mode | Flag | What it does |
|---|---|---|
| **Mock** | `--mock` | Heartbeat publishes for every CAN message with a `sim_freq` in the spec, at its configured frequency. Default when no other mode is chosen; defaults OFF when paired with `--key-map`, `--script`, or `--stream`. |
| **Interactive** | `--key-map FILE` | Raw-mode terminal — each keypress fires a configured topic. Press `Ctrl+C` to exit. |
| **Script** | `--script FILE` (with `--key-map`) | Replay a sequence of keymap keys / `sleep N` lines from a text file, then exit. |
| **Stream** | `--stream` | JSON-RPC 2.0 over stdin/stdout — for agent-driven injection. |

Pick at most one foreground mode: `--key-map` (with optional `--script`) or `--stream`. `--mock` may run alongside either as a background heartbeat — set it explicitly to override the default-off behavior in those modes.

## Quick reference

```
cargo run -- --list-topics                       # enumerate topics, exit
cargo run                                        # mock heartbeat
cargo run -- --key-map manual_sim_keymap.example.json
cargo run -- --key-map keys.json --mock          # background heartbeat + interactive
cargo run -- --key-map keys.json --script play.txt
cargo run -- --stream                            # JSON-RPC over stdio
cargo run -- -u 10.0.0.5:1883 ...                # remote broker
```

The `--enable-topic <REGEX>` and `--disable-topic <REGEX>` flags filter which topics the mock heartbeat publishes (whitelist / blacklist; mutually exclusive). Topics that lack a `sim_freq` in the CAN spec are listed at startup as a `Warning topics (not simulated): ...` line and can only be reached via `--key-map` or `--stream`.

## Topic-ownership model

Every topic has an owner: `mock` (default — mock publishes), `stream` (claimed by stream/keymap), or `silenced` (nobody publishes). The mock loop checks ownership before each publish, so claims from `--stream` or keymap modes cleanly override the heartbeat without fighting it. Releasing a topic returns it to `mock`.

## Keymap format (`--key-map` and `--script`)

A keymap is a JSON object mapping single-character keys to one of four entry shapes:

```json
{
  "v": "BMS/Pack/Voltage",
  "p": {"topic": "VCU/CarState/home_mode", "value": 1, "desc": "home pulse"},
  "n": {"topic": "VCU/CarState/nero_index", "value": 0, "step": 1, "max": 5, "desc": "cycle nero"},
  "w": {
    "desc": "wrap menu",
    "sequence": [
      {"topic": "VCU/CarState/nero_index", "value": 0},
      {"topic": "VCU/CarState/home_mode", "value": 1, "delay_ms": 10},
      {"topic": "VCU/CarState/home_mode", "value": 0, "delay_ms": 100}
    ]
  }
}
```

| Form | What it does |
|---|---|
| Bare topic string | **Random** — publishes a fresh randomized value within the topic's sim bounds. Entries whose topic isn't in the spec are warned about on stderr and skipped at load. |
| Object with `value` | **Pinned** — publishes that exact number every keypress. `unit` is required for unknown topics. |
| Object with `step` | **Increment** — emits the starting value first, then advances by `step` each keypress. Start is `value` if set, else `min`, else `0`. `min`/`max` wrap independently when supplied. |
| Object with `sequence` | **Sequence** — publishes a scripted series of `(topic, value)` pairs with optional per-step `delay_ms`. |

Every object form accepts an optional `desc` for the startup listing and inline log line.

Script files (`--script`) contain one command per line: a single character (fires that key) or `sleep <ms>`. Blank lines and `#` comments are ignored.

## Stream mode protocol (`--stream`)

JSON-RPC 2.0 over stdio — one request per line on stdin, one response per line on stdout, diagnostics on stderr.

```jsonc
// stdin
{"jsonrpc":"2.0","id":1,"method":"publish","params":{"topic":"Wheel/Buttons/button_id","value":5}}
{"jsonrpc":"2.0","id":2,"method":"claim","params":{"topic":"VCU/CarState/home_mode"}}
{"jsonrpc":"2.0","id":3,"method":"publish","params":{"topic":"VCU/CarState/home_mode","value":1}}
{"jsonrpc":"2.0","id":4,"method":"release","params":{"topic":"VCU/CarState/home_mode"}}

// stdout
{"jsonrpc":"2.0","id":1,"result":{"ts_us":1735347123456789}}
{"jsonrpc":"2.0","id":2,"result":{"topic":"...","previous_owner":"mock","owner":"stream"}}
...
```

| Method | Params | Result |
|---|---|---|
| `publish` | `{topic, value? \| values?, unit?}` | `{ts_us}` |
| `claim` | `{topic}` | `{topic, previous_owner, owner}` |
| `release` | `{topic}` | `{topic, previous_owner, owner: "mock"}` |
| `silence` | `{topic}` | `{topic, previous_owner, owner: "silenced"}` |
| `status` | `{}` | `{overrides: [{topic, owner}, ...]}` |
| `list_topics` | `{}` | `{topics: [{name, unit}, ...]}` |
| `ping` | `{}` | `{ok: true}` |

Errors follow JSON-RPC 2.0 (`{error: {code, message}}`) with the standard codes: `-32700` (parse), `-32600` (invalid request), `-32601` (method not found), `-32602` (invalid params), and `-32603` (internal).

## Testing

The tests need **no broker** — just run:

```
cd calypso-sim
cargo test
```

| Layer | Where | What it checks |
|---|---|---|
| Unit — keymap | `src/tests/keymap.rs` | The two fragile pieces of keymap logic: increment emit-then-wrap/saturate (`advance_increment`), and the serde `untagged` shape disambiguation, whose result depends on variant *order* (`step` → increment beats `value` → pinned; `sequence` wins). |
| Unit — CLI modes | `src/tests/cli.rs` | `run_mock` arbitration: heartbeat on by default, off under a foreground mode or `--list-topics`, forced on by explicit `--mock`. |
| Integration — stream | `tests/stream.rs` | Spawns the real `calypso-sim --stream` binary and checks the JSON-RPC contract (`list_topics` is non-empty, `publish` requires exactly one of `value`/`values`, malformed requests get `-32601`/`-32600`) plus an end-to-end ownership flow — claim → silence → release with the heartbeat running, which doubles as the regression guard for the `mock`/`stream`/`silenced` arbitration. |

The suite is deliberately small: each test guards logic a future change could silently break, not code that is obvious by reading it. Unit tests live in `src/tests/` — compiled into the crate under `cfg(test)`, so they reach internals via `use crate::…`; binary-driven tests live in the crate-root `tests/` dir, the only place Cargo sets `CARGO_BIN_EXE_calypso-sim`.

No broker is needed because `publish` only enqueues (the eventloop retries a missing broker rather than dropping the queue), so it still returns a `ts_us`, and ownership arbitration is answered entirely from the JSON-RPC responses. Observing the actual *bytes on the wire* — that a payload reaches a subscriber — needs a live broker, which in practice is **Siren** in the Docker compose stack (see the repo `Dockerfile`); a standalone end-to-end test broker is intentionally out of scope.

CI (`.github/workflows/calypso-sim-ci.yml`) runs the suite on any change under `calypso-sim/**` or its path-dependencies.
