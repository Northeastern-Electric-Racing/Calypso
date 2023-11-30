extern crate paho_mqtt as mqtt;
use mqtt::ServerResponse;
use std::time::Duration;
use std::{process, thread};

use crate::client::Client;
use crate::data::Data;

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
        let topic = format!("/Calypso");
        let payload = data.value.to_string();

        /* If the client is initialized, publish the data. */
        if let Some(client) = &self.client {
            let msg = mqtt::MessageBuilder::new()
                .topic(topic)
                .payload(payload)
                .finalize();
            client.publish(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
            return;
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
        self.client = Some(mqtt::Client::new(create_options).unwrap_or_else(|err| {
            println!("Error creating the client: {:?}", err);
            process::exit(1);
        }));
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
            Err(mqtt::Error::from(mqtt::Error::General(
                "Client not initialized, please set host first",
            )))
        }
    }

    /**
     * Disconnect from the broker.
     */
    fn _disconnect(&mut self) -> Result<(), mqtt::Error> {
        if let Some(client) = &self.client {
            client.disconnect(None)
        } else {
            Err(mqtt::Error::from(mqtt::Error::General(
                "Client not initialized, please set host first",
            )))
        }
    }
}
