use super::data::{Data,FormatData as fd, ProcessData as pd}; 

pub fn decode_mock(_data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(0.0, "Mock", ""));
    result
}
pub fn decode_accumulator_status(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(fd::high_voltage(pd::big_endian(&data[0..2] as &[u8], 8) as f32), "BMS/Pack/Voltage", "V"));
    result.push(Data::new(fd::current(pd::twos_comp(pd::big_endian(&data[2..4] as &[u8], 8) as u32, 16) as f32), "BMS/Pack/Current", "A"));
    result.push(Data::new(pd::big_endian(&data[4..6] as &[u8], 8) as f32, "BMS/Pack/Amp-hours", "Ah"));
    result.push(Data::new(data[6] as f32, "BMS/Pack/SOC", "%"));
    result.push(Data::new(data[7] as f32, "BMS/Pack/Health", "%"));
    result
}
pub fn decode_bms_status(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(data[0] as f32, "BMS/State", ""));
    result.push(Data::new(pd::little_endian(&data[1..5] as &[u8], 8) as f32, "BMS/Faults", ""));
    result.push(Data::new(pd::twos_comp(data[5] as u32, 8) as f32, "BMS/Temps/Average", "C"));
    result.push(Data::new(pd::twos_comp(data[6] as u32, 8) as f32, "BMS/Temps/Internal", "C"));
    result.push(Data::new(data[7] as f32, "BMS/Cells/BurningStatus", ""));
    result
}
pub fn decode_shutdown_control(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(data[0] as f32, "BMS/Shutdown/MPE", ""));
    result
}
pub fn decode_cell_data(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(fd::cell_voltage(pd::little_endian(&data[0..2] as &[u8], 8) as f32), "BMS/Cells/Volts/High/Value", "V"));
    result.push(Data::new(pd::half(data[2] as u8, 4) as f32, "BMS/Cells/Volts/High/Chip", ""));
    result.push(Data::new(pd::half(data[3] as u8, 0) as f32, "BMS/Cells/Volts/High/Cell", ""));
    result.push(Data::new(fd::cell_voltage(pd::little_endian(&data[4..6] as &[u8], 8) as f32), "BMS/Cells/Volts/Low/Value", "V"));
    result.push(Data::new(pd::half(data[6] as u8, 4) as f32, "BMS/Cells/Volts/Low/Chip", ""));
    result.push(Data::new(pd::half(data[7] as u8, 0) as f32, "BMS/Cells/Volts/Low/Cell", ""));
    result.push(Data::new(fd::cell_voltage(pd::little_endian(&data[8..10] as &[u8], 8) as f32), "BMS/Cells/Volts/Ave/Value", "V"));
    result
}
pub fn decode_cell_temperatures(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(pd::twos_comp(pd::little_endian(&data[0..2] as &[u8], 8) as u32, 16) as f32, "BMS/Cells/Temp/High/Value", "C"));
    result.push(Data::new(pd::half(data[2] as u8, 4) as f32, "BMS/Cells/Temp/High/Cell", ""));
    result.push(Data::new(pd::half(data[3] as u8, 0) as f32, "BMS/Cells/Temp/High/Chip", ""));
    result.push(Data::new(pd::twos_comp(pd::little_endian(&data[4..6] as &[u8], 8) as u32, 16) as f32, "BMS/Cells/Temp/Low/Value", "C"));
    result.push(Data::new(pd::half(data[6] as u8, 4) as f32, "BMS/Cells/Temp/Low/Cell", ""));
    result.push(Data::new(pd::half(data[7] as u8, 0) as f32, "BMS/Cells/Temp/Low/Chip", ""));
    result.push(Data::new(pd::twos_comp(pd::little_endian(&data[8..10] as &[u8], 8) as u32, 16) as f32, "BMS/Cells/Temp/Ave/Value", "C"));
    result
}
pub fn decode_segment_temperatures(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(pd::twos_comp(data[0] as u32, 8) as f32, "BMS/Segment/Temp/1", "C"));
    result.push(Data::new(pd::twos_comp(data[1] as u32, 8) as f32, "BMS/Segment/Temp/2", "C"));
    result.push(Data::new(pd::twos_comp(data[2] as u32, 8) as f32, "BMS/Segment/Temp/3", "C"));
    result.push(Data::new(pd::twos_comp(data[3] as u32, 8) as f32, "BMS/Segment/Temp/4", "C"));
    result
}
pub fn decode_nerduino_acceleromter(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(fd::acceleration(pd::big_endian(&data[0..2] as &[u8], 8) as f32), "MPU/Accel/X", "g"));
    result.push(Data::new(fd::acceleration(pd::big_endian(&data[2..4] as &[u8], 8) as f32), "MPU/Accel/Y", "g"));
    result.push(Data::new(fd::acceleration(pd::big_endian(&data[4..6] as &[u8], 8) as f32), "MPU/Accel/Z", "g"));
    result
}
pub fn decode_mpu_status(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(data[0] as f32, "MPU/State/Mode", ""));
    result.push(Data::new(data[1] as f32, "MPU/State/Torque_Limit_Percentage", ""));
    result.push(Data::new(data[2] as f32, "MPU/State/Regen_Strength", ""));
    result.push(Data::new(data[3] as f32, "MPU/State/Traction_Control", ""));
    result
}
pub fn decode_wheel_state(data: &[u8]) -> Vec::<Data> {
    let mut result = Vec::<Data>::new();
    result.push(Data::new(data[0] as f32, "WHEEL/Buttons/1", ""));
    result.push(Data::new(data[1] as f32, "WHEEL/Buttons/2", ""));
    result
}
