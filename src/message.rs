use chrono::{DateTime, Utc};

use super::data::Data;
use super::master_mapping::get_message_info;

/**
 * Wrapper class for an individual message.
 */
pub struct Message {
    timestamp: DateTime<Utc>,
    id: u32,
    data: Vec<u8>,
}

/**
 * Implementation of Message.
 */
impl Message {
    /**
     * Creates a new message with the given timestamp, id, and data.
     */
    pub fn new(timestamp: DateTime<Utc>, id: u32, data: Vec<u8>) -> Self {
        Self {
            timestamp,
            id,
            data,
        }
    }

    /**
     * Decodes the message and returns a vector of Data objects.
     */
    pub fn decode(&self) -> Vec<Data> {
        self.decode_message(&self.timestamp, &self.id, &self.data)
    }

    /**
     * Decodes the message and returns a vector of Data objects.
     * Achieves this by calling the decoder function associated with the message id.
     * param timestamp: The timestamp of the message.
     * param id: The id of the message.
     * param data: The data of the message.
     * return: A vector of Data objects.
     */
    fn decode_message(&self, timestamp: &DateTime<Utc>, id: &u32, data: &[u8]) -> Vec<Data> {
        let decoder = get_message_info(id).decoder;
        let mut decoded_data: Vec<Data> = Vec::new();
        let result = decoder(data);
        for (data_id, value) in result {
            decoded_data.push(Data::new(*timestamp, data_id, value));
        }
        decoded_data
    }
}
