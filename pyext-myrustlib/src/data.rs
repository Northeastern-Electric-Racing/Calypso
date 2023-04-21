use std::fmt;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Data {
   
    // Wrapper class for an individual piece of data.
    

    timestamp: DateTime<Local>,
    id: i32,
    value: i32,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        // Overrides the string representation of the class.
        
        write!(f, "ID {} - {} - {}", self.id, self.timestamp, self.value)
    }
}

impl Data {
    fn new(timestamp: DateTime<Local>, id: i32, value: i32) -> Self {
        Self { timestamp, id, value }
    }
}

pub struct ProcessData {
    // Utility functions to process message data.
}

impl ProcessData {
    fn group_bytes(data_bytes: &[i32], group_length: usize) -> Vec<Vec<i32>> {
        // Splits the given data bytes into lists of specified length.
        data_bytes.chunks(group_length).map(|chunk| chunk.to_vec()).collect()
    }

    fn twos_comp(val: i32, bits: usize) -> i32 {
        // Computes the twos complement of the given value.
        if (val & (1 << (bits - 1))) != 0 {
            val - (1 << bits)
        } else {
            val
        }
    }

    fn little_endian(data_bytes: &[i32], bits: usize) -> i32 {
        
        // Transforms the given data bytes into a value in little endian.
        // Little Endian byte order stores low order bytes first.
        
        let mut result = 0;
        for (i, byte) in data_bytes.iter().enumerate() {
            result |= byte << (bits * i);
        }
        result
    }

    fn big_endian(data_bytes: &[i32], bits: usize) -> i32 {
        
        // Transforms the given data bytes into a value in big endian.
        // Big Endian byte order stores low order bytes last.
        
        let mut result = 0;
        for (i, byte) in data_bytes.iter().enumerate() {
            result |= byte << (bits * (data_bytes.len() - i - 1));
        }
        result
    }

    fn default_decode(byte_vals: &[i32]) -> Vec<i32> {
        
        // Default decode structure seen by a majority of the messages.
        
        let grouped_vals = ProcessData::group_bytes(byte_vals, 2);
        let parsed_vals: Vec<i32> = grouped_vals.iter().map(|val| ProcessData::little_endian(val, 8)).collect();
        let decoded_vals: Vec<i32> = parsed_vals.iter().map(|val| ProcessData::twos_comp(*val, 16)).collect();
        decoded_vals
    }
}

pub struct FormatData {
    
    // Utility functions to scale data values of a specific type.
    
}

impl FormatData {
    fn temperature(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn low_voltage(value: i32) -> f32 {
        value as f32 / 100.0
    }

    fn torque(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn high_voltage(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn current(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn angle(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn angular_velocity(value: i32) -> i32 {
        -value
    }

    fn frequency(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn power(value: i32) -> f32 {
        value as f32 / 10.0
    }

    fn timer(value: i32) -> f32 {
        value as f32 * 0.003
    }

    fn flux(value: i32) -> f32 {
        value as f32 / 1000.0
    }
}