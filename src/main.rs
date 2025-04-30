use std::{
    collections::HashMap,
    sync::{mpsc, Arc, RwLock},
    thread::{self, JoinHandle},
    time::{Duration, UNIX_EPOCH},
};

use calypso::{
    data::{DecodeData, EncodeData},
    decode_data::*,
    encode_data::*,
    mqtt::MqttClient,
    proto::{
        command_data,
        serverdata::{self, ServerData},
    },
};
use clap::Parser;
use protobuf::Message;
use socketcan::{CanError, CanFrame, CanSocket, EmbeddedFrame, Frame, Id, Socket, SocketOptions};

const ENCODER_MAP_SUB: &str = "Calypso/Bidir/Command/#";

/// Calypso command line arguments
#[derive(Parser, Debug)]
#[command(version)]
struct CalypsoArgs {
    /// Whether to enable CAN message encoding
    #[arg(short = 'e', long, env = "CALYPSO_CAN_ENCODE")]
    encode: bool,

    /// The host url of the siren, including port and excluding protocol prefix
    #[arg(
        short = 'u',
        long,
        env = "CALYPSO_SIREN_HOST_URL",
        default_value = "localhost:1883"
    )]
    siren_host_url: String,

    /// The SocketCAN interface port
    #[arg(
        short = 'c',
        long,
        env = "CALYPSO_SOCKETCAN_IFACE",
        default_value = "vcan0"
    )]
    socketcan_iface: String,

    // Whether to enable MQTT multi-client
    #[arg(short = 'm', long, env = "CALYPSO_MQTT_MULTICLIENT")]
    mqtt_multiclient: bool,
}

/**
 * Reads the can socket and publishes the data to the given client.
 */
fn read_can(pub_path: &str, can_interface: &str, mqtt_multiclient: bool) -> JoinHandle<u32> {
    let (siren_send, siren_recv) = mpsc::channel::<(String, ServerData)>();
    MqttClient::new(pub_path, "calypso-decoder").sending_loop(siren_recv, 1883);
    let mut clients: HashMap<u16, mpsc::Sender<(String, ServerData)>> =
        HashMap::from([(1883, siren_send)]);
    // Add 1882 client if multi-client is enabled
    if mqtt_multiclient {
        let (priority_send, priority_recv) = mpsc::channel::<(String, ServerData)>();
        MqttClient::new("localhost:1882", "calypso-priority").sending_loop(priority_recv, 1882);
        clients.insert(1882, priority_send);
    }

    let socket = CanSocket::open(can_interface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to set error mask on CAN socket!");

    thread::spawn(move || loop {
        let mut time = 0;
        // Read from CAN socket
        let decoded_data = match socket.read_frame() {
            // CanDataFrame
            Ok(CanFrame::Data(data_frame)) => {
                time = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                let data = data_frame.data();
                let id: u32 = match data_frame.id() {
                    socketcan::Id::Standard(std) => std.as_raw().into(),
                    socketcan::Id::Extended(ext) => ext.as_raw(),
                };
                match DECODE_FUNCTION_MAP.get(&id) {
                    Some(func) => func(data),
                    None => vec![DecodeData::new(
                        vec![id as f32],
                        "Calypso/Unknown",
                        "ID",
                        None,
                    )],
                }
            }
            // CanRemoteFrame
            Ok(CanFrame::Remote(remote_frame)) => {
                // Send frame ID for Remote
                vec![DecodeData::new(
                    vec![remote_frame.raw_id() as f32],
                    "Calypso/Events/RemoteFrame",
                    "id",
                    None,
                )]
            }
            // CanErrorFrame
            Ok(CanFrame::Error(error_frame)) => {
                // Publish enum index of error onto CAN
                // TODO: maybe look into better representation?
                let error_index: f32 = match CanError::from(error_frame) {
                    CanError::TransmitTimeout => 0.0,
                    CanError::LostArbitration(_) => 1.0,
                    CanError::ControllerProblem(_) => 2.0,
                    CanError::ProtocolViolation { .. } => 3.0,
                    CanError::TransceiverError => 4.0,
                    CanError::NoAck => 5.0,
                    CanError::BusOff => 6.0,
                    CanError::BusError => 7.0,
                    CanError::Restarted => 8.0,
                    CanError::DecodingFailure(_) => 9.0,
                    CanError::Unknown(_) => 10.0,
                };
                vec![DecodeData::new(
                    vec![error_index],
                    "Calypso/Events/ErrorFrame",
                    "CanError enum",
                    None,
                )]
            }
            // Socket failure
            Err(err) => {
                println!("CAN Socket failure: {}", err);
                continue;
            }
        };
        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;

        // Convert decoded CAN to Protobuf and publish over MQTT
        for data in decoded_data.iter() {
            let mut payload = serverdata::ServerData::new();
            payload.unit = data.unit.to_string();
            payload.values = data.value.clone();
            payload.time_us = timestamp;

            // TODO: Publish to other MQTT clients, if necessary.
            if let Some(additional_clients) = &data.clients {
                for port in additional_clients.iter() {
                    if let Some(client) = clients.get_mut(port) {
                        let current_time = UNIX_EPOCH.elapsed().unwrap().as_micros() as u64;
                        println!(
                            "PUBLISHING PRIORITY MESSAGE, TIME TAKEN: {}",
                            (current_time - time) / 1000
                        );
                        client.send((data.topic, payload));
                    }
                }
            }

            // Publish to Siren.
            if let Some(client) = clients.get_mut(&1883) {
                client.send((data.topic, payload));
            }
        }
    })
}

/**
 * Reads the mqtt incoming messages and sends can messages based off of that
 */
fn read_siren(pub_path: &str, send_map: Arc<RwLock<HashMap<u32, EncodeData>>>) -> JoinHandle<()> {
    let mut client = MqttClient::new(pub_path, "calypso-encoder");

    let _ = client.connect();
    while !client.is_connected() {
        println!("[read_siren] Unable to connect to Siren, going into reconnection mode.");
        if client.reconnect().is_ok() {
            println!("[read_siren] Reconnected to Siren!");
        }
    }

    let reciever = client.start_consumer().expect("Could not begin consuming");
    client
        .subscribe(ENCODER_MAP_SUB)
        .expect("Could not subscribe!");

    // do the default initialization for all, do outside of the thread to save time negotiating when send_can comes up
    let mut writable_send_map = send_map.write().expect("Could not modify send messages!");
    for key in ENCODABLE_KEY_LIST {
        let key_owned = key.to_owned();
        let encoder_func = match ENCODE_FUNCTION_MAP.get(&key_owned) {
            Some(func) => *func,
            None => panic!("An unknown message was initialized!"),
        };
        let ret = encoder_func(Vec::new());
        writable_send_map.insert(ret.id, ret);
    }
    drop(writable_send_map);

    thread::spawn(move || {
        for msg in reciever.iter() {
            if let Some(msg) = msg {
                let buf = match command_data::CommandData::parse_from_bytes(msg.payload()) {
                    Ok(buf) => buf,
                    Err(err) => {
                        println!("Could not decode command: {}", err);
                        continue;
                    }
                };
                let key = match msg.topic().split('/').next_back() {
                    Some(key) => key.to_owned(),
                    None => {
                        println!("Could not parse the key value in {}", msg.topic());
                        continue;
                    }
                };

                match ENCODE_FUNCTION_MAP.get(&key) {
                    Some(func) => {
                        let ret = func(buf.data);
                        send_map
                            .write()
                            .expect("Could not modify send messages!")
                            .insert(ret.id, ret);
                    }
                    None => {
                        let id: u32 = 0x7FF;
                        let mut send_map_writable =
                            send_map.write().expect("Could not modify send messages!");
                        let cnt = match send_map_writable.get(&id) {
                            Some(item) => item.value.first().unwrap_or(&0) + 1,
                            None => 1,
                        };
                        let ret = EncodeData {
                            value: vec![cnt],
                            id,
                            is_ext: false,
                        };
                        send_map_writable.insert(ret.id, ret);
                    }
                }
            } else {
                while !client.is_connected() {
                    println!(
                        "[read_siren] Unable to connect to Siren, going into reconnection mode."
                    );
                    if client.reconnect().is_ok() {
                        println!("[read_siren] Reconnected to Siren!");
                    }
                }
                client
                    .subscribe(ENCODER_MAP_SUB)
                    .expect("Could not subscribe!");
            }
        }
    })
}

fn send_out(
    can_interface: &str,
    send_map: Arc<RwLock<HashMap<u32, EncodeData>>>,
) -> JoinHandle<()> {
    let socket = CanSocket::open(can_interface).expect("Failed to open CAN socket!");

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(750));
        let sender = send_map.read().expect("Cannot read map of sendables!");
        for msg in sender.iter() {
            // let id = u32::from_str_radix((msg.1.1).trim_start_matches("0x"), 16).expect("Invalid CAN ID!");

            let id: Id = if !msg.1.is_ext {
                socketcan::StandardId::new(
                    msg.1
                        .id
                        .try_into()
                        .unwrap_or_else(|_| panic!("Invalid standard ID: {}", msg.1.id)),
                )
                .unwrap_or_else(|| panic!("Invalid standard ID: {}", msg.1.id))
                .into()
            } else {
                socketcan::ExtendedId::new(msg.1.id)
                    .unwrap_or_else(|| panic!("Invalid extended ID: {}", msg.1.id))
                    .into()
            };

            match CanFrame::new(id, &msg.1.value) {
                Some(packet) => {
                    match socket.write_frame(&packet) {
                        Ok(_) => (),
                        Err(err) => println!("Error sending can message {}", err),
                    };
                }
                None => {
                    println!("Packet is too long: {}", msg.1);
                }
            }
        }
    })
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments,
 * connects the client and then reads the can socket from specified interface.
 *
 */
fn main() {
    let cli = CalypsoArgs::parse();
    let can_handle = read_can(
        &cli.siren_host_url,
        &cli.socketcan_iface,
        cli.mqtt_multiclient,
    );

    // use a arc for mutlithread, and a rwlock to enforce one writer
    if cli.encode {
        let send_map: Arc<RwLock<HashMap<u32, EncodeData>>> = Arc::new(RwLock::new(HashMap::new()));

        let siren_handle = read_siren(&cli.siren_host_url, Arc::clone(&send_map));

        let send_handle = send_out(&cli.socketcan_iface, Arc::clone(&send_map));

        siren_handle.join().expect("Encoder failed with ");
        println!("Encoder ended");

        send_handle.join().expect("Sender failed with ");
        println!("Sender ended");
    }

    can_handle.join().expect("Decoder failed with ");
    println!("Decoder ended");
}
