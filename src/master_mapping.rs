use super::decode_data::*; 
use super::data::Data; 

pub struct MessageInfo {
    pub decoder: fn(data: &[u8]) -> Vec<Data>,
} 

impl MessageInfo {
    pub fn new(decoder: fn(data: &[u8]) -> Vec<Data>) -> Self {
        Self {
            decoder
        }
    }
}
pub fn get_message_info(id: &u32) -> MessageInfo { 
   match id {    0x80 => MessageInfo::new(decode_accumulator_status),
    0x81 => MessageInfo::new(decode_bms_status),
    0x82 => MessageInfo::new(decode_shutdown_control),
    0x83 => MessageInfo::new(decode_cell_data),
    0x84 => MessageInfo::new(decode_cell_temperatures),
    0x85 => MessageInfo::new(decode_segment_temperatures),
    0x500 => MessageInfo::new(decode_nerduino_acceleromter),
    0x501 => MessageInfo::new(decode_mpu_status),
    0x680 => MessageInfo::new(decode_wheel_state),
    _ => MessageInfo::new(decode_mock), 
    }
}