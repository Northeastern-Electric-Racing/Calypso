# Calypso
Custom CAN decoder to translate CAN messages to MQTT protobuf encoded packets with low latency and a YAML configuration structure.

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

To send a can message:
- `cansend vcan0 <ID_IN_HEX>#<PAYLOAD_IN_HEX>`
Ex. `cansend vcan0 702#01010101FFFFFFFF`  
Now view calypso interpret the can message and broadcast it on `mqttui`


### Simulation Mode
#### Run from build
- Same setup as above, then use the entry point `simulate` instead of `main`
- ```cargo run --bin simulate```
- ```cargo run --bin simulate  -- -u localhost:1883```

#### Run from Docker
- ```docker pull ghcr.io/northeastern-electric-racing/calypso:Develop```
- ```docker run -d --rm --network host ghcr.io/northeastern-electric-racing/calypso:Develop```
- ```docker run -d --rm -e CALYPSO_SIREN_HOST_URL=127.0.0.1:1883 --network host ghcr.io/northeastern-electric-racing/calypso:Develop```