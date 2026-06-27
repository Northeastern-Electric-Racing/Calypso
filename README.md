# Calypso
Custom CAN decoder to translate CAN messages to MQTT protobuf encoded packets with low latency and a JSON configuration structure.

Foll developer guide here: https://nerdocs.atlassian.net/wiki/spaces/NER/pages/1815773187/Calypso+and+Odyssey+Definitions+Guide

Usage: run `-h` to see the full usage options and defaults.

### Develop setup

To test it on linux, please install:
- mosquitto broker: https://mosquitto.org
- can-utils
- mqttui: https://github.com/EdJoPaTo/mqttui


### Developing

Process for testing:  
- run `mosquitto` and leave it open
- run `mqttui` (from [here](https://nerdocs.atlassian.net/wiki/spaces/NER/pages/366280737/Read+Data+from+MQTTUI#Get-the-app-\(do-once\))) and leave it open
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
