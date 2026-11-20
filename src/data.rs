use std::fmt;

/**
 * Wrapper Class for Data coming off the car
 */
pub struct DecodeData {
    pub value: Vec<f32>,
    pub topic: String,
    pub unit: String,
    pub clients: Option<Vec<u16>>,
}

/**
 * Implementation for the format of the data for debugging purposes
 */
impl fmt::Display for DecodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Overrides the string representation of the class.

        write!(
            f,
            "topic: {}, value: {:#?}, unit: {}, clients: {:#?}",
            self.topic, self.value, self.unit, self.clients
        )
    }
}

/**
 * Implementation fo the `DecodeData` methods
 */
impl DecodeData {
    /**
     * Constructor
     * @param id: the id of the data
     * @param value: the value of the data
     * @param topic: the topic of the data
     * @param clients: additional MQTT clients
     */
    #[must_use]
    pub fn new(value: Vec<f32>, topic: &str, unit: &str, clients: Option<Vec<u16>>) -> Self {
        Self {
            value,
            topic: topic.to_string(),
            unit: unit.to_string(),
            clients,
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
 * Implementation fo the `DecodeData` methods
 */
impl EncodeData {
    /**
     * Constructor
     * @param id: the id of the can message
     * @param value: the can message payload
     * @param `is_ext`: whether the can message is extended format ID
     */
    #[must_use]
    pub fn new(id: u32, value: Vec<u8>, is_ext: bool) -> Self {
        Self { value, id, is_ext }
    }
}

/**
 * Class to contain the data formatting functions
 * _d = a func to decode a value
 * _e = its counterpart to encode a value for sending on CAN line
 */
pub struct FormatData {}

impl FormatData {
    /* General divide function */
    #[must_use]
    pub fn divide_d(value: f32, divisor: f32) -> f32 {
        value / divisor
    }
    #[must_use]
    pub fn divide_e(value: f32, multiplicand: f32) -> f32 {
        value * multiplicand
    }

    /* Energy meter temperature is (degC = raw * 0.5) according to datasheet */
    #[must_use]
    pub fn temperature_d(value: f32, _divisor: f32) -> f32 {
        value * 0.5
    }
    #[must_use]
    pub fn temperature_e(value: f32, _multiplicand: f32) -> f32 {
        value * 2.0
    }

    /* Energy meter temperature indices are determined by multiplexor signal */
    #[must_use]
    pub fn multiply_d(value: f32, multiplicand: f32) -> f32 {
        value * multiplicand
    }
    #[must_use]
    pub fn multiply_e(value: f32, divisor: f32) -> f32 {
        value / divisor
    }
}
