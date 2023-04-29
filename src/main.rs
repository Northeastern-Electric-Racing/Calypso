extern crate systemstat;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use socketcan::*;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
mod data;
mod decode_data;
mod master_mapping;
mod message;

fn main() {
    let mut down_command = Command::new("sudo")
        .arg("ifconfig")
        .arg("can0")
        .arg("down")
        .spawn()
        .expect("down command did not work");
    down_command
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
    bit_rate_commmand
        .wait()
        .expect("Fail while waiting for bit rate");
    let mut up_command = Command::new("sudo")
        .arg("ifconfig")
        .arg("can0")
        .arg("up")
        .spawn()
        .expect("up command did nto work");
    up_command
        .wait()
        .expect("Fail while waiting for up command");

    let mut stream = UnixStream::connect("/tmp/ipc.sock").unwrap();
    let (tx, rx) = channel();
    println!("uhhh");
    //open can socket channel at name can0
    const CAN_CHANNEL: &str = "can0";
    let socket = CANSocket::open(&CAN_CHANNEL);
    println!("CUCK");
    let socket = match socket {
        Ok(socket) => socket,
        Err(err) => {
            println!("Failed to open CAN socket: {}", err);
            return;
        }
    };
    println!("penis");
    thread::spawn(move || loop {
        println!("CUCKLORD");
        let msg = socket.read_frame().unwrap();
        let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let data = msg.data();
        let message = message::Message::new(&date, &msg.id(), &data);
        println!("CREATED MESSAGE");
        let decoded_data = message.decode();
        println!("CUCKY MAGOO");
        for (i, data) in decoded_data.iter().enumerate() {
            tx.send(format!("index:{},{}", data.id.to_string(), data.value.to_string())).unwrap();
            println!("SENDING: index:{},{}", data.id, data.value);
        }
    });
    loop {
        let _ = rx
            .try_recv()
            .map(|reply| stream.write_all(reply.as_bytes()));
    }
}
