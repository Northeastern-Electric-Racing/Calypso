use chrono::prelude::*;
use std::fmt;

pub struct Data {
    // Wrapper class for an individual piece of data.
    pub(crate) timestamp: DateTime<Utc>,
    pub id: u8,
    pub value: f32,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Overrides the string representation of the class.

        write!(f, "ID {} - {} - {}", self.id, self.timestamp, self.value)
    }
}

impl Data {
    pub fn new(timestamp: DateTime<Utc>, id: u8, value: f32) -> Self {
        Self {
            timestamp,
            id,
            value,
        }
    }
}

pub struct ProcessData {
    // Utility functions to process message data.
}

impl ProcessData {
    pub fn group_bytes(data_bytes: &[u8], group_length: usize) -> Vec<Vec<u8>> {
        // Splits the given data bytes into lists of specified length.
        data_bytes
            .chunks(group_length)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    pub fn twos_comp(val: u32, bits: usize) -> i32 {
        // Computes the twos complement of the given value.
        if (val & (1 << (bits - 1))) != 0 {
            (val - (1 << bits)) as i32
        } else {
            val as i32
        }
    }

    pub fn little_endian(data_bytes: &[u8], bits: usize) -> u32 {
        // Transforms the given data bytes into a value in little endian.
        // Little Endian byte order stores low order bytes first.
        let mut result: u32 = 0;
        for (i, byte) in data_bytes.iter().enumerate() {
            // println!("Little End Byte: {}", byte);
            result |= (*byte as u32) << (bits * i);
            // println!("Little End Result: {}", result)
        }
        result
    }

    pub fn big_endian(bytes: &[u8], bits: usize) -> u32 {
        // Transforms the given data bytes into a value in big endian.
        // Big Endian byte order stores low order bytes last.
        let mut result: u32 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            // println!("Big End Byte: {}", byte);
            result |= (*byte as u32) << (bits * (bytes.len() - i - 1));
            // println!("Big End Result: {}", result);
        }
        result
    }

    pub fn default_decode(byte_vals: &[u8]) -> Vec<i32> {
        // Default decode structure seen by a majority of the messages.

        let grouped_vals = ProcessData::group_bytes(byte_vals, 2);
        println!("CUCKED GROUP BYTES");
        let parsed_vals: Vec<u32> = grouped_vals
            .iter()
            .map(|val| ProcessData::little_endian(val, 8))
            .collect();
        println!("CUCKED LITTLE ENDIAN");
        let decoded_vals: Vec<i32> = parsed_vals
            .iter()
            .map(|val| ProcessData::twos_comp(*val, 16))
            .collect();
        println!("CUCKED TWOS COMP");
        decoded_vals
    }
}

pub struct FormatData {
    // Utility functions to scale data values of a specific type.
}

impl FormatData {
    pub fn temperature(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn low_voltage(value: i32) -> f32 {
        value as f32 / 100.0
    }

    pub fn torque(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn high_voltage(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn current(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn angle(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn angular_velocity(value: i32) -> i8 {
        -(value as i8)
    }

    pub fn frequency(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn power(value: i32) -> f32 {
        value as f32 / 10.0
    }

    pub fn timer(value: i32) -> f32 {
        value as f32 * 0.003
    }

    pub fn flux(value: i32) -> f32 {
        value as f32 / 1000.0
    }
}
