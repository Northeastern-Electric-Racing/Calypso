use std::{
    collections::HashMap,
    env, process,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

use calypso::{
    command_data, data::EncodeData, decodable_message::DecodableMessage,
    encodable_message::EncodableMessage, encode_master_mapping::ENCODABLE_KEY_LIST,
    mqtt::MqttClient,
};
use protobuf::Message;
use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Id, Socket};

/**
 * Reads the can socket and publishes the data to the given client.
 */
fn read_can(pub_path: &str, can_interface: &str) -> Result<JoinHandle<u32>, String> {
    //open can socket channel at name can_interface
    let mut client = MqttClient::new(pub_path, "calypso-decoder");
    client.connect();
    let socket = match CanSocket::open(can_interface) {
        Ok(socket) => socket,
        Err(err) => {
            return Err(format!("Failed to open CAN socket: {}", err));
        }
    };
    let join_handle: JoinHandle<_> = thread::spawn(move || loop {
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
            client.publish(data)
        }
    });
    return Ok(join_handle);
}

/**
 * Reads the mqtt incoming messages and sends can messages based off of that
 */
fn read_siren(
    pub_path: &str,
    send_map: Arc<RwLock<HashMap<u32, EncodeData>>>,
) -> Result<JoinHandle<()>, String> {
    let mut client = MqttClient::new(pub_path, "calypso-encoder");
    client.connect();
    let rx = client.start_consumer().expect("Could not begin consuming");
    client.subscribe("Calypso/Bidir/Command/#");

    // do the default initialization for all, do outside of the thread to save time negotiating when send_can comes up
    let mut writable_send_map = send_map.write().expect("Could not modify send messages!");
    for key in ENCODABLE_KEY_LIST {
        let packet = EncodableMessage::new(String::clone(&key.to_owned()), Vec::new());
        let ret = packet.encode();
        writable_send_map.insert(ret.id, ret);
    }
    drop(writable_send_map);

    let join_handle = thread::spawn(move || {
        for msg in rx.iter() {
            if let Some(msg) = msg {
                let mut buf = command_data::CommandData::new();
                match buf.merge_from_bytes(msg.payload()) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Could not decode command!");
                        continue;
                    }
                }
                let key = msg
                    .topic()
                    .split("/")
                    .last()
                    .unwrap_or("Reserved")
                    .to_owned();

                let packet = EncodableMessage::new(String::clone(&key), buf.data);
                let ret = packet.encode();
                println!("{ret}");
                send_map
                    .write()
                    .expect("Could not modify send messages!")
                    .insert(ret.id, ret);
            } else {
                // the code doesnt work without this else statement
                // idk why but never remove this else statement
                let is_conn = client.is_connected();
                println!("Client is {}", is_conn);
                if !is_conn {
                    println!("Trying to reconnect");
                    client.reconnect().expect("Could not reconnect");
                }
            }
        }
    });
    return Ok(join_handle);
}

fn send_out(
    can_interface: &str,
    send_map: Arc<RwLock<HashMap<u32, EncodeData>>>,
) -> Result<JoinHandle<()>, String> {
    let socket = match CanSocket::open(can_interface) {
        Ok(socket) => socket,
        Err(err) => {
            return Err(format!("Failed to open CAN socket: {}", err));
        }
    };

    let join_handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        let sender = send_map.read().expect("Cannot read map of sendables!");
        for msg in sender.iter() {
            // let id = u32::from_str_radix((msg.1.1).trim_start_matches("0x"), 16).expect("Invalid CAN ID!");

            let id: Id;
            if !msg.1.is_ext {
                id = socketcan::StandardId::new(msg.1.id.try_into().unwrap())
                    .unwrap()
                    .into();
            } else {
                id = socketcan::ExtendedId::new(msg.1.id).unwrap().into();
            }

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
    });
    return Ok(join_handle);
}

/**
 * Parses the command line a            return Err(-1);
rguments.
 */
fn parse_args() -> (String, String, bool) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
        println!("Not enough arguments!");
        println!("Siren (MQTT) URL and can interface are required");
        process::exit(1);
    }
    let path = &args[1];
    let can_interface = &args[2];
    let encode_en = args.len() >= 4 && args[3] == "encode_en";

    println!("Siren URL: {}", path);
    println!("Can interface: {}", can_interface);

    (String::from(path), String::from(can_interface), encode_en)
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments,
 * connects the client and then reads the can socket from specified interface.
 *
 * Args:
 * 1 -> IP address:port of Siren mqtt server (ex. localhost:1883)
 * 2 -> Socketcan interface name (ex. vcan0)
 * 3 -> encode_en or blank, whether to enable encoding
 */
fn main() {
    let (path, can_interface, encoding) = parse_args();
    let can_handle = match read_can(&path, &can_interface) {
        Ok(handle) => handle,
        Err(err) => {
            println!("Decoder Errored! {}", err);
            process::exit(1);
        }
    };

    // use a arc for mutlithread, and a rwlock to enforce one writer
    if encoding {
        let send_map: Arc<RwLock<HashMap<u32, EncodeData>>> = Arc::new(RwLock::new(HashMap::new()));

        let siren_handle = match read_siren(&path, Arc::clone(&send_map)) {
            Ok(handle) => handle,
            Err(err) => {
                println!("Encoder Errored! {}", err);
                process::exit(1);
            }
        };

        let send_handle = match send_out(&can_interface, Arc::clone(&send_map)) {
            Ok(handle) => handle,
            Err(err) => {
                println!("Sender Errored! {}", err);
                process::exit(1);
            }
        };

        siren_handle.join().expect("Encoder failed with ");
        println!("Encoder ended");

        send_handle.join().expect("Sender failed with ");
        println!("Sender ended");
    }

    can_handle.join().expect("Decoder failed with ");
    println!("Decoder ended");
}
