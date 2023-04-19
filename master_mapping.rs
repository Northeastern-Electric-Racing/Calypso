mod decode_data;

use decode_data::*;
use std::collections::HashMap;

#[derive(Debug)]
struct MessageInfo {
    description: &'static str,
    decoder: &'static fn(data: &[u8]) -> HashMap<i32, f32>
}

// Mapping from external message ID to decoding information
const MESSAGE_IDS: HashMap<u8, MessageInfo> = [
    (
        1,
        MessageInfo {
            description: "accumulator status",
            decoder: decode_accumulator_status,
        },
    ),
    (
        2,
        MessageInfo {
            description: "BMS status",
            decoder: decode_bms_status,
        },
    ),
    (
        3,
        MessageInfo {
            description: "shutdown control",
            decoder: decode3,
        },
    ),
    (
        4,
        MessageInfo {
            description: "cell data",
            decoder: decode_cell_voltages,
        },
    ),
    (
        160,
        MessageInfo {
            description: "temperatures (igbt modules, gate driver board)",
            decoder: decode5,
        },
    ),
    (
        161,
        MessageInfo {
            description: "temperatures (control board)",
            decoder: decode6,
        },
    ),
    (
        162,
        MessageInfo {
            description: "temperatures (motor)",
            decoder: decode7,
        },
    ),
    (
        163,
        MessageInfo {
            description: "analog input voltages",
            decoder: decode8,
        },
    ),
    (
        164,
        MessageInfo {
            description: "digital input status",
            decoder: decode9,
        },
    ),
    (
        165,
        MessageInfo {
            description: "motor position information",
            decoder: decode10,
        },
    ),
    (
      166,
      MessageInfo {
        description: "Current information",
        decoder: decode11,
      },
    ),
    (
      167,
      MessageInfo {
        description: "Voltage Inforamtion",
        decoder: decode12,
      },
    ),
    (
    168,
    MessageInfo {
      description: "Flux Information",
      decoder: decode13,
    },
   ),
   (
      169,
      MessageInfo {
        description: "Internal Voltages",
        decoder: decode14,
      },
   ),
   (
      170,
      MessageInfo {
        description: "Internal States",
        decoder: decode15,
      },
   ),
   (
      171,
      MessageInfo {
        description: "Fault Codes",
        decoder: decode16,
      },
   ),
   (
      172,
      MessageInfo {
        description: "Torque and Timer Decoder",
        decoder: decode17,
      },
   ),
   (
      192,
      MessageInfo {
        description: "Commanded Data",
        decoder: decode18,
      },
   ),
   (
      514,
      MessageInfo {
        description: "Current Limits",
        decoder: decode19,
      },
   ),
   (
      768,
      MessageInfo {
        description: "NERduino Accelerometer",
        decoder: decode_accelerometer_data,
      },
   ),
   (
      769,
      MessageInfo {
        description: "NERduino Humidity",
        decoder: decode21,
      },
   ),
   (
      7,
      MessageInfo {
        description: "Ce;; Voltages",
        decoder: decode22,
      },
   ),
   (
      193,
      MessageInfo {
        description: "Unknown 1",
        decoder: decode_mock,
      },
   ),
   (
      6,
      MessageInfo {
         description: "Unknown 2",
         decoder: decode_mock
      },
   ),
   (
      194,
      MessageInfo{
         description: "Unknown 3",
         decoder: decode_mock
      },
   ),
   (
      1744,
       MessageInfo{
         description: "Unknown 4",
         decoder: decode_mock
      },
   ),
   (
      1745,
       MessageInfo{
         description: "Unknown 5",
         decoder: decode_mock
      },
   ),
   (
      175,
      MessageInfo{
         description: "Unknown 6",
         decoder: decode_mock
      },
   ),
   (
      770,
      MessageInfo{
         description: "GLV Current",
         decoder: decode29
      },
   ),
   (
      2015,
       MessageInfo{
         description: "Unknown 2015",
         decoder: decode_mock
      },
   ),
   (
      2027, MessageInfo{
         description: "Unknown 2027",
         decoder: decode_mock
      },
   ),
   (
      2019,
      MessageInfo{
         description: "Unknown 2019",
         decoder: decode_mock
      },
   ),
   (
      771,
      MessageInfo{
         description: "Strain Gauge",
         decoder: decode34
      },
   ),
   (
      1024,
      MessageInfo{
         description: "Wheel State",
         decoder: decode35
      },
   ),
   (
      10,
      MessageInfo{
         description: "MPU States",
         decoder: decode_mpu_dashboard_info
      },
   ),
   (
      772,
      MessageInfo{
         description: "GPS Data 1",
         decoder: decode_gps_1
      },
   ),
   (
      773, 
      MessageInfo{
         description: "GPS Data 2",
         decoder: decode_gps_2
      },
   ),
   (
      774, 
      MessageInfo{
         description: "GPS Data 3",
         decoder: decode_gps_3
      },
   ),
   (
      8,
      MessageInfo{
         description: "Cell Temperatures",
         decoder: decode_cell_temps
      },
   ),
   (
      9,
      MessageInfo{
         description: "Segment Temperatures",
         decoder: decode_segment_temps
      },
   ),
   (
      775,
      MessageInfo{
         description: "Logging Status",
         decoder: decode_logging_status
      },
   )
]

#[derive(Debug)]
struct DataInfo {
    name: &'static str,
    units: &'static str,
}

const DATA_IDS: HashMap<u32, DataInfo> = [
        (
            0,
            DataInfo {
                name: "Mock Data",
                units: "",
            },
        ),
        (
            1,
            DataInfo {
                name: "Pack Inst Voltage",
                units: "V",
            },
        ),
        (
            2,
            DataInfo {
                name: "Pack Current",
                units: "A",
            },
        ),
        (
            3,
            DataInfo {
                name: "Pack Amphours",
                units: "Ah",
            },
        ),
        (
            4,
            DataInfo {
                name: "Pack SOC",
                units: "%",
            },
        ),
        (
            5,
            DataInfo {
                name: "Pack Health",
                units: "%",
            },
        ),
        (
            6,
            DataInfo {
                name: "Failsafe Statuses",
                units: "HEX",
            },
        ),
        (
            7,
            DataInfo {
                name: "DTC Status 1",
                units: "HEX",
            },
        ),
        (
            8,
            DataInfo {
                name: "DTC Status 2",
                units: "HEX",
            },
        ),
        (
            9,
            DataInfo {
                name: "Current Limits Status",
                units: "",
            },
        ),
        (
            10,
            DataInfo {
                name: "Average Temp",
                units: "C",
            },
        ),
        (
            11,
            DataInfo {
                name: "Internal Temp",
                units: "C",
            },
        ),
        (
            12,
            DataInfo {
                name: "MPE State",
                units: "BIN",
            },
        ),
        (
            13,
            DataInfo {
                name: "High Cell Voltage",
                units: "V",
            },
        ),
        (
            14,
            DataInfo {
                name: "High Cell Voltage ID",
                units: "",
            },
        ),
        (
            15,
            DataInfo {
                name: "Low Cell Voltage",
                units: "V",
            },
        ),
        (
            16,
            DataInfo {
                name: "Low Cell Voltage ID",
                units: "",
            },
        ),
        (
            17,
            DataInfo {
                name: "Average Cell Voltage",
                units: "V",
            },
        ),
        (
            18,
            DataInfo {
                name: "Module A Temperature",
                units: "C",
            },
        ),
        (
            19,
            DataInfo {
                name: "Module B Temperature",
                units: "C",
            },
        ),
        (
            20,
            DataInfo {
                name: "Module C Temperature",
                units: "C",
            },
        ),
        (
            21,
            DataInfo {
                name: "Gate Driver Board Temperature",
                units: "C",
            },
        ),
        (
            22,
            DataInfo {
                name: "Control Board Temperature",
                units: "C",
            },
        ),
        (
            23,
            DataInfo {
                name: "RTD #1 Temperature",
                units: "C",
            },
         ),
         (
            24,
            DataInfo {
                  name: "RTD #2 Temperature",
                  units: "C",
               },
         ),
         (
            25,
            DataInfo {
                  name: "RTD #3 Temperature",
                  units: "C",
               },
         ),
         (
            26,
            DataInfo {
                  name: "RTD #4 Temperature",
                  units: "C",
               },
         ),
         (
            27,
            DataInfo {
                  name: "RTD #5 Temperature",
                  units: "C",
               },
         ),
         (
            28,
            DataInfo {
                  name: "Motor Temperature",
                  units: "C",
               },
         ),
         (
            29,
            DataInfo {
                  name: "Torque Shudder",
                  units: "N-m",
               },
         ),
         (
            30,
            DataInfo {
                  name: "Analog Input 1",
                  units: "V",
               },
         ),
         (
            31,
            DataInfo {
                  name: "Analog Input 2",
                  units: "V",
               },
         ),
         (
            32,
            DataInfo {
                  name: "Analog Input 3",
                  units: "V",
               },
         ),
         (
            33,
            DataInfo {
                  name: "Analog Input 4",
                  units: "V",
               },
         ),
         (
            34,
            DataInfo {
                  name: "Analog Input 5",
                  units: "V",
               },
         ),
         (
            35,
            DataInfo {
                  name: "Analog Input 6",
                  units: "V",
               },
         ),
         (
            36,
            DataInfo {
                  name: "Digital Input 1",
                  units: "BIN",
               },
         ),
         (
            37,
            DataInfo {
                  name: "Digital Input 2",
                  units: "BIN",
               },
         ),
         (
            38,
            DataInfo {
                  name: "Digital Input 3",
                  units: "BIN",
               },
         ),
         (
            39,
            DataInfo {
                  name: "Digital Input 4",
                  units: "BIN",
               },
         ),
         (
            40,
            DataInfo {
                  name: "Digital Input 5",
                  units: "BIN",
               },
         ),
         (
            41,
            DataInfo {
                  name: "Digital Input 6",
                  units: "BIN",
               },
         ),
         (
            42,
            DataInfo {
                  name: "Digital Input 7",
                  units: "BIN",
               },
         ),
         (
            43,
            DataInfo {
                  name: "Digital Input 8",
                  units: "BIN",
               },
         ),
         (
            44,
            DataInfo {
                  name: "Motor Angle (Electrical",
                  units: "Deg",
               },
         ),
         (
            45,
            DataInfo {
                  name: "Motor Speed",
                  units: "RPM",
               },
         ),
         (
            46,
            DataInfo {
                  name: "Electrical Output Frequency",
                  units: "Hz",
               },
         ),
         (
            48,
            DataInfo {
                  name: "Phase A Current",
                  units: "A",
               },
         ),
         (
            49,
            DataInfo {
                  name: "Phase B Current",
                  units: "A",
               },
         ),
         (
            50,
            DataInfo {
                  name: "Phase C Current",
                  units: "A",
               },
         ),
         (
            51,
            DataInfo {
                  name: "DC Bus Current",
                  units: "A",
               },
         ),
         (
            52,
            DataInfo {
                  name: "DC Bus Voltage",
                  units: "V",
               },
         ),
         (
            53,
            DataInfo {
                  name: "Output Voltage",
                  units: "V",
               },
         ),
         (
            54,
            DataInfo {
                  name: "VAB_Vd Voltage",
                  units: "V",
               },
         ),
         (
            55,
            DataInfo {
                  name: "VBC_Vq Voltage",
                  units: "V",
               },
         ),
         (
            56,
            DataInfo {
                  name: "Flux Command",
                  units: "Wb",
               },
         ),
         (
            57,
            DataInfo {
                  name: "Flux Feedback",
                  units: "wb",
               },
         ),
         (
            58,
            DataInfo {
                  name: "Id Feedback",
                  units: "A",
               },
         ),
         (
            59,
            DataInfo {
                  name: "Iq Feedback",
                  units: "A",
               },
         ),
         (
            60,
            DataInfo {
                  name: "1.5V Reference Voltage",
                  units: "V",
               },
         ),
         (
            61,
            DataInfo {
                  name: "2.5V Reference Voltage",
                  units: "V",
               },
         ),
         (
            62,
            DataInfo {
                  name: "5.0V Reference Voltage",
                  units: "V",
               },
         ),
         (
            63,
            DataInfo {
                  name: "12V System Voltage",
                  units: "V",
               },
         ),
         (
            64,
            DataInfo {
                  name: "VSM State",
                  units: "",
               },
         ),
         (
            65,
            DataInfo {
                  name: "Inverter State",
                  units: "",
               },
         ),
         (
            66,
            DataInfo {
                  name: "Relay State",
                  units: "BIN",
               },
         ),
         (
            67,
            DataInfo {
                  name: "Inverter Run Mode",
                  units: "BIN",
               },
         ),
         (
            68,
            DataInfo {
                  name: "Inverter Active Discharge State",
                  units: "BIN",
               },
         ),
         (
            69,
            DataInfo {
                  name: "Inverter Command Mode",
                  units: "BIN",
               },
         ),
         (
            70,
            DataInfo {
                  name: "Inverter Enable State",
                  units: "BIN",
               },
         ),
         (
            71,
            DataInfo {
                  name: "Inverter Enable Lockout",
                  units: "BIN",
               },
         ),
         (
            72,
            DataInfo {
                  name: "Direction Command",
                  units: "BIN",
               },
         ),
         (
            73,
            DataInfo {
                  name: "BMS Active",
                  units: "BIN",
               },
         ),
         (
            74,
            DataInfo {
                  name: "BMS Limiting Torque",
                  units: "BIN",
               },
         ),
         (
            75,
            DataInfo {
                  name: "POST Fault Lo",
                  units: "BIN",
               },
         ),
         (
            76,
            DataInfo {
                  name: "POST Fault Hi",
                  units: "BIN",
               },
         ),
         (
            77,
            DataInfo {
                  name: "Run Fault Lo",
                  units: "BIN",
               },
         ),
         (
            78,
            DataInfo {
                  name: "Run Fault Hi",
                  units: "BIN",
               },
         ),
         (
            79,
            DataInfo {
                  name: "Commanded Torque",
                  units: "N-m",
               },
         ),
         (
            80,
            DataInfo {
                  name: "Torque Feedback",
                  units: "N-m",
               },
         ),
         (
            81,
            DataInfo {
                  name: "Power on Timer",
                  units: "s",
               },
         ),
         (
            82,
            DataInfo {
                  name: "Torque Command",
                  units: "N-m",
               },
         ),
         (
            83,
            DataInfo {
                  name: "Speed Command",
                  units: "RPM",
               },
         ),
         (
            84,
            DataInfo {
                  name: "Direction Command",
                  units: "BIN",
               },
         ),
         (
            85,
            DataInfo {
                  name: "Inverter Enable",
                  units: "BIN",
               },
         ),
         (
            86,
            DataInfo {
                  name: "Inverter Discharge",
                  units: "BIN",
               },
         ),
         (
            87,
            DataInfo {
                  name: "Speed Mode Enable",
                  units: "BIN",
               },
         ),
         (
            88,
            DataInfo {
                  name: "Commanded Torque Limit",
                  units: "N-m",
               },
         ),
         (
            89,
            DataInfo {
                  name: "Pack DCL",
                  units: "A",
               },
         ),
         (
            90,
            DataInfo {
                  name: "Pack CCL",
                  units: "A",
               },
         ),
         (
            91,
            DataInfo {
                  name: "TCU X-Axis Acceleration",
                  units: "g",
               },
         ),
         (
            92,
            DataInfo {
                  name: "TCU Y-Axis Acceleration",
                  units: "g",
               },
         ),
         (
            93,
            DataInfo {
                  name: "TCU Z-Axis Acceleration",
                  units: "g",
               },
         ),
         (
            94,
            DataInfo {
                  name: "TCU Temperature C",
                  units: "C",
               },
         ),
         (
            95,
            DataInfo {
                  name: "TCU Temperature F",
                  units: "F",
               },
         ),
         (
            96,
            DataInfo {
                  name: "Relative Humidity",
                  units: "%",
               },
         ),
         (
            97,
            DataInfo {
                  name: "Cell Voltage Info",
                  units: "",
               },
         ),
         (
            98,
            DataInfo {
                  name: "GLV Current",
                  units: "A",
               },
         ),
         (
            99,
            DataInfo {
                  name: "Strain Gauge Voltage 1",
                  units: "V",
               },
         ),
         (
            100,
            DataInfo {
                  name: "Strain Gauge Voltage 2",
                  units: "V",
               },
         ),
         (
            101,
            DataInfo {
                  name: "Vehicle Speed",
                  units: "MPH",
               },
         ),
         (
            102,
            DataInfo {
                  name: "Wheel Knob 1",
                  units: "",
               },
         ),
         (
            103,
            DataInfo {
                  name: "Wheel Knob 2",
                  units: "",
               },
         ),
         (
            104,
            DataInfo {
                  name: "Wheel Buttons",
                  units: "",
               },
         ),
         (
            105,
            DataInfo {
                  name: "MPU Mode State",
                  units: "",
               },
         ),
         (
            106,
            DataInfo {
                  name: "BMS State",
                  units: "",
               },
         ),
         (
            107,
            DataInfo {
                  name: "BMS Faults",
                  units: "HEX",
               },
         ),
         (
            108,
            DataInfo {
                  name: "Latitude",
                  units: "Deg",
               },
         ),
         (
            109,
            DataInfo {
                  name: "Longitude",
                  units: "Deg",
               },
         ),
         (
            110,
            DataInfo {
                  name: "GPS Fix Status",
                  units: "",
               },
         ),
         (
            111,
            DataInfo {
                  name: "Altitude",
                  units: "m",
               },
         ),
         (
            112,
            DataInfo {
                  name: "Ground Speed",
                  units: "m/s",
               },
         ),
         (
            113,
            DataInfo {
                  name: "Heading Direction",
                  units: "Deg",
               },
         ),
         (
            114,
            DataInfo {
                  name: "High Cell Temp",
                  units: "C",
               },
         ),
         (
            115,
            DataInfo {
                  name: "High Cell Temp Chip Number",
                  units: "",
               },
         ),
         (
            116,
            DataInfo {
                  name: "High Cell Temp Cell Number",
                  units: "",
               },
         ),
         (
            117,
            DataInfo {
                  name: "Low Cell Temp",
                  units: "C",
               },
         ),
         (
            118,
            DataInfo {
                  name: "Low Cell Temp Chip Number",
                  units: "",
               },
         ),
         (
            119,
            DataInfo {
                  name: "Low Cell temp Cell Number",
                  units: "",
               },
         ),
         (
            120,
            DataInfo {
                  name: "Average Cell Temp",
                  units: "C",
               },
         ),
         (
            121,
            DataInfo {
                  name: "High Cell Voltage Chip Number",
                  units: "",
               },
         ),
         (
            122,
            DataInfo {
                  name: "High Cell Voltage Cell Number",
                  units: "",
               },
         ),
         (
            123,
            DataInfo {
                  name: "Low Cell Voltage Chip Number",
                  units: "",
               },
         ),
         (
            124,
            DataInfo {
                  name: "Low Cell Voltage Cell Number",
                  units: "",
               },
         ),
         (
            125,
            DataInfo {
                  name: "Segment 1 Average Temperature",
                  units: "C",
               },
         ),
         (
            126,
            DataInfo {
                  name: "Segment 2 Average Temperature",
                  units: "C",
               },
         ),
         (
            127,
            DataInfo {
                  name: "Segment 3 Average Temperature",
                  units: "C",
               },
         ),
         (
            128,
            DataInfo {
                  name: "Segment 4 Average Temperature",
                  units: "C",
               },
         ),
         (
            129,
            DataInfo {
                  name: "Logging Status",
                  units: "",
               },
         ),
         (
            130,
            DataInfo {
                  name: "Accumulator Fan Percentage",
                  units: "%",
               },
         ),
         (
            131,
            DataInfo {
                  name: "Motor Fan Percentage",
                  units: "%",
               },
         ),
         (
            132,
            DataInfo {
                  name: "Torque Limit Percentage",
                  units: "%",
               },
         ),
         (
            133,
            DataInfo {
                  name: "Regen Strength value",
                  units: "",
               },
         )
]

