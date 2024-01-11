use std::{
    env,
    process::{self, Command},
};

use calypso::{client::Client, message::Message, mqtt::MqttClient};
use socketcan::CANSocket;

fn configure_can() {
    let mut down_command = Command::new("sudo")
        .arg("ifconfig")
        .arg("can0")
        .arg("down")
        .spawn()
        .expect("down command did not work");
    down_command // Takes down any current can networks
        .wait()
        .expect("Fail while waiting for down command");
    let mut bit_rate_commmand = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg("can0")
        .arg("type")
        .arg("can")
        .arg("bitrate")
        .arg("1000000")
        .spawn()
        .expect("bit rate command did not work");
    bit_rate_commmand //sets the bit rate of the can network
        .wait()
        .expect("Fail while waiting for bit rate");
    let mut up_command = Command::new("sudo")
        .arg("ifconfig")
        .arg("can0")
        .arg("up")
        .spawn()
        .expect("up command did nto work");
    up_command // Brings up the new can network
        .wait()
        .expect("Fail while waiting for up command");
}

/**
 * Reads the can socket and publishes the data to the given client.
 */
fn read_can(mut publisher: Box<dyn Client + Send>) {
    //open can socket channel at name can0
    const CAN_CHANNEL: &str = "can0";
    let socket = CANSocket::open(CAN_CHANNEL);
    let socket = match socket {
        Ok(socket) => socket,
        Err(err) => {
            println!("Failed to open CAN socket: {}", err);
            return;
        }
    };
    loop {
        let msg = match { socket.read_frame() } {
            Ok(msg) => msg,
            Err(err) => {
                println!("Failed to read CAN frame: {}", err);
                continue;
            }
        };
        let data = msg.data();
        let message = Message::new(msg.id(), data.to_vec());
        let decoded_data = message.decode();
        for data in decoded_data.iter() {
            publisher.publish(data)
        }
    }
}

/**
 * Parses the command line arguments.
 * Returns the client type and the path to connect to.
 */
fn parse_args() -> (String, Box<dyn Client + 'static + Send>) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("Please provide a client type");
        process::exit(1);
    }
    let client_type = &args[1];
    let path = &args[2];

    println!("Client type: {}", client_type);
    println!("Path: {}", path);

    match client_type.as_str() {
        "mqtt" => (
            String::from(path),
            Box::new(MqttClient::new()) as Box<dyn Client + 'static + Send>,
        ),
        _ => {
            println!("Please provide a valid client type");
            process::exit(1);
        }
    }
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments, connects the client and then reads the can socket.
 * Sample Calls for IPC "/home/ner/Desktop/Calypso/target/release/calypso ipc /tmp/ipc.sock &"
 * Sample Call for Mqtt "/home/ner/Desktop/Calypso/target/release/calypso mqtt localhost:1883 &"
 */
fn main() {
    configure_can();
    let (path, mut client) = parse_args();
    client.connect(&path);
    read_can(client);
}
