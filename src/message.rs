use chrono::prelude::*;

use super::data::Data;
use super::master_mapping::get_message_info;

pub struct Message<'a> {
    // Wrapper class for an individual message.
    timestamp: DateTime<Utc>,
    id: u32,
    data: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn new(timestamp: &DateTime<Utc>, id: &u32, data: &'a [u8]) -> Self {
        Self {
            timestamp: *timestamp,
            id: *id,
            data,
        }
    }
    pub fn decode(&self) -> Vec<Data> {
        self.decode_message(&self.timestamp, &self.id, &self.data)
    }

    fn decode_message(&self, timestamp: &DateTime<Utc>, id: &u32, data: &[u8]) -> Vec<Data> {
        let decoder = get_message_info(id).decoder;
        println!("ATTEMPTING TO CUCK: {}", id);
        let mut decoded_data: Vec<Data> = Vec::new();
        let result = decoder(data);
        for (data_id, value) in result {
            decoded_data.push(Data::new(*timestamp, data_id, value));
        }
        return decoded_data;
    }
}
