use std::{
    os::unix::net::UnixStream,
    sync::mpsc::{channel, Sender},
};

use crate::{client::Client, data::Data};

/**
 * IPCConnection is a wrapper around the IPC server.
 */
pub struct IPCConnection {
    sender: Option<Sender<String>>,
}

/**
 * Implements the Client trait for IPCConnection.
 */
impl Client for IPCConnection {
    /**
     * Sends the data over the IPC connection.
     */
    fn publish(&mut self, data: &Data) {
        self.send(data);
    }

    /**
     * Attempts to connect to the IPC at the given path
     * param path: The path to connect to
     */
    fn connect(&mut self, path: &str) {
        let _stream: UnixStream = UnixStream::connect(path).unwrap();
        let (tx, _rx) = channel();
        self.sender = Some(tx);
    }
}

impl IPCConnection {
    /**
     * Creates a new IPCConnection.
     */
    pub fn new() -> IPCConnection {
        IPCConnection { sender: None }
    }
    /**
     * Sends the given data to the IPC server.
     * param data: The data object to format and send.
     */
    fn send(&mut self, data: &Data) {
        if let Some(sender) = &self.sender {
            let message = format!(
                "{{
                  index:{},
                  value:{}
               }}",
                data.id.to_string(),
                data.value.to_string()
            );
            sender
                .send(message)
                .unwrap_or(println!("Failed to send message, is NERO running?"));
        } else {
            println!("Sender not initialized, please connect first")
        }
    }
}
