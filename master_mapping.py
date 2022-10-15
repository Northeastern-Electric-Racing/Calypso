"""
This file specifes the CAN and data ID mappings. There are three levels of IDs specified:
    - External Message ID (actual CAN message id)
    - Internal Message ID (internal id for messages to allow easier changing of external ids)
    - Data ID (id for individual data values contained in the messages)
"""

from ner_processing.decode_data import *

# Mapping from external to internal message IDs
MESSAGE_IDS = {
    1: 1,
    2: 2,
    3: 3,
    4: 4,
    160: 5,
    161: 6,
    162: 7,
    163: 8,
    164: 9,
    165: 10,
    166: 11,
    167: 12,
    168: 13,
    169: 14,
    170: 15,
    171: 16,
    172: 17,
    192: 18,
    514: 19,
    768: 20,
    769: 21,
    7: 22,
    193: 23,
    6: 24,
    194: 25,
    1744: 26,
    1745: 27,
    175: 28,
    770: 29,
    2015: 30,
    2027: 31,
    2019: 32,
    5: 33,
    771: 34
}

# Mapping from internal message IDs to information used to decode the message
DECODE_DATA = {
    1: {
        "description": "accumulator status",
        "decoder": decode1
    },
    2: {
        "description": "BMS status",
        "decoder": decode2
    },
    3: {
        "description": "shutdown control",
        "decoder": decode3
    },
    4: {
        "description": "cell data",
        "decoder": decode4
    },
    5: {
        "description": "temperatures (igbt modules, gate driver board)",
        "decoder": decode5
    },
    6: {
        "description": "temperatures (control board)",
        "decoder": decode6,
    },
    7: {
        "description": "temperatures (motor)",
        "decoder": decode7,
    },
    8: {
        "description": "analog input voltages",
        "decoder": decode8,
    },
    9: {
        "description": "digital input status",
        "decoder": decode9,
    },
    10: {
        "description": "motor position information",
        "decoder": decode10,
    },
    11: {
        "description": "current information",
        "decoder": decode11,
    },
    12: {
        "description": "voltage information",
        "decoder": decode12,
    },
    13: {
        "description": "flux information",
        "decoder": decode13,
    },
    14: {
        "description": "internal voltages",
        "decoder": decode14,
    },
    15: {
        "description": "internal states",
        "decoder": decode15,
    },
    16: {
        "description": "fault codes",
        "decoder": decode16,
    },
    17: {
        "description": "torque and timer",
        "decoder": decode17,
    },
    18: {
        "description": "commanded data",
        "decoder": decode18,
    },
    19: {
        "description": "current limits",
        "decoder": decode19,
    },
    20: {
        "description": "nerduino accelerometer",
        "decoder": decode20,
    },
    21: {
        "description": "nerduino humidity",
        "decoder": decode21,
    },
    22: {
        "description": "cell voltages",
        "decoder": decode22,
    },
    23: {
        "description": "unknown 1",
        "decoder": decodeMock,
    },
    24: {
        "description": "unknown 2",
        "decoder": decodeMock,
    },
    25: {
        "description": "unknown 3",
        "decoder": decodeMock,
    },
    26: {
        "description": "unknown 4",
        "decoder": decodeMock,
    },
    27: {
        "description": "unknown 5",
        "decoder": decodeMock,
    },
    28: {
        "description": "unknown 6",
        "decoder": decodeMock,
    },
    29: {
        "description": "GLV current",
        "decoder": decode29,
    },
    30: {
        "description": "unknown 2015",
        "decoder": decodeMock,
    },
    31: {
        "description": "unknown 2027",
        "decoder": decodeMock,
    },
    32: {
        "description": "unknown 2019",
        "decoder": decodeMock,
    },
    33: {
        "description": "Is-Charging",
        "decoder": decodeMock,
    },
    34: {
        "description": "strain gauge",
        "decoder": decode34,
    }
}

# Mapping from data ids to their description (potentially add format information)
DATA_IDS = {
    0: "Mock Data",
    1: "Pack Inst Voltage",
    2: "Pack Current",
    3: "Pack Amphours",
    4: "Pack SOC",
    5: "Pack Health",
    6: "Failsafe Statuses",
    7: "DTC Status 1",
    8: "DTC Status 2",
    9: "Current Limits Status",
    10: "Average Temp",
    11: "Internal Temp",
    12: "MPE State",
    13: "High Cell Voltage",
    14: "High Cell Voltage ID",
    15: "Low Cell Voltage",
    16: "Low Cell Voltage ID",
    17: "Average Cell Voltage",
    18: "Module A Temperature",
    19: "Module B Temperature",
    20: "Module C Temperature",
    21: "Gate Driver Board Temperature",
    22: "Control Board Temperature",
    23: "RTD #1 Temperature",
    24: "RTD #2 Temperature",
    25: "RTD #3 Temperature",
    26: "RTD #4 Temperature",
    27: "RTD #5 Temperature",
    28: "Motor Temperature",
    29: "Torque Shudder",
    30: "Analog Input 1",
    31: "Analog Input 2",
    32: "Analog Input 3",
    33: "Analog Input 4",
    34: "Analog Input 5",
    35: "Analog Input 6",
    36: "Digital Input 1",
    37: "Digital Input 2",
    38: "Digital Input 3",
    39: "Digital Input 4",
    40: "Digital Input 5",
    41: "Digital Input 6",
    42: "Digital Input 7",
    43: "Digital Input 8",
    44: "Motor Angle (Electrical)",
    45: "Motor Speed",
    46: "Electrical Output Frequency",
    47: "Delta Resolver Filtered", 
    48: "Phase A Current",
    49: "Phase B Current",
    50: "Phase C Current",
    51: "DC Bus Current",
    52: "DC Bus Voltage",
    53: "Output Voltage",
    54: "VAB_Vd Voltage",
    55: "VBC_Vq Voltage",
    56: "Flux Command",
    57: "Flux Feedback",
    58: "Id Feedback",
    59: "Iq Feedback",
    60: "1.5V Reference Voltage",
    61: "2.5V Reference Voltage",
    62: "5.0V Reference Voltage",
    63: "12V System Voltage",
    64: "VSM State",
    65: "Inverter State",
    66: "Relay State",
    67: "Inverter Run Mode",
    68: "Inverter Active Discharge State",
    69: "Inverter Command Mode",
    70: "Inverter Enable State",
    71: "Inverter Enable Lockout",
    72: "Direction Command",
    73: "BMS Active",
    74: "BMS Limiting Torque",
    75: "POST Fault Lo",
    76: "POST Fault Hi",
    77: "Run Fault Lo",
    78: "Run Fault Hi",
    79: "Commanded Torque",
    80: "Torque Feedback",
    81: "Power on Timer",
    82: "Torque Command",
    83: "Speed Command",
    84: "Direction Command",
    85: "Inverter Enable",
    86: "Inverter Discharge",
    87: "Speed Mode Enable",
    88: "Commanded Torque Limit",
    89: "Pack DCL",
    90: "Pack CCL",
    91: "TCU X-Axis Acceleration",
    92: "TCU Y-Axis Acceleration",
    93: "TCU Z-Axis Acceleration",
    94: "TCU Temperature C",
    95: "TCU Temperature F",
    96: "Relative Humidity",
    97: "Cell Voltage Info",
    98: "GLV Current",
    99: "Strain Gauge Voltage 1",
    100: "Strain Gauge Voltage 2"
}

