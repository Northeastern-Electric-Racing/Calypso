extern crate paho_mqtt as mqtt;
use mqtt::ServerResponse;
use protobuf::Message;
use std::time::Duration;
use std::{process, thread};

use crate::client::Client;
use crate::data::Data;
use crate::serverdata;

pub const DFLT_BROKER: &str = "mqtt://localhost:1883";
const DFLT_CLIENT: &str = "calypso";

/**
 * MqttClient is a wrapper around the paho_mqtt::Client.
 */
pub struct MqttClient {
    client: Option<mqtt::Client>,
}

/**
 * Implement the Publish trait for MqttClient.
 */
impl Client for MqttClient {
    /**
     * Publishes the given data to the broker.
     * param data: The data object to format and send.
     */
    fn publish(&mut self, data: &Data) {
        let topic = data.topic.to_string();
        let mut payload = serverdata::ServerData::new();
        payload.unit = data.unit.to_string();
        payload.value = data.value
                            .iter()
                            .map(|x| x.to_string())
                            .collect();

        /* If the client is initialized, publish the data. */
        if let Some(client) = &self.client {
            let msg = mqtt::MessageBuilder::new()
                .topic(topic)
                .payload(payload.write_to_bytes().unwrap())
                .finalize();

            match { client.publish(msg) } {
                Ok(_) => (),
                Err(e) => println!("Error sending message: {:?}", e),
            }
            thread::sleep(Duration::from_millis(1));
        } else {
            println!("Client not initialized, please set host first and connect")
        }
    }

    /**
     * Connects to the broker.
     * Sets the host and then connects
     */
    fn connect(&mut self, host: &str) {
        self.set_host(&host.to_string());
        self.connect();
    }
}

impl Default for MqttClient {
    /**
     * Creates a new MqttClient.
     */
    fn default() -> Self {
        Self::new()
    }
}

/**
 * Implementation of the MqttClient struct.
 */
impl MqttClient {
    /**
     * Creates a new MqttClient.
     */
    pub fn new() -> MqttClient {
        MqttClient { client: None }
    }

    /**
     * Creates a new MqttClient with the given host name.
     * param host_name: The host name of the broker.
     */
    fn set_host(&mut self, host_name: &String) {
        let create_options = mqtt::CreateOptionsBuilder::new()
            .server_uri(host_name)
            .client_id(DFLT_CLIENT.to_string())
            .finalize();
        self.client = Some(match { mqtt::Client::new(create_options) } {
            Ok(client) => client,
            Err(e) => {
                println!("Error creating the client: {:?}", e);
                process::exit(1);
            }
        });
    }

    /**
     * Connects to the broker.
     * Sets the last will and testament.
     */
    fn connect(&mut self) {
        if let Some(client) = &self.client {
            let lastwilltestatment = mqtt::MessageBuilder::new()
                .topic("Calypso/Status")
                .payload("Calypso is offline")
                .finalize();
            let conn_opts = mqtt::ConnectOptionsBuilder::new()
                .keep_alive_interval(Duration::from_secs(20))
                .clean_session(false)
                .will_message(lastwilltestatment)
                .finalize();
            if let Err(e) = client.connect(conn_opts) {
                println!("Unable to connect:\n\t{:?}", e);
                process::exit(1);
            }
        } else {
            println!("Client not initialized, please set host first");
        }
    }

    /**
     * Check if the client is connected to the broker.
     */
    fn _is_connected(&self) -> bool {
        if let Some(client) = &self.client {
            client.is_connected()
        } else {
            println!("Client not initialized, please set host first");
            false
        }
    }

    /**
     * Reconnect to the broker.
     */
    fn _reconnect(&mut self) -> Result<ServerResponse, mqtt::Error> {
        if let Some(client) = &self.client {
            client.reconnect()
        } else {
            Err(mqtt::Error::General(
                "Client not initialized, please set host first",
            ))
        }
    }

    /**
     * Disconnect from the broker.
     */
    fn _disconnect(&mut self) -> Result<(), mqtt::Error> {
        if let Some(client) = &self.client {
            client.disconnect(None)
        } else {
            Err(mqtt::Error::General(
                "Client not initialized, please set host first",
            ))
        }
    }
}
