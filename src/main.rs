use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::{Duration, UNIX_EPOCH},
};

use calypso::{
    command_data, data::DecodeData, data::EncodeData, decode_data::*, encode_data::*,
    mqtt::MqttClient, serverdata,
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
}

/**
 * Reads the can socket and publishes the data to the given client.
 */
fn read_can(pub_path: &str, can_interface: &str) -> JoinHandle<u32> {
    // Open CAN socket channel at name can_interface
    let mut client = MqttClient::new(pub_path, "calypso-decoder");
    if client.connect().is_err() {
        println!("Unable to connect to Siren, going into reconnection mode.");
        if client.reconnect().is_ok() {
            println!("Reconnected to Siren!");
        }
    }

    let socket = CanSocket::open(can_interface).expect("Failed to open CAN socket!");
    socket
        .set_error_filter_accept_all()
        .expect("Failed to set error mask on CAN socket!");

    thread::spawn(move || loop {
        if !client.is_connected() {
            println!("[read_can] Unable to connect to Siren, going into reconnection mode.");
            if client.reconnect().is_ok() {
                println!("[read_can] Reconnected to Siren!");
            }
        }
        // Read from MQTT socket
        let decoded_data = match socket.read_frame() {
            // CanDataFrame
            Ok(CanFrame::Data(data_frame)) => {
                let data = data_frame.data();
                let id: u32 = match data_frame.id() {
                    socketcan::Id::Standard(std) => std.as_raw().into(),
                    socketcan::Id::Extended(ext) => ext.as_raw(),
                };
                let decoder_func = match DECODE_FUNCTION_MAP.get(&id) {
                    Some(func) => *func,
                    None => decode_mock,
                };
                decoder_func(data)
            }
            // CanRemoteFrame
            Ok(CanFrame::Remote(remote_frame)) => {
                // Send frame ID for Remote
                vec![DecodeData::new(
                    vec![remote_frame.raw_id() as f32],
                    "Calypso/Events/RemoteFrame",
                    "id",
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

            if client
                .publish(
                    data.topic.to_string(),
                    protobuf::Message::write_to_bytes(&payload).unwrap_or_else(|e| {
                        format!("failed to serialize {}", e).as_bytes().to_vec()
                    }),
                )
                .is_err()
            {
                println!("[read_can] Failed to publish to Siren.");
            }

            // TODO: investigate disabling this
            thread::sleep(Duration::from_micros(100));
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
            None => encode_mock,
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
                let key = match msg.topic().split('/').last() {
                    Some(key) => key.to_owned(),
                    None => {
                        println!("Could not parse the key value in {}", msg.topic());
                        continue;
                    }
                };

                let encoder_func = match ENCODE_FUNCTION_MAP.get(&key) {
                    Some(func) => *func,
                    None => encode_mock,
                };
                let ret = encoder_func(buf.data);

                send_map
                    .write()
                    .expect("Could not modify send messages!")
                    .insert(ret.id, ret);
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
    let can_handle = read_can(&cli.siren_host_url, &cli.socketcan_iface);

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
