use chrono::prelude::*;

struct Data {
    timestamp: DateTime<Utc>,
    id: i32,
    value: i32,
}

struct Message {
    // Wrapper class for an individual message.
    timestamp: DateTime<Utc>,
    id: i32,
    data: Vec<i32>,
}

impl Message {
    fn new(timestamp: DateTime<Utc>, id: i32, data: Vec<i32>) -> Self {
        Message {
            timestamp,
            id,
            data,
        }
    }

    fn decode(&self) -> Result<Vec<Data>, MessageFormatException> {
        self.decode_message(&self.timestamp, self.id, &self.data)
    }

    fn decode_message(timestamp: &DateTime<Utc>, id: i32, data: &[i32]) -> Result<Vec<Data>, MessageFormatException> {
        let decoder = MESSAGE_IDS[id].decoder;
        let mut decoded_data: Vec<Data> = Vec::new();

        match decoder(data) {
            Ok(result) => {
                for (data_id, value) in result {
                    decoded_data.push(Data {
                        timestamp: *timestamp,
                        id: data_id,
                        value,
                    });
                }
                Ok(decoded_data)
            }
            Err(_) => Err(MessageFormatException { message: format!("Invalid data format for can id {}", id) }),
        }
    }
}

struct MessageFormatException {
    
    // A class to represent exceptions related to invalid message formats.
    
    message: String,
}