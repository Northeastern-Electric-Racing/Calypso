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

/**
 * Class to contain the data processing functions
 */
pub struct ProcessData {}

impl ProcessData {
    /**
     * Computes the twos complement of the given value.
     */
    pub fn twos_comp(val: u32, bits: usize) -> i64 {
        if (val & (1 << (bits - 1))) != 0 {
            (val as i64) - ((1 << bits) as i64)
        } else {
            val as i64
        }
    }

    /**
     * Transforms the given data bytes into a value in little endian.
     * Little Endian byte order stores low order bytes first.
     */
    pub fn little_endian(data_bytes: &[u8], bits: usize) -> u32 {
        let mut result: u32 = 0;
        for (i, byte) in data_bytes.iter().enumerate() {
            result |= (*byte as u32) << (bits * i);
        }
        result
    }

    /**
     * Transforms the given data bytes into a value in big endian.
     * Big Endian byte order stores low order bytes last.
     */
    pub fn big_endian(bytes: &[u8], bits: usize) -> u32 {
        let mut result: u32 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            result |= (*byte as u32) << (bits * (bytes.len() - i - 1));
        }
        result
    }

    /**
     * Decodes the given byte by taking the top four bits after shifting it by the given number of bits.
     */
    pub fn half(byte: u8, bits: u8) -> u32 {
        (byte >> bits & 15) as u32
    }
}

/**
 * Class to contain the data formatting functions
 */
pub struct FormatData {}

impl FormatData {
    /* Temperatures are divided by 10 for 1 decimal point precision in C */
    pub fn temperature(value: f32) -> f32 {
        value / 10.0
    }

    /* Torque values are divided by 10 for one decimal point precision in N-m */
    pub fn torque(value: f32) -> f32 {
        value / 10.0
    }

    /* Current values are divided by 10 for one decimal point precision in A */
    pub fn current(value: f32) -> f32 {
        value / 10.0
    }

    /* Cell Voltages are recorded on a 10000x multiplier for V, must be divided by 10000 to get accurate number */
    pub fn cell_voltage(value: f32) -> f32 {
        value / 10000.0
    }

    /* Acceleration values must be offset by 0.0029 according to datasheet */
    pub fn acceleration(value: f32) -> f32 {
        value * 0.0029
    }

    /* High Voltage values are divided by 100 for one decimal point precision in V, high voltage is in regards to average voltage from the accumulator pack */
    pub fn high_voltage(value: f32) -> f32 {
        value / 100.0
    }
}
