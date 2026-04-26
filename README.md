# Calypso
Custom CAN decoder to translate CAN messages to MQTT protobuf encoded packets with low latency and a JSON configuration structure.

Usage: run `-h` to see the full usage options and defaults.

### Develop setup
#### Go to Settings in VSCode
search Rust-analyzer check and set the command from check -> clippy

#### Open Settings.json
add following information:
```
"[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
} 
```

To test it on linux, please install:
- mosquitto broker: https://mosquitto.org
- can-utils
- mqttui: https://github.com/EdJoPaTo/mqttui


### Developing

Process for testing:  
- run `mosquitto` and leave it open
- run `mqttui` and leave it open
- setup the can network:
    - `sudo ip link add dev vcan0 type vcan`
    - `sudo ip link set dev vcan0 up`

run ```cargo run -- -u localhost:1883 -c vcan0```

To send a CAN message:
- `cansend vcan0 <ID_IN_HEX>#<PAYLOAD_IN_HEX>`
Ex. `cansend vcan0 702#01010101FFFFFFFF`  
Now view calypso interpret the can message and broadcast it on `mqttui`


### Simulation (`calypso-sim`)

`calypso-sim` is a single binary that publishes simulated MQTT messages into the broker. It has four input modes that compose freely:

| Mode | Flag | What it does |
|---|---|---|
| **Autonomous** | `--auto` | Heartbeat publishes for every CAN message with a `sim_freq` in the spec, at its configured frequency. Default when no other mode is chosen. |
| **Interactive** | `--key-map FILE` | Raw-mode terminal — each keypress fires a configured topic. Press `Ctrl+C` to exit. |
| **Script** | `--script FILE` (with `--key-map`) | Replay a sequence of keymap keys / `sleep N` lines from a text file, then exit. |
| **Stream** | `--stream` | JSON-RPC 2.0 over stdin/stdout — for agent-driven injection. `--auto` defaults OFF in this mode. |

`--auto` can run alongside `--key-map` or `--stream` to provide a simulated background while you override specific topics.

#### Quick reference

```
cargo run --bin calypso-sim -- --list-topics                       # enumerate topics, exit
cargo run --bin calypso-sim                                        # autonomous heartbeat
cargo run --bin calypso-sim -- --key-map manual_sim_keymap.example.json
cargo run --bin calypso-sim -- --key-map keys.json --auto          # background heartbeat + interactive
cargo run --bin calypso-sim -- --key-map keys.json --script play.txt
cargo run --bin calypso-sim -- --stream                            # JSON-RPC over stdio
cargo run --bin calypso-sim -- -u 10.0.0.5:1883 ...                # remote broker
```

The `--enable-topic <REGEX>` and `--disable-topic <REGEX>` flags filter which topics the autonomous heartbeat publishes (whitelist / blacklist; mutually exclusive). Topics that lack a `sim_freq` in the CAN spec are listed at startup as a `Warning topics (not simulated): ...` line and can only be reached via `--key-map` or `--stream`.

#### Topic-ownership model

Every topic has an owner: `auto` (default — autonomous publishes), `stream` (claimed by stream/keymap), or `silenced` (nobody publishes). The autonomous loop checks ownership before each publish, so claims from `--stream` or keymap modes cleanly override the heartbeat without fighting it. Releasing a topic returns it to `auto`.

#### Keymap format (`--key-map` and `--script`)

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
| Bare topic string | **Random** — publishes a fresh randomized value within the topic's sim bounds. Topic must exist in the spec. |
| Object with `value` | **Pinned** — publishes that exact number every keypress. `unit` is required for unknown topics. |
| Object with `step` | **Increment** — emits the starting value first, then advances by `step` each keypress. `min`/`max` wrap independently when supplied. |
| Object with `sequence` | **Sequence** — publishes a scripted series of `(topic, value)` pairs with optional per-step `delay_ms`. |

Every object form accepts an optional `desc` for the startup listing and inline log line.

Script files (`--script`) contain one command per line: a single character (fires that key) or `sleep <ms>`. Blank lines and `#` comments are ignored.

#### Stream mode protocol (`--stream`)

JSON-RPC 2.0 over stdio — one request per line on stdin, one response per line on stdout, diagnostics on stderr.

```jsonc
// stdin
{"jsonrpc":"2.0","id":1,"method":"publish","params":{"topic":"Wheel/Buttons/button_id","value":5}}
{"jsonrpc":"2.0","id":2,"method":"claim","params":{"topic":"VCU/CarState/home_mode"}}
{"jsonrpc":"2.0","id":3,"method":"publish","params":{"topic":"VCU/CarState/home_mode","value":1}}
{"jsonrpc":"2.0","id":4,"method":"release","params":{"topic":"VCU/CarState/home_mode"}}

// stdout
{"jsonrpc":"2.0","id":1,"result":{"ts_us":1735347123456789}}
{"jsonrpc":"2.0","id":2,"result":{"topic":"...","previous_owner":"auto","owner":"stream"}}
...
```

| Method | Params | Result |
|---|---|---|
| `publish` | `{topic, value? \| values?, unit?}` | `{ts_us}` |
| `claim` | `{topic}` | `{topic, previous_owner, owner}` |
| `release` | `{topic}` | `{topic, previous_owner, owner: "auto"}` |
| `silence` | `{topic}` | `{topic, previous_owner, owner: "silenced"}` |
| `status` | `{}` | `{overrides: [{topic, owner}, ...]}` |
| `list_topics` | `{}` | `{topics: [{name, unit}, ...]}` |
| `ping` | `{}` | `{ok: true}` |

Errors follow JSON-RPC 2.0 (`{error: {code, message}}` with codes -32600 through -32700 and -32603 for internal failures).

#### Run from Docker

The default container entry point is the main `calypso` decoder. To run `calypso-sim` instead, override the entry point:

```
docker pull ghcr.io/northeastern-electric-racing/calypso:Develop
docker run -d --rm -e CALYPSO_SIREN_HOST_URL=127.0.0.1:1883 --network host \
    --entrypoint calypso-sim ghcr.io/northeastern-electric-racing/calypso:Develop
```
