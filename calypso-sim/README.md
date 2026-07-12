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
| **Mock** | `--mock` | Heartbeat publishes for every CAN message with a `sim_freq` in the spec, at its configured frequency. Default when no other mode is chosen; defaults OFF when paired with `--key-map` or `--stream`. |
| **Interactive** | `--key-map FILE` | Raw-mode terminal — each keypress fires its bound action. Press `Ctrl+C` to exit. |
| **Replay** | `--play ACTION` (with `--key-map`) | Run one named action from the scenario file to completion (following invokes and sleeps), then exit. |
| **Stream** | `--stream` | JSON-RPC 2.0 over stdin/stdout — for agent-driven injection. |

Pick at most one foreground mode: `--key-map` (interactive, or replay with `--play ACTION`) or `--stream`. `--mock` may run alongside either as a background heartbeat — set it explicitly to override the default-off behavior in those modes.

## Quick reference

```
cargo run -- --list-topics                                        # enumerate topics, exit
cargo run                                                         # mock heartbeat
cargo run -- --key-map manual_sim_buttons.keymap.json             # interactive
cargo run -- --key-map manual_sim_buttons.keymap.json --mock      # + background heartbeat
cargo run -- --key-map manual_sim_buttons.keymap.json --play demo # replay the "demo" action
cargo run -- --stream                                             # JSON-RPC over stdio
cargo run -- -u 10.0.0.5:1883 ...                                 # remote broker
```

The `--enable-topic <REGEX>` and `--disable-topic <REGEX>` flags filter which topics the mock heartbeat publishes (whitelist / blacklist; mutually exclusive). Topics that lack a `sim_freq` in the CAN spec are listed at startup as a `Warning topics (not simulated): ...` line and can only be reached via `--key-map` or `--stream`.

## Topic-ownership model

Every topic has an owner: `mock` (default — mock publishes), `stream` (claimed by stream/keymap), or `silenced` (nobody publishes). The mock loop checks ownership before each publish, so claims from `--stream` or keymap modes cleanly override the heartbeat without fighting it. Releasing a topic returns it to `mock`.

## Scenario file (`--key-map` and `--play`)

A scenario is a JSON object mapping action names to **actions**. An action is an ordered list of **steps**, optionally bound to a keyboard `key` and given a `desc`:

```json
{
  "enter": { "key": "e", "steps": [{"topic": "Wheel/Buttons/button_id", "value": 5}] },
  "home": {
    "key": "h",
    "desc": "home pulse",
    "steps": [
      {"topic": "VCU/CarState/home_mode", "value": 1},
      {"sleep_ms": 10},
      {"topic": "VCU/CarState/home_mode", "value": 0}
    ]
  },
  "menu_wrap": { "key": "w", "steps": ["enter", "home"] },
  "demo": { "desc": "run via --play demo", "steps": ["menu_wrap", {"sleep_ms": 500}, "menu_wrap"] }
}
```

Each **step** is one of three shapes, disambiguated purely by form (so there is no order-dependent parsing):

| Step | Shape | What it does |
|---|---|---|
| **Publish** | `{"topic": …, "value": N}` or `{"topic": …, "values": [...]}` | Publish to a topic. Exactly one of `value` / `values`; optional `unit`. |
| **Sleep** | `{"sleep_ms": N}` | Wait N milliseconds before the next step. |
| **Invoke** | `"other_action"` (bare string) | Run another action's steps here — the reuse / composition primitive. |

- **Interactive** (`--key-map FILE`): each action with a `key` fires on that keypress.
- **Replay** (`--key-map FILE --play ACTION`): run `ACTION` to completion — following its invokes and `sleep_ms` waits — then exit. A replay program is just an action, so there is no separate script file.

The scenario is validated at load: every publish sets exactly one of `value` / `values`, every invoke names an action that exists, the invoke graph must be acyclic (so replays always terminate), and no two actions may claim the same `key`.

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
| `publish` | `{topic, value? \| values?, unit?}` | `{ts_us}`, or `{skipped: "silenced"}` on a silenced topic |
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
| Unit — scenario | `src/tests/keymap.rs` | The fragile scenario logic: the serde `untagged` step-shape disambiguation (invoke / publish / sleep, by shape not order), and load-time validation — unknown or cyclic invokes are rejected, and publishes must set exactly one of `value` / `values`. |
| Unit — CLI modes | `src/tests/cli.rs` | `run_mock` arbitration: heartbeat on by default, off under a foreground mode or `--list-topics`, forced on by explicit `--mock`. |
| Integration — stream | `tests/stream.rs` | Spawns the real `calypso-sim --stream` binary and checks the JSON-RPC contract (`list_topics` is non-empty, `publish` requires exactly one of `value`/`values`, malformed requests get `-32601`/`-32600`) plus an end-to-end ownership flow — claim → silence → release with the heartbeat running, which doubles as the regression guard for the `mock`/`stream`/`silenced` arbitration. |

The suite is deliberately small: each test guards logic a future change could silently break, not code that is obvious by reading it. Unit tests live in `src/tests/` — compiled into the crate under `cfg(test)`, so they reach internals via `use crate::…`; binary-driven tests live in the crate-root `tests/` dir, the only place Cargo sets `CARGO_BIN_EXE_calypso-sim`.

No broker is needed because `publish` only enqueues (the eventloop retries a missing broker rather than dropping the queue), so it still returns a `ts_us`, and ownership arbitration is answered entirely from the JSON-RPC responses. Observing the actual *bytes on the wire* — that a payload reaches a subscriber — needs a live broker, which in practice is **Siren** in the Docker compose stack (see the repo `Dockerfile`); a standalone end-to-end test broker is intentionally out of scope.

CI (`.github/workflows/calypso-sim-ci.yml`) runs the suite on any change under `calypso-sim/**` or its path-dependencies.
