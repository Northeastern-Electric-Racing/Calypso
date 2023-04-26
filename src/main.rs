
extern crate systemstat;
use std::sync::mpsc::channel;
use std::thread;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use socketcan::*;
use std::os::unix::net::UnixStream;
mod master_mapping;
mod decode_data;
mod data;
mod message;

use master_mapping::MESSAGE_IDS;

fn main() {
    let _stream = UnixStream::connect("/tmp/ipc.sock").unwrap();

    let (tx, _rx) = channel();

    thread::spawn(move || {
        loop {
            let can_socket = CANSocket::open("can0").unwrap();
            let msg = can_socket.read_frame().unwrap();
            if MESSAGE_IDS.contains_key(&msg.id()) {
                let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
                let data = msg.data();
                let message = message::Message::new(date, msg.id(), data);
                let decoded_data = message.decode();
                for (data_id, value) in decoded_data {
                    tx.send(format!("Value is: {}, ID is: {}", value, data_id));
                }
            }
        }
    });
}