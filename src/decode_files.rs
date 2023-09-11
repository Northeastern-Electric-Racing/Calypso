
// This file specifies methods to decode message fields (timestamp, id, data bytes) from 
// a line in a log file. 


use chrono::prelude::*;
use std::convert::TryInto;

use message::Message;

mod message;

enum LogFormat {
    Textual1,
    Textual1Legacy,
    Textual2,
    Binary,
}

fn process_line(line: &str, format: LogFormat) -> Message {
    
    // Processes a line of textual data according to a given format.
    
    match format {
        LogFormat::Textual1 => _process_textual1(line),
        LogFormat::Textual1Legacy => _process_textual1_legacy(line),
        LogFormat::Textual2 => _process_textual2(line),
        LogFormat::Binary => _process_binary(line),
    }
}

fn _process_textual1(line: &str) -> Message {
    
    // Processes a line of data in the format "Timestamp id length [data1,data2,...]"
    // Example line format: 1679511802367 514 8 [54,0,10,0,0,0,0,0]
    
    let fields: Vec<&str> = line.trim().split(' ').collect();
    let timestamp = NaiveDateTime::from_timestamp(fields[0].parse::<i64>().unwrap() / 1000, 0);
    let id = fields[1].parse::<u16>().unwrap();
    let length = fields[2].parse::<usize>().unwrap();
    let data: Vec<u8> = fields[3][1..fields[3].len() - 1]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    // remove commas and brackets at start and end
    let int_data: Vec<i16> = data
        .chunks_exact(2)
        .map(|x| i16::from_le_bytes(x.try_into().unwrap()))
        .collect();
    Message::new(timestamp, id, int_data)
}

fn _process_textual1_legacy(line: &str) -> Message {
    
    // Processes a line of data in the format:  Timestamp id length [data1,data2,...]
    // Example line format: 2021-01-01T00:00:00.003Z 514 8 [54,0,10,0,0,0,0,0]
    
    let fields: Vec<&str> = line.trim().split(' ').collect();
    let timestamp = NaiveDateTime::parse_from_str(fields[0], "%Y-%m-%dT%H:%M:%S.%fZ")
        .unwrap()
        .into();
    let id = fields[1].parse::<u16>().unwrap();
    let length = fields[2].parse::<usize>().unwrap();
    let data: Vec<u8> = fields[3][1..fields[3].len() - 1]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let int_data: Vec<i16> = data
        .chunks_exact(2)
        .map(|x| i16::from_le_bytes(x.try_into().unwrap()))
        .collect();
    Message::new(timestamp, id, int_data)
}

fn _process_textual2(line: &str) -> Message {
    
    // Processes a line of data in the format "Timestamp id length data1 data2 ..."
    // Example line format: 1659901910.121 514 8 54 0 10 0 0 0 0 0
    
    let fields: Vec<&str> = line.trim().split(' ').collect();
    let timestamp = NaiveDateTime::from_timestamp(fields[0].parse::<i64>().unwrap(), 0);
    let id = fields[1].parse::<u16>().unwrap();
    let length = fields[2].parse::<usize>().unwrap();
    let data: Vec<i16> = fields[3..3 + length]
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    Message::new(timestamp, id, data)
}

fn _process_binary(line: &str) -> Message {
    panic!("Binary files not currently supported.")
}