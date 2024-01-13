# Calypso
Custom CAN Decoder for all the data being streamed around the car

### Recommended Extensions
View https://www.youtube.com/watch?v=BU1LYFkpJuk for more information

- rust-analyzer
- CodeLLDB
- Even Better TOML
- Error Lens
- Todo Tree
- crates

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

### NERO 1.0 Config
Utilizes a linux IPC to stream data to the NERO frontend

run ```/home/ner/Desktop/Calypso/target/release/calypso ipc /tmp/ipc.sock```

### SIREN and NERO 2.0 Config
Utilizes MQTT Web Socket to offload data from the car for our telemetry system
run ```/home/ner/Desktop/Calypso/target/release/calypso mqtt localhost:1883```

