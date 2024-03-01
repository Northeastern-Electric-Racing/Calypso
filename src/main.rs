use std::{
    env,
    process::{self, Command},
};

use calypso::{client::Client, message::Message, mqtt::MqttClient};
use socketcan::CANSocket;

fn configure_can(can_interface: &str) {
    let mut down_command = Command::new("sudo")
        .arg("ifconfig")
        .arg(can_interface)
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
        .arg(can_interface)
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
        .arg(can_interface)
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
fn read_can(mut publisher: Box<dyn Client + Send>, can_interface: &str) {
    //open can socket channel at name can_interface
    let socket = CANSocket::open(can_interface);
    let socket = match socket {
        Ok(socket) => socket,
        Err(err) => {
            println!("Failed to open CAN socket: {}", err);
            return;
        }
    };
    loop {
        let msg = match socket.read_frame() {
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
fn parse_args() -> (String, Box<dyn Client + 'static + Send>, String, bool) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 3 {
        println!("Not enough arguments!");
        println!("Client type and client path are required.");
        process::exit(1);
    }
    let client_type = &args[1];
    let path = &args[2];
    let can_interface = if args.len() > 3 { &args[3] } else { "can0" };

    let skip_can_config = args.len() > 4 && args[4] == "skip_can_configure";

    println!("Client type: {}", client_type);
    println!("Path: {}", path);
    println!("Can interface: {}", can_interface);
    println!("Skip can configuration: {}", skip_can_config);

    match client_type.as_str() {
        "mqtt" => (
            String::from(path),
            Box::new(MqttClient::new()) as Box<dyn Client + 'static + Send>,
            String::from(can_interface),
            skip_can_config,
        ),
        _ => {
            println!("Please provide a valid client type");
            process::exit(1);
        }
    }
}

/**
 * Main Function
 * Configures the can network, retrieves the client based on the command line arguments,
 * connects the client and then reads the can socket from specified interface.
 * Sample Calls for IPC "/home/ner/Desktop/Calypso/target/release/calypso ipc /tmp/ipc.sock &"
 * Sample Call for Mqtt "/home/ner/Desktop/Calypso/target/release/calypso mqtt localhost:1883 &"
 *
 * 3rd argument: if a can interface is passed the program will use it instead of the default can0
 * 4th argument: if skip_can_configure is passed the program will not call can interface setup commands
 * Ex: "/home/ner/Desktop/Calypso/target/release/calypso mqtt localhost:1883 can0 skip_can_configure &"
 */
fn main() {
    let (path, mut client, can_interface, skip_can_configure) = parse_args();
    if !skip_can_configure {
        configure_can(&can_interface)
    }
    client.connect(&path);
    read_can(client, &can_interface);
}
