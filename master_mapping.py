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
    165: 8,
    166: 9,
    167: 10,
    170: 11,
    171: 12,
    172: 13,
    514: 14,
    768: 15,
    769: 16,
    192: 17
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
        "description": "motor position information",
        "decoder": decode8,
    },
    9: {
        "description": "current information",
        "decoder": decode9,
    },
    10: {
        "description": "voltage information",
        "decoder": decode10,
    },
    11: {
        "description": "internal states",
        "decoder": decode11,
    },
    12: {
        "description": "fault codes",
        "decoder": decode12,
    },
    13: {
        "description": "torque and timer",
        "decoder": decode13,
    },
    14: {
        "description": "current limits",
        "decoder": decode14,
    },
    15: {
        "description": "nerduino accelerometer",
        "decoder": decode15,
    },
    16: {
        "description": "nerduino humidity",
        "decoder": decode16,
    },
    17: {
        "description": "commanded data",
        "decoder": decode17
    }
}

# Mapping from data ids to their description (potentially add format information)
DATA_IDS = {
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
    30: "Motor Angle (Electrical)",
    31: "Motor Speed",
    32: "Electrical Output Frequency",
    33: "Delta Resolver Filtered",
    34: "Phase A Current",
    35: "Phase B Current",
    36: "Phase C Current",
    37: "DC Bus Current",
    38: "DC Bus Voltage",
    39: "Output Voltage",
    40: "VAB_Vd Voltage",
    41: "VBC_Vq Voltage",
    42: "VSM State",
    43: "Inverter State",
    44: "Relay State",
    45: "Inverter Run Mode",
    46: "Inverter Active Discharge State",
    47: "Inverter Command Mode",
    48: "Inverter Enable State",
    49: "Inverter Enable Lockout",
    50: "Direction Command",
    51: "BMS Active",
    52: "BMS Limiting Torque",
    53: "POST Fault Lo",
    54: "POST Fault Hi",
    55: "Run Fault Lo",
    56: "Run Fault Hi",
    57: "Commanded Torque",
    58: "Torque Feedback",
    59: "Power on Timer",
    60: "Pack DCL",
    61: "Pack CCL",
    62: "TCU X-Axis Acceleration",
    63: "TCU Y-Axis Acceleration",
    64: "TCU Z-Axis Acceleration",
    65: "TCU Temperature C",
    66: "TCU Temperature F",
    67: "Relative Humidity",
    68: "Torque Command",
    69: "Speed Command",
    70: "Direction Command",
    71: "Inverter Enable",
    72: "Inverter Discharge",
    73: "Speed Mode Enable",
    74: "Commanded Torque Limit"
}

