use std::fmt;

/**
 * Wrapper Class for Data coming off the car
 */
pub struct DecodeData {
    pub value: Vec<f32>,
    pub topic: String,
    pub unit: String,
}

/**
 * Implementation for the format of the data for debugging purposes
 */
impl fmt::Display for DecodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Overrides the string representation of the class.

        write!(
            f,
            "topic: {}, value: {:#?}, unit: {}",
            self.topic, self.value, self.unit
        )
    }
}

/**
 * Implementation fo the Data Structs' methods
 */
impl DecodeData {
    /**
     * Constructor
     * @param id: the id of the data
     * @param value: the value of the data
     * @param topic: the topic of the data
     */
    pub fn new(value: Vec<f32>, topic: &str, unit: &str) -> Self {
        Self {
            value,
            topic: topic.to_string(),
            unit: unit.to_string(),
        }
    }
}

/**
 * Wrapper Class for data going into the car
 */
pub struct EncodeData {
    pub value: Vec<u8>,
    pub id: u32,
    pub is_ext: bool,
}

/**
 * Implementation for the format of the data for debugging purposes
 */
impl fmt::Display for EncodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Overrides the string representation of the class.

        write!(
            f,
            "{}#{:?} (extended: {})",
            self.id, self.value, self.is_ext
        )
    }
}

/**
 * Implementation fo the Data Structs' methods
 */
impl EncodeData {
    /**
     * Constructor
     * @param id: the id of the can message
     * @param value: the can message payload
     * @param topic: whether the can message is extended format
     */
    pub fn new(id: u32, value: Vec<u8>, is_ext: bool) -> Self {
        Self {
            id: id,
            value: value,
            is_ext: is_ext,
        }
    }
}
