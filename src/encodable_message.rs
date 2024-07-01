use crate::data::EncodeData;

use super::encode_master_mapping::get_message_info;
/**
 * Wrapper class for an individual encodable message.
 */
pub struct EncodableMessage {
    key: String,
    data: Vec<f32>,
}

/**
 * Implementation of Message.
 */
impl EncodableMessage {
    /**
     * Creates a new message with the given string key and value
     */
    pub fn new(key: String, data: Vec<f32>) -> Self {
        Self { key, data }
    }

    /**
     * Decodes the message and returns a struct which defines a CAN packet
     */
    pub fn encode(self) -> EncodeData {
        EncodableMessage::encode_message(self.key, self.data)
    }

    /**
     * Decodes the message and returns a vector of Data objects.
     * Achieves this by calling the decoder function associated with the message key.
     * param key: The key of the message.
     * param data: The data of the message.
     * return: A vector of Data objects.
     */
    fn encode_message(key: String, data: Vec<f32>) -> EncodeData {
        let encoder = get_message_info(key).encoder;
        encoder(data)
    }
}
