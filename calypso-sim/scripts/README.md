# calypso-sim test scripts

Python harness that drives `calypso-sim --stream` to exercise the JSON-RPC protocol
under realistic load — mimicking what the Cerberus-2.0 VCU firmware would publish
onto the topics NERO consumes.

## Setup

```bash
brew install mosquitto                       # MQTT broker
python3 -m pip install --user paho-mqtt      # MQTT client
cd ..  &&  cargo build --release             # build calypso-sim
```

Optional visual aid:

```bash
brew install mqttui                          # live topic viewer
```

The Python protobuf bindings (`serverdata_pb2.py`) are checked in; regenerate if
`src/proto/serverdata.proto` changes:

```bash
protoc --python_out=scripts -Isrc/proto src/proto/serverdata.proto
```

## Run

```bash
mosquitto                                    # terminal 1
python3 scripts/vcu_mimic.py                 # terminal 2 (run from calypso-sim/)
```

Useful flags:

| Flag | Effect |
|---|---|
| `--with-auto` | Run autonomous heartbeat alongside stream — exercises ownership isolation (S4b). |
| `--only S1,S3` | Run only the named scenarios (default: all). |
| `--broker host:port` | Override broker address (default `localhost:1883`). |
| `--debug` | Verbose logging including raw stderr from calypso-sim. |

## Scenarios

| ID | What it covers |
|---|---|
| `S1` | Boot — claim every `VCU/*` topic, publish READY/OFF/home, verify on broker. |
| `S2` | Menu navigation — `nero_index` 0→7→0 (wrap at `MAX_NERO_STATES`). |
| `S3` | Enter PIT — `menu_select` rejected without brake+tsms, accepted once both set. |
| `S4` | Drive telemetry sweep — sine accelerator, speed capped at `PIT_MAX_SPEED=5 mph`. |
| `S4b` | (with `--with-auto`) Verify autonomous does not republish claimed topics. |
| `S5` | Enter REVERSE — only allowed from F_PIT, requires brake+tsms+menu cycle. |
| `S6` | Fault — trigger `BSPD_PREFAULT`, verify FAULTED+OFF+home, then recover. |

Each scenario prints `Sx ✓ ...` on success or raises `AssertionError` on failure.

## Adding a scenario

1. Add a function `def s_my_thing(vcu, obs): ...` in `vcu_mimic.py`.
2. Use `vcu.menu_increment()`, `vcu.set_pedals(...)`, etc. to drive state.
3. Use `obs.assert_published(topic, expected_value, within_ms=500)` to verify.
4. Register it in the `SCENARIOS` dict at the bottom.

The `VcuMock` mirrors the state machine in Cerberus-2.0's `Core/Src/u_statemachine.c` —
when adding behavior, cross-reference the C source to keep semantics aligned.
