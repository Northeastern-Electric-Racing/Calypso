use super::decode_data::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MessageInfo {
    pub description: String,
    pub decoder: fn(data: &[u8]) -> HashMap<u8, f32>,
}

impl MessageInfo {
    pub fn new(description: String, decoder: fn(data: &[u8]) -> HashMap<u8, f32>) -> Self {
        Self {
            description,
            decoder,
        }
    }
}

// Mapping from external message ID to decoding information
pub fn get_message_info(id: &u32) -> MessageInfo {
    match id {
        1 => return MessageInfo::new("accumulator status".to_string(), decode_accumulator_status),
        2 => return MessageInfo::new("BMS status".to_string(), decode_bms_status),
        3 => return MessageInfo::new("shutdown control".to_string(), decode3),
        4 => return MessageInfo::new("cell data".to_string(), decode_cell_voltages),
        160 => {
            return MessageInfo::new(
                "temperatures (igbt modules, gate driver board)".to_string(),
                decode5,
            )
        }
        161 => return MessageInfo::new("temperatures (control board)".to_string(), decode6),
        162 => return MessageInfo::new("temperatures (motor)".to_string(), decode7),
        163 => return MessageInfo::new("analog input voltages".to_string(), decode8),
        164 => return MessageInfo::new("digital input status".to_string(), decode9),
        165 => return MessageInfo::new("motor position information".to_string(), decode10),
        166 => return MessageInfo::new("Current information".to_string(), decode11),
        167 => return MessageInfo::new("Voltage Information".to_string(), decode12),
        168 => return MessageInfo::new("Flux Information".to_string(), decode13),
        169 => return MessageInfo::new("Internal Voltages".to_string(), decode14),
        170 => return MessageInfo::new("Internal States".to_string(), decode15),
        171 => return MessageInfo::new("Fault Codes".to_string(), decode16),
        172 => return MessageInfo::new("Torque and Timer Decoder".to_string(), decode17),
        192 => return MessageInfo::new("Command Data".to_string(), decode18),
        514 => return MessageInfo::new("Current Limits".to_string(), decode19),
        768 => {
            return MessageInfo::new(
                "NERduino Accelerometer".to_string(),
                decode_accelerometer_data,
            )
        }
        769 => return MessageInfo::new("NERduino Humidity".to_string(), decode21),
        7 => return MessageInfo::new("Cell Voltages".to_string(), decode_mock),
        193 => return MessageInfo::new("Unknown 1".to_string(), decode_mock),
        6 => return MessageInfo::new("Unknown 2".to_string(), decode_mock),
        194 => return MessageInfo::new("Unknown 3".to_string(), decode_mock),
        1744 => return MessageInfo::new("Unknown 4".to_string(), decode_mock),
        1745 => return MessageInfo::new("Unknown 5".to_string(), decode_mock),
        175 => return MessageInfo::new("Unknown 6".to_string(), decode_mock),
        770 => return MessageInfo::new("GLV Current".to_string(), decode29),
        2015 => return MessageInfo::new("Unknown 2015".to_string(), decode_mock),
        2027 => return MessageInfo::new("Unknown 2027".to_string(), decode_mock),
        2019 => return MessageInfo::new("Unknown 2019".to_string(), decode_mock),
        771 => return MessageInfo::new("Strain Gauge".to_string(), decode34),
        1024 => return MessageInfo::new("Wheel State".to_string(), decode35),
        10 => return MessageInfo::new("MPU States".to_string(), decode_mpu_dashboard_info),
        772 => return MessageInfo::new("GPS Data 1".to_string(), decode_gps_1),
        773 => return MessageInfo::new("GPS Data 2".to_string(), decode_gps_2),
        774 => return MessageInfo::new("GPS Data 3".to_string(), decode_gps_3),
        8 => return MessageInfo::new("Cell Temperatures".to_string(), decode_cell_temps),
        9 => return MessageInfo::new("Segment Temperatures".to_string(), decode_segment_temps),
        775 => return MessageInfo::new("Logging Status".to_string(), decode_logging_status),
        _ => return MessageInfo::new("Unknown".to_string(), decode_mock),
    }
}

#[derive(Clone)]
pub struct DataInfo {
    name: String,
    units: String,
}

impl DataInfo {
    pub fn new(name: String, units: String) -> Self {
        Self { name, units }
    }
}

pub fn get_data_info(id: u8) -> DataInfo {
    match id {
        0 => return DataInfo::new("Mock Data".to_string(), "".to_string()),
        1 => return DataInfo::new("Pack Inst Voltage".to_string(), "V".to_string()),
        2 => return DataInfo::new("Pack Current".to_string(), "A".to_string()),
        3 => return DataInfo::new("Pack Amphours".to_string(), "Ah".to_string()),
        4 => return DataInfo::new("Pack SOC".to_string(), "%".to_string()),
        5 => return DataInfo::new("Pack Health".to_string(), "%".to_string()),
        6 => return DataInfo::new("Failsafe Statuses".to_string(), "HEX".to_string()),
        7 => return DataInfo::new("DTC Status 1".to_string(), "HEX".to_string()),
        8 => return DataInfo::new("DTC Status 2".to_string(), "HEX".to_string()),
        9 => return DataInfo::new("Current Limits Status".to_string(), "".to_string()),
        10 => return DataInfo::new("Average Temp".to_string(), "C".to_string()),
        11 => return DataInfo::new("Internal Temp".to_string(), "C".to_string()),
        12 => return DataInfo::new("MPE State".to_string(), "BIN".to_string()),
        13 => return DataInfo::new("High Cell Voltage".to_string(), "V".to_string()),
        14 => return DataInfo::new("High Cell Voltage ID".to_string(), "".to_string()),
        15 => return DataInfo::new("Low Cell Voltage".to_string(), "V".to_string()),
        16 => return DataInfo::new("Low Cell Voltage ID".to_string(), "".to_string()),
        17 => return DataInfo::new("Average Cell Voltage".to_string(), "V".to_string()),
        18 => return DataInfo::new("Module A Temperature".to_string(), "C".to_string()),
        19 => return DataInfo::new("Module B Temperature".to_string(), "C".to_string()),
        20 => return DataInfo::new("Module C Temperature".to_string(), "C".to_string()),
        21 => return DataInfo::new("Gate Driver Board Temperature".to_string(), "C".to_string()),
        22 => return DataInfo::new("Control Board Temperature".to_string(), "C".to_string()),
        23 => return DataInfo::new("RTD #1 Temperature".to_string(), "C".to_string()),
        24 => return DataInfo::new("RTD #2 Temperature".to_string(), "C".to_string()),
        25 => return DataInfo::new("RTD #3 Temperature".to_string(), "C".to_string()),
        26 => return DataInfo::new("RTD #4 Temperature".to_string(), "C".to_string()),
        27 => return DataInfo::new("RTD #5 Temperature".to_string(), "C".to_string()),
        28 => return DataInfo::new("Motor Temperature".to_string(), "C".to_string()),
        29 => return DataInfo::new("Torque Shudder".to_string(), "N-m".to_string()),
        30 => return DataInfo::new("Analog Input 1".to_string(), "V".to_string()),
        31 => return DataInfo::new("Analog Input 2".to_string(), "V".to_string()),
        32 => return DataInfo::new("Analog Input 3".to_string(), "V".to_string()),
        33 => return DataInfo::new("Analog Input 4".to_string(), "V".to_string()),
        34 => return DataInfo::new("Analog Input 5".to_string(), "V".to_string()),
        35 => return DataInfo::new("Analog Input 6".to_string(), "V".to_string()),
        36 => return DataInfo::new("Digital Input 1".to_string(), "BIN".to_string()),
        37 => return DataInfo::new("Digital Input 2".to_string(), "BIN".to_string()),
        38 => return DataInfo::new("Digital Input 3".to_string(), "BIN".to_string()),
        39 => return DataInfo::new("Digital Input 4".to_string(), "BIN".to_string()),
        40 => return DataInfo::new("Digital Input 5".to_string(), "BIN".to_string()),
        41 => return DataInfo::new("Digital Input 6".to_string(), "BIN".to_string()),
        42 => return DataInfo::new("Digital Input 7".to_string(), "BIN".to_string()),
        43 => return DataInfo::new("Digital Input 8".to_string(), "BIN".to_string()),
        44 => return DataInfo::new("Motor Angle Electrical".to_string(), "Deg".to_string()),
        45 => return DataInfo::new("Motor Speed".to_string(), "RPM".to_string()),
        46 => return DataInfo::new("Electrical Output Frequency".to_string(), "Hz".to_string()),
        48 => return DataInfo::new("Phase A Current".to_string(), "A".to_string()),
        49 => return DataInfo::new("Phase B Current".to_string(), "A".to_string()),
        50 => return DataInfo::new("Phase C Current".to_string(), "A".to_string()),
        51 => return DataInfo::new("DC Bus Current".to_string(), "A".to_string()),
        52 => return DataInfo::new("DC Bus Voltage".to_string(), "V".to_string()),
        53 => return DataInfo::new("Output Voltage".to_string(), "V".to_string()),
        54 => return DataInfo::new("VAB_Vd Voltage".to_string(), "V".to_string()),
        55 => return DataInfo::new("VBC_Vq Voltage".to_string(), "V".to_string()),
        56 => return DataInfo::new("Flux Command".to_string(), "Wb".to_string()),
        57 => return DataInfo::new("Flux Feedback".to_string(), "wb".to_string()),
        58 => return DataInfo::new("Id Feedback".to_string(), "A".to_string()),
        59 => return DataInfo::new("Iq Feedback".to_string(), "A".to_string()),
        60 => return DataInfo::new("1.5V Reference Voltage".to_string(), "V".to_string()),
        61 => return DataInfo::new("2.5V Reference Voltage".to_string(), "V".to_string()),
        62 => return DataInfo::new("5.0V Reference Voltage".to_string(), "V".to_string()),
        63 => return DataInfo::new("12V System Voltage".to_string(), "V".to_string()),
        64 => return DataInfo::new("VSM State".to_string(), "".to_string()),
        65 => return DataInfo::new("Inverter State".to_string(), "".to_string()),
        66 => return DataInfo::new("Relay State".to_string(), "BIN".to_string()),
        67 => return DataInfo::new("Inverter Run Mode".to_string(), "BIN".to_string()),
        68 => {
            return DataInfo::new(
                "Inverter Active Discharge State".to_string(),
                "BIN".to_string(),
            )
        }
        69 => return DataInfo::new("Inverter Command Mode".to_string(), "BIN".to_string()),
        70 => return DataInfo::new("Inverter Enable State".to_string(), "BIN".to_string()),
        71 => return DataInfo::new("Inverter Enable Lockout".to_string(), "BIN".to_string()),
        72 => return DataInfo::new("Direction Command".to_string(), "BIN".to_string()),
        73 => return DataInfo::new("BMS Active".to_string(), "BIN".to_string()),
        74 => return DataInfo::new("BMS Limiting Torque".to_string(), "BIN".to_string()),
        75 => return DataInfo::new("POST Fault Lo".to_string(), "BIN".to_string()),
        76 => return DataInfo::new("POST Fault Hi".to_string(), "BIN".to_string()),
        77 => return DataInfo::new("Run Fault Lo".to_string(), "BIN".to_string()),
        78 => return DataInfo::new("Run Fault Hi".to_string(), "BIN".to_string()),
        79 => return DataInfo::new("Commanded Torque".to_string(), "N-m".to_string()),
        80 => return DataInfo::new("Torque Feedback".to_string(), "N-m".to_string()),
        81 => return DataInfo::new("Power on Timer".to_string(), "s".to_string()),
        82 => return DataInfo::new("Torque Command".to_string(), "N-m".to_string()),
        83 => return DataInfo::new("Speed Command".to_string(), "RPM".to_string()),
        84 => return DataInfo::new("Direction Command".to_string(), "BIN".to_string()),
        85 => return DataInfo::new("Inverter Enable".to_string(), "BIN".to_string()),
        86 => return DataInfo::new("Inverter Discharge".to_string(), "BIN".to_string()),
        87 => return DataInfo::new("Speed Mode Enable".to_string(), "BIN".to_string()),
        88 => return DataInfo::new("Commanded Torque Limit".to_string(), "N-m".to_string()),
        89 => return DataInfo::new("Pack DCL".to_string(), "A".to_string()),
        90 => return DataInfo::new("Pack CCL".to_string(), "A".to_string()),
        91 => return DataInfo::new("TCU X-Axis Acceleration".to_string(), "g".to_string()),
        92 => return DataInfo::new("TCU Y-Axis Acceleration".to_string(), "g".to_string()),
        93 => return DataInfo::new("TCU Z-Axis Acceleration".to_string(), "g".to_string()),
        94 => return DataInfo::new("TCU Temperature C".to_string(), "C".to_string()),
        95 => return DataInfo::new("TCU Temperature F".to_string(), "F".to_string()),
        96 => return DataInfo::new("Relative Humidity".to_string(), "%".to_string()),
        97 => return DataInfo::new("Cell Voltage Info".to_string(), "".to_string()),
        98 => return DataInfo::new("GLV Current".to_string(), "A".to_string()),
        99 => return DataInfo::new("Strain Gauge Voltage 1".to_string(), "V".to_string()),
        100 => return DataInfo::new("Strain Gauge Voltage 2".to_string(), "V".to_string()),
        101 => return DataInfo::new("Vehicle Speed".to_string(), "MPH".to_string()),
        102 => return DataInfo::new("Wheel Knob 1".to_string(), "".to_string()),
        103 => return DataInfo::new("Wheel Knob 2".to_string(), "".to_string()),
        104 => return DataInfo::new("Wheel Buttons".to_string(), "".to_string()),
        105 => return DataInfo::new("MPU Mode State".to_string(), "".to_string()),
        106 => return DataInfo::new("BMS State".to_string(), "".to_string()),
        107 => return DataInfo::new("BMS Faults".to_string(), "HEX".to_string()),
        108 => return DataInfo::new("Latitude".to_string(), "Deg".to_string()),
        109 => return DataInfo::new("Longitude".to_string(), "Deg".to_string()),
        110 => return DataInfo::new("GPS Fix Status".to_string(), "".to_string()),
        111 => return DataInfo::new("Altitude".to_string(), "m".to_string()),
        112 => return DataInfo::new("Ground Speed".to_string(), "m/s".to_string()),
        113 => return DataInfo::new("Heading Direction".to_string(), "Deg".to_string()),
        114 => return DataInfo::new("High Cell Temp".to_string(), "C".to_string()),
        115 => return DataInfo::new("High Cell Temp Chip Number".to_string(), "".to_string()),
        116 => return DataInfo::new("High Cell Temp Cell Number".to_string(), "".to_string()),
        117 => return DataInfo::new("Low Cell Temp".to_string(), "C".to_string()),
        118 => return DataInfo::new("Low Cell Temp Chip Number".to_string(), "".to_string()),
        119 => return DataInfo::new("Low Cell temp Cell Number".to_string(), "".to_string()),
        120 => return DataInfo::new("Average Cell Temp".to_string(), "C".to_string()),
        121 => return DataInfo::new("High Cell Voltage Chip Number".to_string(), "".to_string()),
        122 => return DataInfo::new("High Cell Voltage Cell Number".to_string(), "".to_string()),
        123 => return DataInfo::new("Low Cell Voltage Chip Number".to_string(), "".to_string()),
        124 => return DataInfo::new("Low Cell Voltage Cell Number".to_string(), "".to_string()),
        125 => return DataInfo::new("Segment 1 Average Temperature".to_string(), "C".to_string()),
        126 => return DataInfo::new("Segment 2 Average Temperature".to_string(), "C".to_string()),
        127 => return DataInfo::new("Segment 3 Average Temperature".to_string(), "C".to_string()),
        128 => return DataInfo::new("Segment 4 Average Temperature".to_string(), "C".to_string()),
        129 => return DataInfo::new("Logging Status".to_string(), "".to_string()),
        130 => return DataInfo::new("Accumulator Fan Percentage".to_string(), "%".to_string()),
        131 => return DataInfo::new("Motor Fan Percentage".to_string(), "%".to_string()),
        132 => return DataInfo::new("Torque Limit Percentage".to_string(), "%".to_string()),
        133 => return DataInfo::new("Regen Strength value".to_string(), "".to_string()),
        _ => return DataInfo::new("".to_string(), "".to_string())
    }
}