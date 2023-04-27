extern crate systemstat;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use socketcan::*;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::sync::mpsc::channel;
use std::thread;
mod data;
mod decode_data;
mod master_mapping;
mod message;

fn main() {
    let mut stream = UnixStream::connect("/tmp/ipc.sock").unwrap();
    let (tx, rx) = channel();
    println!("uhhh");
    //open can socket channel at name can0
    let can_socket = CANSocket::open("can0").unwrap();
    //open("can0").unwrap();
    print!("penis");
    thread::spawn(move || {
        loop {
            let msg = can_socket.read_frame().unwrap();
            let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
            let data = msg.data();
            let message = message::Message::new(&date, &msg.id(), &data);
            let decoded_data = message.decode();
            for data in decoded_data {
                tx.send(format!("Data is {}", data));
            }
        }
    });
    loop {
        let _ = rx
            .try_recv()
            .map(|reply| stream.write_all(reply.as_bytes()));
    }

}
