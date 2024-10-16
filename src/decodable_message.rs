use super::data::DecodeData;

use super::decode_master_mapping::get_message_info;
/**
 * Wrapper class for an individual message.
 */
pub struct DecodableMessage {
    id: u32,
    data: Vec<u8>,
}

/**
 * Implementation of Message.
 */
impl DecodableMessage {
    /**
     * Creates a new message with the given timestamp, id, and data.
     */
    pub fn new(id: u32, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    /**
     * Decodes the message and returns a vector of Data objects.
     */
    pub fn decode(&self) -> Vec<DecodeData> {
        DecodableMessage::decode_message(&self.id, &self.data)
    }

    /**
     * Decodes the message and returns a vector of Data objects.
     * Achieves this by calling the decoder function associated with the message id.
     * param timestamp: The timestamp of the message.
     * param id: The id of the message.
     * param data: The data of the message.
     * return: A vector of Data objects.
     */
    fn decode_message(id: &u32, data: &[u8]) -> Vec<DecodeData> {
        let decoder = get_message_info(id).decoder;
        let mut decoded_data: Vec<DecodeData> = Vec::new();
        let result = decoder(data);
        for data in result {
            decoded_data.push(data);
        }
        decoded_data
    }
}
