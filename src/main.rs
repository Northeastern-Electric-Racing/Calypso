use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

use calypso::{
    command_data, data::EncodeData, decodable_message::DecodableMessage,
    encodable_message::EncodableMessage, encode_master_mapping::ENCODABLE_KEY_LIST,
    mqtt::MqttClient, serverdata,
};
use clap::Parser;
use protobuf::Message;
use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Id, Socket};

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
    //open can socket channel at name can_interface
    let mut client = MqttClient::new(pub_path, "calypso-decoder");
    if client.connect().is_err() {
        println!("Unable to connect to Siren, going into reconnection mode.");
        if client.reconnect().is_ok() {
            println!("Reconnected to Siren!");
        }
    }
    
    let socket = CanSocket::open(can_interface).expect("Failed to open CAN socket!");
    
    thread::spawn(move || loop {
        if !client.is_connected() {
            println!("[read_can] Unable to connect to Siren, going into reconnection mode.");
            if client.reconnect().is_ok() {
                println!("[read_can] Reconnected to Siren!");
            }
        }

        let msg = match socket.read_frame() {
            Ok(CanFrame::Data(msg)) => msg,
            Ok(CanFrame::Remote(_)) => {
                println!("Ignoring remote frame");
                continue;
            }
            Ok(CanFrame::Error(_)) => {
                println!("Ignoring error frame");
                continue;
            }
            Err(err) => {
                println!("Failed to read CAN frame: {}", err);
                continue;
            }
        };
        let data = msg.data();
        let message = DecodableMessage::new(
            match msg.id() {
                socketcan::Id::Standard(std) => std.as_raw().into(),
                socketcan::Id::Extended(ext) => ext.as_raw(),
            },
            data.to_vec(),
        );
        let decoded_data = message.decode();
        for data in decoded_data.iter() {
            let mut payload = serverdata::ServerData::new();
            payload.unit = data.unit.to_string();
            payload.value = data.value.iter().map(|x| x.to_string()).collect();

            if client.publish(
                    data.topic.to_string(),
                    protobuf::Message::write_to_bytes(&payload).unwrap_or_else(|e| {
                        format!("failed to serialize {}", e).as_bytes().to_vec()
                    }),
                ).is_err() {
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
    // client.connect().expect("Could not connect to Siren!");

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
        let packet = EncodableMessage::new(String::clone(&key.to_owned()), Vec::new());
        let ret = packet.encode();
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

                let packet = EncodableMessage::new(String::clone(&key), buf.data);
                let ret = packet.encode();
                send_map
                    .write()
                    .expect("Could not modify send messages!")
                    .insert(ret.id, ret);
            } else {
                // the code doesnt work without this else statement
                // idk why but never remove this else statement
                
                while !client.is_connected() {
                    println!("[read_siren] Unable to connect to Siren, going into reconnection mode.");
                    if client.reconnect().is_ok() {
                        println!("[read_siren] Reconnected to Siren!");
                    }
                }

                // let is_conn = client.is_connected();
                // println!("Client is {}", is_conn);
                // if !is_conn {
                //     println!("Trying to reconnect");
                //     match client.reconnect() {
                //         Ok(_) => println!("Reconnected!"),
                //         Err(_) => println!("Could not reconnect!"),
                //     }
                //     continue;
                // }
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
