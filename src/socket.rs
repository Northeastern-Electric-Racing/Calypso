use std::{io::Write, os::unix::net::UnixStream};

use crate::{client::Client, data::Data};

/**
 * IPCConnection is a wrapper around the IPC server.
 */
pub struct IPCConnection {
    stream: Option<UnixStream>,
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
        let stream: UnixStream = match UnixStream::connect(path) {
            Ok(stream) => stream,
            Err(_) => {
                println!("Failed to connect to IPC server, is NERO running?");
                return;
            }
        };
        self.stream = Some(stream);
    }
}

impl Default for IPCConnection {
    /**
     * Creates a new IPCConnection.
     */
    fn default() -> Self {
        Self::new()
    }
}

impl IPCConnection {
    /**
     * Creates a new IPCConnection.
     */
    pub fn new() -> IPCConnection {
        IPCConnection { stream: None }
    }
    /**
     * Sends the given data to the IPC server.
     * param data: The data object to format and send.
     */
    fn send(&mut self, data: &Data) {
        if let Some(stream) = &mut self.stream {
            let message = format!("{{index:{},value:{}}}", data.id, data.value);
            stream
                .write_all(message.as_bytes())
                .unwrap_or_else(|_| println!("Failed to send message, is NERO running?"));
        } else {
            println!("Sender not initialized, please connect first");
        }
    }
}
