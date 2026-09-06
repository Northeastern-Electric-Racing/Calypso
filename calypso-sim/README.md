# calypso-sim

Standalone CAN simulation tool. Publishes simulated messages onto the same wire the main `calypso` decoder uses — MQTT by default, or Zenoh with `--zenoh` — for testing UIs and dependent services without a live CAN bus.

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
cargo run -- -u 10.0.0.5:1883 ...                                 # remote MQTT broker
cargo run -- --zenoh                                              # publish over Zenoh
cargo run -- --zenoh --zenoh-conf zenoh.json5                     # ... with an explicit conf
cargo run -- --stream --mock                                      # stream + live add/remove
```

## Transport (`--zenoh`)

Every mode above is transport-agnostic; the wire is chosen once at startup.

| Flag | Wire |
|---|---|
| *(default)* | MQTT to `--siren-host-url` (`-u`, default `localhost:1883`), QoS 0. |
| `--zenoh` / `-z` | Zenoh `put` on the topic as the key expression, encoded `application/protobuf`. |

The payload is the same `ServerData` protobuf either way, so a Zenoh-mode sim feeds the `calypso` decoder's own `--zenoh` path. `--zenoh-conf FILE` (requires `--zenoh`) loads a JSON5 Zenoh config; omit it and the Zenoh defaults are used, so no conf file is needed to get running. `-u` is ignored under `--zenoh`.

Both flags also read from the environment (`CALYPSO_ZENOH`, `CALYPSO_ZENOH_CONF`), matching the main `calypso` binary.

## Topic filtering (live)

The mock heartbeat publishes randomized values for every topic with a `sim_freq`
in the CAN spec. `--enable-topic <REGEX>` / `--disable-topic <REGEX>` restrict
that set (whitelist / blacklist, mutually exclusive), and the active set is
logged whenever it changes.

This is a **filter, not an ownership claim**. The heartbeat and a foreground
driver may publish the same topic; nothing arbitrates between them. If the
heartbeat overwriting your injected values matters for what you are testing,
mute its copy — the shipped `manual_sim_buttons.keymap.json` overlaps the
heartbeat on the three `VCU/CarState/*` topics, which republish every 250 ms:

```
cargo run -- --key-map manual_sim_buttons.keymap.json --mock --disable-topic '^VCU/CarState/'
```

The filter can be retuned **while the sim runs**, so you can mute a noisy
subsystem without a restart. How depends on which mode owns stdin:

| Mode | Control surface |
|---|---|
| plain `--mock` | line commands on stdin (stdin is otherwise unused) |
| `--stream` | the `set_filter` JSON-RPC method |
| `--key-map` | none — the terminal is in raw mode for keypresses |

Stdin commands, one per line:

```
disable <regex>...   publish everything EXCEPT these
enable  <regex>...   publish ONLY these
clear                remove the filter (all topics)
status               print the current filter
help                 list these commands
```

`enable` and `disable` each replace the filter rather than accumulating, so one
command always states the whole resulting filter — the same mutual exclusion the
CLI flags have. A bad regex is reported and the previous filter stays in effect.

Topics that lack a `sim_freq` in the CAN spec are listed at startup as a
`Warning topics (not simulated): ...` line. They are not compiled into the
binary at all, so no filter can reach them — see the next section.

## Adding and removing simulated topics (live)

The filter above can only mask topics the binary was **built** with. The
`gen_simulate_data!()` macro skips any CAN message without a `sim_freq`, so a
large share of the spec's fields never become components and cannot be enabled
by any filter. To simulate one of those, add it at runtime.

Parameters come from you rather than the CAN spec: the runtime Docker image
ships only the binaries, so there is no spec to read where the sim actually
runs. The value shapes mirror the spec's own `sim` block, so a definition can be
copied straight out of Odyssey-Definitions.

Both surfaces require the mock heartbeat (`--mock`) — it owns the component
list. Without it you get `mock heartbeat is not running`.

**Stream RPC:**

```json
{"method":"add_topic","params":{"topic":"BMS/Pack/SoC","unit":"%","sim_freq":750,
  "sim":{"min":0,"max":100,"inc_min":1,"inc_max":5}}}

{"method":"add_topic","params":{"topic":"BMS/Status/State","unit":"","sim_freq":250,
  "sim":{"options":[[0,0.7],[1,0.2],[2,0.1]]}}}

{"method":"remove_topic","params":{"topic":"BMS/Pack/SoC"}}
```

**Stdin** (plain `--mock`), continuous range only — a weighted option set is too
unwieldy for a command line, so use the RPC for those:

```
add <topic> <unit> <freq_ms> <min> <max>
remove <topic>
```

The two value shapes:

| shape | for | fields |
|---|---|---|
| range | continuous sensors — voltage, temperature, RPM | `min`, `max`, `inc_min`, `inc_max`, `round?` |
| discrete | states, modes, fault flags | `options: [[value, weight], ...]` |

Discrete weights are relative and normalised, so `[[0,7],[1,3]]` and
`[[0,0.7],[1,0.3]]` are the same distribution.

Rejected up front: a topic already being simulated (this is add-only, not an
upsert — `remove` then `add` to change one), a `sim_freq` of zero or less (zero
would publish on every 5 ms tick), a name containing `{}` (those need in-topic
placeholder interpolation a runtime topic cannot supply), and range bounds that
are non-finite or too wide to sample.

Two behaviours worth knowing:

- **`remove` reports a count, which is not always 1.** Names are not unique in
  the generated set — the `{}` placeholder topics repeat — and `remove` drops
  every component under that name.
- **Removed is not the same as filtered.** `remove` deletes the component; the
  filter is untouched, so a later `clear` will not bring it back. An added topic
  *is* subject to the current filter, so one added while a whitelist excludes it
  sits inactive until the filter changes.

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
{"jsonrpc":"2.0","id":2,"method":"publish","params":{"topic":"VCU/CarState/home_mode","values":[1,0]}}

// stdout
{"jsonrpc":"2.0","id":1,"result":{"ts_us":1735347123456789}}
{"jsonrpc":"2.0","id":2,"result":{"ts_us":1735347123456999}}
...
```

| Method | Params | Result |
|---|---|---|
| `publish` | `{topic, value? \| values?, unit?}` | `{ts_us}` |
| `list_topics` | `{}` | `{topics: [{name, unit}, ...]}` |
| `ping` | `{}` | `{ok: true}` |

There are no `claim`/`release`/`silence` methods: nothing arbitrates between the heartbeat and a driver. To keep the heartbeat off a stream driver's topics, mute them with `--disable-topic` at startup or `set_filter` while running.

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
| Unit — CLI modes | `src/tests/cli.rs` | `run_mock` arbitration: heartbeat on by default, off under a foreground mode or `--list-topics`, forced on by explicit `--mock`. Plus that `--zenoh-conf` is not `requires = "zenoh"`, so an exported `CALYPSO_ZENOH_CONF` cannot block startup. |
| Unit — filter | `src/tests/filter.rs` | Whitelist / blacklist / no-filter selection, bad regexes rejected, and that an over-narrow whitelist silences the heartbeat rather than falling open. |
| Unit — runtime topics | `src/tests/runtime_topic.rs` | That an unusable spec is rejected at build time rather than at the first publish tick — non-finite or over-wide range bounds (which would panic the sampler), a `sim_freq` of zero, `{}` in the name — and that discrete weights become the running ceilings `SimValue` expects, ending at exactly 1.0. |
| Integration — stream | `tests/stream.rs` | Spawns the real `calypso-sim --stream` binary and checks the JSON-RPC contract: `list_topics` is non-empty and well-formed, `publish` requires exactly one of `value`/`values` (and a well-formed one returns a `ts_us`), and malformed requests get `-32601`/`-32600`. Also the runtime-topic lifecycle against a `--mock` sim — add, see it listed, remove — plus the rejections and the `--mock`-off error. |

The suite is deliberately small: each test guards logic a future change could silently break, not code that is obvious by reading it. Unit tests live in `src/tests/` — compiled into the crate under `cfg(test)`, so they reach internals via `use crate::…`; binary-driven tests live in the crate-root `tests/` dir, the only place Cargo sets `CARGO_BIN_EXE_calypso-sim`.

No broker is needed because the MQTT `publish` only enqueues (the eventloop retries a missing broker rather than dropping the queue), so it still returns a `ts_us`. Observing the actual *bytes on the wire* — that a payload reaches a subscriber — needs a live broker, which in practice is **Siren** in the Docker compose stack (see the repo `Dockerfile`); a standalone end-to-end test broker is intentionally out of scope.

CI (`.github/workflows/calypso-sim-ci.yml`) runs the suite on any change under `calypso-sim/**` or its path-dependencies.
