extern crate paho_mqtt as mqtt;
use mqtt::ServerResponse;
use paho_mqtt::{Message, Receiver};
use std::time::Duration;

/**
 * MqttClient is a wrapper around the paho_mqtt::Client.
 */
pub struct MqttClient {
    client: mqtt::Client,
}

/**
 * Implementation of the MqttClient struct.
 */
impl MqttClient {
    /**
     * Creates a new MqttClient.
     */
    pub fn new(host_name: &str, client_id: &str) -> Self {
        let create_options = mqtt::CreateOptionsBuilder::new()
            .server_uri(host_name)
            .client_id(client_id)
            .finalize();
        MqttClient {
            client: mqtt::Client::new(create_options).expect("Could not instantiate MQTT client"),
        }
    }

    /**
     * Sets the hostname and connects to the server
     */
    pub fn connect(&mut self) -> Result<ServerResponse, mqtt::Error> {
        self._connect()
    }

    /**const DFLT_CLIENT: &str = "calypso";

    * Publishes a protobuf packet to the server
    */
    pub fn publish(&mut self, topic: String, payload: Vec<u8>) -> Result<(), mqtt::errors::Error> {
        let msg = mqtt::MessageBuilder::new()
            .topic(topic)
            .payload(payload)
            .finalize();
        self.client.publish(msg)
    }

    pub fn start_consumer(&mut self) -> Option<Receiver<Option<Message>>> {
        Some(self.client.start_consuming())
    }

    pub fn subscribe(&mut self, topic: &str) -> Result<ServerResponse, mqtt::Error> {
        self.client.subscribe(topic, 2)
    }
    /**
     * Connects to the broker.
     * Sets the last will and testament.
     */
    fn _connect(&mut self) -> Result<ServerResponse, mqtt::Error> {
        let lastwilltestatment = mqtt::MessageBuilder::new()
            .topic("Calypso/Status")
            .payload(format!("Calypso {} is offline", self.client.client_id()))
            .finalize();
        let conn_opts = mqtt::ConnectOptionsBuilder::new_v5()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(false)
            .will_message(lastwilltestatment)
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
            .finalize();
        self.client.connect(conn_opts)
    }

    /**
     * Check if the client is connected to the broker.
     */
    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    /**
     * Reconnect to the broker.
     */
    pub fn reconnect(&mut self) -> Result<ServerResponse, mqtt::Error> {
        self.client.reconnect()
    }

    /**
     * Disconnect from the broker.
     */
    fn _disconnect(&mut self) -> Result<(), mqtt::Error> {
        self.client.disconnect(None)
    }
}
