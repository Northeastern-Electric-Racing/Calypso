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


### Simulation

The MQTT simulator lives in [`calypso-sim/`](calypso-sim/) as its own standalone crate. See [calypso-sim/README.md](calypso-sim/README.md) for usage.
