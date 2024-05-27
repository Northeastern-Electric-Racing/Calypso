use std::fmt;

/**
 * Wrapper Class for Data coming off the car
 */
pub struct Data {
    pub value: Vec<f32>,
    pub topic: String,
    pub unit: String,
}

/**
 * Implementation for the format of the data for debugging purposes
 */
impl fmt::Display for Data {
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
impl Data {
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

    pub fn to_json(&self) -> String {
        format!("{{\"value\": {:#?}, \"unit\": \"{}\"}}", self.value, self.unit)
    }
}