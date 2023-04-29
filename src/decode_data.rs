// This file specifies methods to decode messages into the many pieces of data they contain.

use std::collections::HashMap;

use super::data::FormatData as fd;
use super::data::ProcessData as pd;

pub fn decode_mock(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(0, 0.0);
    result
}

pub fn decode_accumulator_status(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(1, pd::big_endian(&data[0..2], 8) as f32);
    result.insert(
        2,
        pd::twos_comp(pd::big_endian(&data[2..4], 8) as u32, 16) as f32 / 10.0,
    );
    result.insert(3, pd::big_endian(&data[4..6], 8) as f32);
    result.insert(4, data[6] as f32);
    result.insert(5, data[7] as f32);
    result
}

pub fn decode_bms_status(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(106, data[0] as f32);
    result.insert(107, pd::little_endian(&data[1..5], 8) as f32);
    result.insert(10, pd::twos_comp(data[5] as u32, 8) as f32);
    result.insert(11, pd::twos_comp(data[6] as u32, 8) as f32);
    result
}

pub fn decode3(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(12, data[0] as f32);
    result
}

pub fn decode_cell_voltages(data: &[u8]) -> HashMap<u8, f32> {
    let high_cell_volt_chip_number = (data[2] >> 4) & 15;
    let high_cell_volt_cell_number = (data[2] >> 0) & 15;
    let low_cell_volt_chip_number = (data[5] >> 4) & 15;
    let low_cell_volt_cell_number = (data[5] >> 0) & 15;
    let mut result = HashMap::new();
    result.insert(13, (pd::little_endian(&data[0..2], 8) as f32) / 10000.0);
    result.insert(121, high_cell_volt_chip_number as f32);
    result.insert(122, high_cell_volt_cell_number as f32);
    result.insert(15, (pd::little_endian(&data[3..5], 8) as f32) / 10000.0);
    result.insert(123, low_cell_volt_chip_number as f32);
    result.insert(124, low_cell_volt_cell_number as f32);
    result.insert(17, (pd::little_endian(&data[6..8], 8) as f32) / 10000.0);
    result
}

pub fn decode5(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(data);
    let final_data = decoded_data
        .iter()
        .map(|d| fd::temperature(*d))
        .collect::<Vec<f32>>();
    let mut result = HashMap::new();
    result.insert(18, final_data[0]);
    result.insert(19, final_data[1]);
    result.insert(20, final_data[2]);
    result.insert(21, final_data[3]);
    result
}

pub fn decode6(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(data);
    let final_data = decoded_data
        .iter()
        .map(|d| fd::temperature(*d))
        .collect::<Vec<f32>>();
    let mut result = HashMap::new();
    result.insert(22, final_data[0]);
    result.insert(23, final_data[1]);
    result.insert(24, final_data[2]);
    result.insert(25, final_data[3]);
    result
}

pub fn decode7(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(data);
    let final_data = decoded_data[..3]
        .iter()
        .map(|d| fd::temperature(*d))
        .collect::<Vec<f32>>();
    let mut result = HashMap::new();
    result.insert(26, final_data[0]);
    result.insert(27, final_data[1]);
    result.insert(28, final_data[2]);
    result.insert(29, fd::torque(decoded_data[3]));
    result
}

// TODO: Fill this method out (complicated with bit shifts)
pub fn decode8(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(30, 0.0);
    result.insert(31, 0.0);
    result.insert(32, 0.0);
    result.insert(33, 0.0);
    result.insert(34, 0.0);
    result.insert(35, 0.0);
    result
}

pub fn decode9(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(36, data[0] as f32);
    result.insert(37, data[1] as f32);
    result.insert(38, data[2] as f32);
    result.insert(39, data[3] as f32);
    result.insert(40, data[4] as f32);
    result.insert(41, data[5] as f32);
    result.insert(42, data[6] as f32);
    result.insert(43, data[7] as f32);
    result
}

pub fn decode10(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(data);
    let motor_speed: f32 = fd::angular_velocity(decoded_data[1]) as f32;
    let vehicle_speed = motor_speed * 0.013048225;
    let mut result = HashMap::new();
    result.insert(44, fd::angle(decoded_data[0]));
    result.insert(45, motor_speed);
    result.insert(46, fd::frequency(decoded_data[2]));
    result.insert(47, fd::angle(decoded_data[3]));
    result.insert(101, vehicle_speed);
    result
}

pub fn decode11(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(data);
    let final_data = decoded_data
        .iter()
        .map(|d| fd::current(*d))
        .collect::<Vec<f32>>();
    let mut result = HashMap::new();
    result.insert(48, final_data[0]);
    result.insert(49, final_data[1]);
    result.insert(50, final_data[2]);
    result.insert(51, final_data[3]);
    result
}

pub fn decode12(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(&data);
    let final_data: Vec<f32> = decoded_data.iter().map(|d| fd::high_voltage(*d)).collect();
    let mut result = HashMap::new();
    result.insert(52, final_data[0]);
    result.insert(53, final_data[1]);
    result.insert(54, final_data[2]);
    result.insert(55, final_data[3]);
    result
}

pub fn decode13(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(&data);
    let mut result = HashMap::new();
    result.insert(56, fd::flux(decoded_data[0]));
    result.insert(57, fd::flux(decoded_data[1]));
    result.insert(58, fd::current(decoded_data[2]));
    result.insert(59, fd::current(decoded_data[3]));
    result
}

pub fn decode14(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(&data);
    let final_data: Vec<f32> = decoded_data.iter().map(|d| fd::low_voltage(*d)).collect();
    let mut result = HashMap::new();
    result.insert(60, final_data[0]);
    result.insert(61, final_data[1]);
    result.insert(62, final_data[2]);
    result.insert(63, final_data[3]);
    result
}

pub fn decode15(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(64, pd::little_endian(&data[0..2], 8) as f32);
    result.insert(65, data[2] as f32);
    result.insert(66, data[3] as f32);
    result.insert(67, (data[4] & 1) as f32);
    result.insert(68, ((data[4] >> 5) & 7) as f32);
    result.insert(69, (data[5] & 1) as f32);
    result.insert(70, (data[6] & 1) as f32);
    result.insert(71, ((data[6] >> 7) & 1) as f32);
    result.insert(72, (data[7] & 1) as f32);
    result.insert(73, ((data[7] >> 1) & 1) as f32);
    result.insert(74, ((data[7] >> 2) & 1) as f32);
    result
}

pub fn decode16(data: &[u8]) -> HashMap<u8, f32> {
    let binding = pd::group_bytes(&data, 2);
    let data = binding.iter().map(|d| pd::little_endian(d, 8) as f32);
    let grouped_data = data.collect::<Vec<f32>>();
    let mut result = HashMap::new();
    result.insert(75, grouped_data[0]);
    result.insert(76, grouped_data[1]);
    result.insert(77, grouped_data[2]);
    result.insert(78, grouped_data[3]);
    result
}

pub fn decode17(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(&data[0..4]);
    let timer_data = pd::little_endian(&data[4..], 8) as i32;
    let mut result = HashMap::new();
    result.insert(79, fd::torque(decoded_data[0]));
    result.insert(80, fd::torque(decoded_data[1]));
    result.insert(81, fd::timer(timer_data));
    result
}

pub fn decode18(data: &[u8]) -> HashMap<u8, f32> {
    let decoded_data = pd::default_decode(&data);
    let mut result = HashMap::new();
    result.insert(82, fd::torque(decoded_data[0]));
    result.insert(83, fd::angular_velocity(decoded_data[1]) as f32);
    result.insert(84, data[4] as f32);
    result.insert(85, (data[5] & 1) as f32);
    result.insert(86, ((data[5] >> 1) & 1) as f32);
    result.insert(87, ((data[5] >> 2) & 1) as f32);
    result.insert(88, fd::torque(decoded_data[3]));
    result
}

pub fn decode19(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(89, pd::little_endian(&data[0..2], 8) as f32);
    result.insert(90, pd::little_endian(&data[2..4], 8) as f32);
    result
}

pub fn decode_accelerometer_data(data: &[u8]) -> HashMap<u8, f32> {
    // let decoded_data = pd::default_decode(&data);
    // let converted_data = decoded_data
    //     .iter()
    //     .map(|val| *val as f32 * 0.0029)
    //     .collect::<Vec<f32>>();
    // let mut matrix_data = vec![0.0; 3];
    // transpose::transpose(&converted_data[0..3], &mut matrix_data, 3, 1);
    // let transform_matrix = vec![
    //     1.0, 0.0, 0.0,
    //     0.0, f32::cos(f32::to_radians(70.0)), f32::sin(f32::to_radians(70.0)),
    //     0.0, -f32::sin(f32::to_radians(70.0)), f32::cos(f32::to_radians(70.0))
    // ];
    //TODO: matrix multiplication
    // let transformed_data = 
    // matrixmultiply::sgemm(3, 3, 0, alpha, a, rsa, csa, b, rsb, csb, beta, c, rsc, csc)transform_matrix * matrix_data;
    let mut result = HashMap::new();
    result.insert(91, 0.0);
    result.insert(92, 0.0);
    result.insert(93, 0.0);
    result
}

pub fn decode21(data: &[u8]) -> HashMap<u8, f32> {
    let temp = pd::little_endian(&data[0..2], 8) as f32;
    let humid = pd::little_endian(&data[2..4], 8) as f32;
    let temp_f = -49.0 + (315.0 * temp / 65535.0);
    let temp_c = -45.0 + (175.0 * temp / 65535.0);
    let rel_humid = 100.0 * humid / 65535.0;
    let mut result = HashMap::new();
    result.insert(94, temp_c);
    result.insert(95, temp_f);
    result.insert(96, rel_humid);
    result
}

// fn decode22(data: &[u8]) -> HashMap<u8, f32> {
//     let cell_id = data[0] as u32;
//     let instant_voltage = pd::big_endian(&data[1..3], 8);
//     let internal_resistance = (pd::big_endian(&data[3..5], 8) & 32767) as u32;
//     let shunted = ((data[3] >> 7) & 1) as u32;
//     let open_voltage = pd::big_endian(&data[5..7], 8);
//     let mut result = HashMap::new();
//     result.insert(
//         97,
//         "Cell ID: ".to_string()
//             + &cell_id.to_string()
//             + ", Instant Voltage: "
//             + &instant_voltage.to_string()
//             + ", Internal Resistance: "
//             + &internal_resistance.to_string()
//             + ", Shunted: "
//             + &shunted.to_string()
//             + ", Open Voltage: "
//             + &open_voltage.to_string(),
//     );

//     result
// }

pub fn decode29(data: &[u8]) -> HashMap<u8, f32> {
    let glv_current = pd::twos_comp(pd::little_endian(data, 8), 32) as f32;
    let mut result = HashMap::new();
    result.insert(98, glv_current / 1000000.0);
    result
}

pub fn decode34(data: &[u8]) -> HashMap<u8, f32> {
    let voltage1 = pd::twos_comp(pd::little_endian(&data[0..4], 8), 32) as f32;
    let voltage2 = pd::twos_comp(pd::little_endian(&data[4..], 8), 32) as f32;
    let mut result = HashMap::new();
    result.insert(99, voltage1 / 1000000.0);
    result.insert(100, voltage2 / 1000000.0);
    result
}

pub fn decode35(data: &[u8]) -> HashMap<u8, f32> {
    let mut result: HashMap<u8, f32> = HashMap::new();
    result.insert(102, pd::big_endian(&data[0..2], 8) as f32);
    result.insert(103, pd::big_endian(&data[2..4], 8) as f32);
    result.insert(104, data[4] as f32);
    result
}

pub fn decode_mpu_dashboard_info(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(105, data[0] as f32);
    result.insert(130, data[1] as f32);
    result.insert(131, data[2] as f32);
    result.insert(132, data[3] as f32);
    result.insert(133, data[4] as f32);
    result
}

pub fn decode_gps_1(data: &[u8]) -> HashMap<u8, f32> {
    let longitude = pd::twos_comp(pd::little_endian(&data[0..4], 8), 32) as f32 / 10000000.0;
    let latitude = pd::twos_comp(pd::little_endian(&data[4..8], 8), 32) as f32 / 10000000.0;
    let mut result = HashMap::new();
    result.insert(108, longitude);
    result.insert(109, latitude);
    result
}

pub fn decode_gps_2(data: &[u8]) -> HashMap<u8, f32> {
    let altitude = pd::twos_comp(pd::little_endian(&data[4..8], 8), 32) as f32 / 1000.0;
    let mut result: HashMap<u8, f32> = HashMap::new();
    result.insert(
        110,
        pd::twos_comp(pd::little_endian(&data[0..4], 8), 32) as f32,
    );
    result.insert(111, altitude);
    result
}

pub fn decode_gps_3(data: &[u8]) -> HashMap<u8, f32> {
    let ground_speed = pd::twos_comp(pd::little_endian(&data[0..4], 8), 32) as f32 / 1000.0;
    let heading = pd::twos_comp(pd::little_endian(&data[4..8], 8), 32) as f32 / 100000.0;
    let mut result = HashMap::new();
    result.insert(112, ground_speed);
    result.insert(113, heading);
    result
}

pub fn decode_cell_temps(data: &[u8]) -> HashMap<u8, f32> {
    let high_cell_temp_chip_number = (data[2] >> 4) & 15;
    let high_cell_temp_cell_number = (data[2] >> 0) & 15;
    let low_cell_temp_chip_number = (data[5] >> 4) & 15;
    let low_cell_temp_cell_number = (data[5] >> 0) & 15;

    let mut result = HashMap::new();
    result.insert(114, pd::twos_comp(pd::little_endian(&data[0..2], 8), 16) as f32);
    result.insert(115, high_cell_temp_chip_number as f32);
    result.insert(116, high_cell_temp_cell_number as f32);
    result.insert(117, pd::twos_comp(pd::little_endian(&data[3..5], 8), 16) as f32);
    result.insert(118, low_cell_temp_chip_number as f32);
    result.insert(119, low_cell_temp_cell_number as f32);
    result.insert(120, pd::twos_comp(pd::little_endian(&data[6..8], 8), 16) as f32);

    result
}

pub fn decode_segment_temps(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(125, pd::twos_comp(data[0] as u32, 8) as f32);
    result.insert(126, pd::twos_comp(data[1] as u32, 8) as f32);
    result.insert(127, pd::twos_comp(data[2] as u32, 8) as f32);
    result.insert(128, pd::twos_comp(data[3] as u32, 8) as f32);

    result
}

pub fn decode_logging_status(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(129, data[0] as f32);

    result
}

pub fn decode_lv_battery_1(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(134, (pd::little_endian(&data[0..2], 8)) as f32);
    result.insert(135, data[2] as f32);
    result.insert(136, (pd::little_endian(&data[3..5], 8)) as f32);
    result.insert(137, data[5] as f32);
    result.insert(138, (pd::little_endian(&data[6..8], 8)) as f32);
    result
}

pub fn decode_lv_battery_2(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(139, (pd::twos_comp(pd::little_endian(&data[0..2], 8), 16) as f32) * (192.264 / 1000000.0) * 4.0);
    result.insert(140, (pd::twos_comp(pd::little_endian(&data[2..4], 8), 16) as f32) * (1.648 / 1000.0));
    result.insert(141, (pd::twos_comp(pd::little_endian(&data[4..6], 8), 16) as f32) * (1.648 / 1000.0));
    result.insert(142, (pd::twos_comp(pd::little_endian(&data[6..8], 8), 16) as f32) * (1.46487 / 1000000.0) / (0.5 / 1000.0));
    result
}