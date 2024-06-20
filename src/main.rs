use std::{env, process};

use calypso::{message::Message, mqtt::MqttClient};
use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Socket};

/**
 * Reads the can socket and publishes the data to the given client.
 */
fn read_can(mut publisher: MqttClient, can_interface: &str) {
    //open can socket channel at name can_interface
    let socket = CanSocket::open(can_interface);
    let socket = match socket {
        Ok(socket) => socket,
        Err(err) => {
            println!("Failed to open CAN socket: {}", err);
            return;
        }
    };
    loop {
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
        let message = Message::new(
            match msg.id() {
                socketcan::Id::Standard(std) => std.as_raw().into(),
                socketcan::Id::Extended(ext) => ext.as_raw(),
            },
            data.to_vec(),
        );
        let decoded_data = message.decode();
        for data in decoded_data.iter() {
            publisher.publish(data)
        }
    }
}

/**
 * Parses the command line arguments.
 */
fn parse_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
        println!("Not enough arguments!");
        println!("Siren (MQTT) URL and can interface are required");
        process::exit(1);
    }
    let path = &args[1];
    let can_interface = &args[2];

    println!("Siren URL: {}", path);
    println!("Can interface: {}", can_interface);

    (String::from(path), String::from(can_interface))
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments,
 * connects the client and then reads the can socket from specified interface.
 * Sample Call for Mqtt "/home/ner/Desktop/Calypso/target/release/calypso mqtt localhost:1883 &"
 *
 * 3rd argument: if a can interface is passed the program will use it instead of the default can0
 * Ex: "/home/ner/Desktop/Calypso/target/release/calypso mqtt localhost:1883 can0 &"
 */
fn main() {
    let (path, can_interface) = parse_args();
    let mut client = MqttClient::new();
    client.connect(&path);
    read_can(client, &can_interface);
}
