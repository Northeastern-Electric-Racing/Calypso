use std::{collections::HashMap};

use chrono::prelude::*;

use super::data::Data;
use super::master_mapping::MESSAGE_IDS;

pub struct Message<'a> {
    // Wrapper class for an individual message.
    timestamp: DateTime<Utc>,
    id: u32,
    data: &'a [u8],
}

impl Message<'_> {
    pub fn new(timestamp: DateTime<Utc>, id: u32, data: &[u8]) -> Self {
        Message {
            timestamp,
            id,
            data,
        }
    }

    pub fn decode(&self) -> HashMap<u8, f32> {
        self.decode_message(&self.timestamp, &self.id, &self.data)
    }

    fn decode_message(
        &self,
        timestamp: &DateTime<Utc>,
        id: &u32,
        data: &[u8],
    ) -> HashMap<u8, f32> {
        let decoder = MESSAGE_IDS[id].decoder;
        let mut decoded_data: Vec<Data> = Vec::new();

        let result = decoder(data);
        for (data_id, value) in result {
            decoded_data.push(Data {
                timestamp: *timestamp,
                id: data_id,
                value,
            });
        }
        return result;
    }
}

struct MessageFormatException {
    // A class to represent exceptions related to invalid message formats.
    message: String,
}
