use chrono::prelude::*;
use std::fmt;

/**
 * Wrapper Class for Data coming off the car
 */
pub struct Data {
    pub(crate) timestamp: DateTime<Utc>,
    pub id: u8,
    pub value: f32,
}

/**
 * Implementation for the format of the data for debugging purposes
 */
impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Overrides the string representation of the class.

        write!(f, "ID {} - {} - {}", self.id, self.timestamp, self.value)
    }
}

/**
 * Implementation fo the Data Structs' methods
 */
impl Data {
    /**
     * Constructor
     * param timestamp: The time the data is collected
     * param id: the id of the data
     * param value: the value of the data
     */
    pub fn new(timestamp: DateTime<Utc>, id: u8, value: f32) -> Self {
        Self {
            timestamp,
            id,
            value,
        }
    }
}

/**
 * Class to contain the data processing functions
 */
pub struct ProcessData {}

impl ProcessData {
    /**
     * Groups the given data bytes into lists of specified length.
     */
    pub fn group_bytes(data_bytes: &[u8], group_length: usize) -> Vec<Vec<u8>> {
        data_bytes
            .chunks(group_length)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /**
     * Computes the twos complement of the given value.
     */
    pub fn twos_comp(val: u32, bits: usize) -> i64 {
        if (val & (1 << (bits - 1))) != 0 {
            (val as i64) - (1 << bits)
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
     * Decodes the given data bytes by grouping them into 2 byte chunks,
     * transforming them into little endian, and then computing the twos complement.
     */
    pub fn default_decode(byte_vals: &[u8]) -> Vec<i64> {
        let grouped_vals = ProcessData::group_bytes(byte_vals, 2);
        let parsed_vals: Vec<u32> = grouped_vals
            .iter()
            .map(|val| ProcessData::little_endian(val, 8))
            .collect();
        let decoded_vals: Vec<i64> = parsed_vals
            .iter()
            .map(|val| ProcessData::twos_comp(*val, 16))
            .collect();
        decoded_vals
    }
}

/**
 * Class to contain the data formatting functions
 */
pub struct FormatData {}

impl FormatData {
    pub fn temperature(value: i64) -> f32 {
        value as f32 / 10.0
    }

    pub fn low_voltage(value: i64) -> f32 {
        value as f32 / 100.0
    }

    pub fn torque(value: i64) -> f32 {
        value as f32 / 10.0
    }

    pub fn high_voltage(value: i64) -> f32 {
        value as f32 / 10.0
    }

    pub fn current(value: i64) -> f32 {
        value as f32 / 10.0
    }

    pub fn angle(value: i64) -> f32 {
        value as f32 / 10.0
    }

    pub fn angular_velocity(value: i64) -> i64 {
        -value
    }

    pub fn frequency(value: i64) -> f32 {
        value as f32 / 10.0
    }

    pub fn power(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn timer(value: i32) -> f32 {
        value as f32 * 0.003
    }

    pub fn flux(value: i64) -> f32 {
        value as f32 / 1000.0
    }
}
