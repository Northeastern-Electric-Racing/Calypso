use std::collections::HashMap;

use crate::data::Data;

// Mapping from data IDs to the status bits they encode
// Each data ID contains a hash map with keys that are bit names and values that are the indexes
const STATUS_MAP: HashMap<u8, HashMap<&str, u8>> = [
    (6, HashMap::new()),  // Failsafe Statuses
    (7, HashMap::new()),  // DTC Status 1
    (8, HashMap::new()),  // DTC Status 2
    (9, HashMap::new()),  // Current Limits Status
    (12, HashMap::new()), // MPE State
    (
        // VSM State
        64,
        hashmap![
           "VSM Start State" => 0,
           "Pre-charge Init State" => 1,
           "Pre-charge Active State" => 2,
           "Pre-charge Complete State" => 3,
           "VSM Wait State" => 4,
           "VSM Ready State" => 5,
           "Motor Running State" => 6,
           "Blink Fault Code State" => 7
        ],
    ),
    (
        // Inverter State
        65,
        hashmap![
           "Power on State" => 0,
           "Stop State" => 1,
           "Open Loop State" => 2,
           "Closed Loop State" => 3,
           "Wait State" => 4,
           "Idle Run State" => 8,
           "Idle Stop State" => 9
        ],
    ),
    (
        // Relay State
        66,
        hashmap![
           "Relay 1 Status" => 0,
           "Relay 2 Status" => 1,
           "Relay 3 Status" => 2,
           "Relay 4 Status" => 3,
           "Relay 5 Status" => 4,
           "Relay 6 Status" => 5,
        ],
    ),
    (
        // Inverter Run Mode
        67,
        hashmap!["Inverter Run Mode" => 0],
    ),
    (
        // Inverter Command Mode
        69,
        hashmap!["Inverter Command Mode" => 0],
    ),
    (
        // Inverter Enable State
        70,
        hashmap!["Inverter Enable State" => 0],
    ),
    (
        // Inverter Enable Lockout
        71,
        hashmap!["Inverter Enable Lockout" => 0],
    ),
    (
        // Direction Command
        72,
        hashmap!["Direction Command" => 0],
    ),
    (
        // BMS Active
        73,
        hashmap!["BMS Active" => 0],
    ),
    (
        // BMS Limiting Torque
        74,
        hashmap!["BMS Limiting Torque" => 0],
    ),
    (
        // POST Fault Lo
        75,
        hashmap![
           "Hardware Gate/Desaturation Fault" => 0,
           "HW Over-current Fault" => 1,
           "Accelerator Shorted" => 2,
           "Accelerator Open" => 3,
           "Current Sensor Low" => 4,
           "Current Sensor High" => 5,
           "Module Temperature Low" => 6,
           "Module Temperature High" => 7,
           "Control PCB Temperature Low" => 8,
           "Control PCB Temperature High" => 9,
           "Gate Drive PCB Temperature Low" => 10,
           "Gate Drive PCB Temperature High" => 11,
           "5V Sense Voltage Low" => 12,
           "5V Sense Voltage High" => 13,
           "12V Sense Voltage Low" => 14,
           "12V Sense Voltage High" => 15
        ],
    ),
    (
        76,
        hashmap![ // POST Fault Hi
           "2.5V Sense Voltage Low" => 0,
        "2.5V Sense Voltage High" => 1,
        "1.5V Sense Voltage Low" => 2,
        "1.5V Sense Voltage High" => 3,
        "DC Bus Voltage High" => 4,
        "DC Bus Voltage Low" => 5,
        "Pre-charge Timeout" => 6,
        "Pre-charge Voltage Failure" => 7,
        "Brake Shorted" => 14,
        "Brake Open" => 15
        ],
    ),
    (
        77,
        hashmap![ // Run Fault Lo
           "Motor Over-speed Fault" => 0,
        "Over-current Fault" => 1,
        "Over-voltage Fault" => 2,
        "Inverter Over-temperature Fault" => 3,
        "Accelerator Input Shorted Fault" => 4,
        "Accelerator Input Open Fault" => 5,
        "Direction Command Fault" => 6,
        "Inverter Response Time-out Fault" => 7,
        "Hardware Gate/Desaturation Fault" => 8,
        "Hardware Over-current Fault" => 9,
        "Under-voltage Fault" => 10,
        "CAN Command Message Lost Fault" => 11,
        "Motor Over-temperature Fault" => 12
        ],
    ),
    (
        78,
        hashmap![ // Run Fault Hi
           "Brake Input Shorted Fault" => 0,
        "Brake Input Open Fault" => 1,
        "Module A Over-temperature Fault" => 2,
        "Module B Over temperature Fault" => 3,
        "Module C Over-temperature Fault" => 4,
        "PCB Over-temperature Fault" => 5,
        "Gate Drive Board 1 Over-temperature Fault" => 6,
        "Gate Drive Board 2 Over-temperature Fault" => 7,
        "Gate Drive Board 3 Over-temperature Fault" => 8,
        "Current Sensor Fault" => 9,
        "Hardware Over-Voltage Fault" => 11,
        "Resolver Not Connected" => 14,
        "Inverter Discharge Active" => 15
        ],
    ),
    (
        // Direction Command
        84,
        hashmap!["Direction Command" => 0],
    ),
    (
        // Inverter Enable
        85,
        hashmap!["Inverter Enable" => 0],
    ),
    (
        // Inverter Discharge
        86,
        hashmap!["Inverter Discharge" => 0],
    ),
    (
        // Speed Mode Enable
        87,
        hashmap!["Speed Mode Enable" => 0],
    ),
    (// Cell Voltage Info
),
];

fn get_status(data: &Data, name: &str) -> i32 {
    if !STATUS_MAP.contains_key(&data.id) {
        panic!("Data ID has no associated status mapping");
    }
    let bitmap = &STATUS_MAP[&data.id];

    if !bitmap.contains_key(name) {
        panic!("Status name could not be found in the given data point");
    }
    let index = bitmap[name];

    return (data.value >> index) & 1;
}

fn get_statuses(data: &Data) -> HashMap<&str, i32> {
    if !STATUS_MAP.contains_key(&data.id) {
        panic!("Data ID has no associated status mapping");
    }
    let bitmap = &STATUS_MAP[&data.id];

    // Convert each dict value to the bit value at the index
    return bitmap
        .iter()
        .map(|(name, index)| (*name, (data.value >> index) & 1))
        .collect();
}
