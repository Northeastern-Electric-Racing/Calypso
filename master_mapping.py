"""
This file specifes the CAN and data ID mappings. IDS:
    - External Message ID (actual CAN message id)
    - Data ID (id for individual data values contained in the messages)
"""

from ner_processing.decode_data import *

# Mapping from external message ID to decoding information
MESSAGE_IDS = {
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
    160: {
        "description": "temperatures (igbt modules, gate driver board)",
        "decoder": decode5
    },
    161: {
        "description": "temperatures (control board)",
        "decoder": decode6,
    },
    162: {
        "description": "temperatures (motor)",
        "decoder": decode7,
    },
    163: {
        "description": "analog input voltages",
        "decoder": decode8,
    },
    164: {
        "description": "digital input status",
        "decoder": decode9,
    },
    165: {
        "description": "motor position information",
        "decoder": decode10,
    },
    166: {
        "description": "current information",
        "decoder": decode11,
    },
    167: {
        "description": "voltage information",
        "decoder": decode12,
    },
    168: {
        "description": "flux information",
        "decoder": decode13,
    },
    169: {
        "description": "internal voltages",
        "decoder": decode14,
    },
    170: {
        "description": "internal states",
        "decoder": decode15,
    },
    171: {
        "description": "fault codes",
        "decoder": decode16,
    },
    172: {
        "description": "torque and timer",
        "decoder": decode17,
    },
    192: {
        "description": "commanded data",
        "decoder": decode18,
    },
    514: {
        "description": "current limits",
        "decoder": decode19,
    },
    768: {
        "description": "nerduino accelerometer",
        "decoder": decode20,
    },
    769: {
        "description": "nerduino humidity",
        "decoder": decode21,
    },
    7: {
        "description": "cell voltages",
        "decoder": decode22,
    },
    193: {
        "description": "unknown 1",
        "decoder": decodeMock,
    },
    6: {
        "description": "unknown 2",
        "decoder": decodeMock,
    },
    194: {
        "description": "unknown 3",
        "decoder": decodeMock,
    },
    1744: {
        "description": "unknown 4",
        "decoder": decodeMock,
    },
    1745: {
        "description": "unknown 5",
        "decoder": decodeMock,
    },
    175: {
        "description": "unknown 6",
        "decoder": decodeMock,
    },
    770: {
        "description": "GLV current",
        "decoder": decode29,
    },
    2015: {
        "description": "unknown 2015",
        "decoder": decodeMock,
    },
    2027: {
        "description": "unknown 2027",
        "decoder": decodeMock,
    },
    2019: {
        "description": "unknown 2019",
        "decoder": decodeMock,
    },
    5: {
        "description": "Is-Charging",
        "decoder": decodeMock,
    },
    771: {
        "description": "strain gauge",
        "decoder": decode34,
    },
    1024: {
        "description": "wheel state",
        "decoder": decode35,
    },
}

# Mapping from data ids to their description (potentially add format information)
DATA_IDS = {
    0: {
        "name": "Mock Data",
        "units": "",
    },
    1: {
        "name": "Pack Inst Voltage",
        "units": "V",
    },
    2: {
        "name": "Pack Current",
        "units": "A",
    },
    3: {
        "name": "Pack Amphours",
        "units": "Ah",
    },
    4: {
        "name": "Pack SOC",
        "units": "%",
    },
    5: {
        "name": "Pack Health",
        "units": "%",
    },
    6: {
        "name": "Failsafe Statuses",
        "units": "",
    },
    7: {
        "name": "DTC Status 1",
        "units": "",
    },
    8: {
        "name": "DTC Status 2",
        "units": "",
    },
    9: {
        "name": "Current Limits Status",
        "units": "",
    },
    10: {
        "name": "Average Temp",
        "units": "C",
    },
    11: {
        "name": "Internal Temp",
        "units": "C",
    },
    12: {
        "name": "MPE State",
        "units": "",
    },
    13: {
        "name": "High Cell Voltage",
        "units": "V",
    },
    14: {
        "name": "High Cell Voltage ID",
        "units": "",
    },
    15: {
        "name": "Low Cell Voltage",
        "units": "V",
    },
    16: {
        "name": "Low Cell Voltage ID",
        "units": "",
    },
    17: {
        "name": "Average Cell Voltage",
        "units": "V",
    },
    18: {
        "name": "Module A Temperature",
        "units": "C",
    },
    19: {
        "name": "Module B Temperature",
        "units": "C",
    },
    20: {
        "name": "Module C Temperature",
        "units": "C",
    },
    21: {
        "name": "Gate Driver Board Temperature",
        "units": "C",
    },
    22: {
        "name": "Control Board Temperature",
        "units": "C",
    },
    23: {
        "name": "RTD #1 Temperature",
        "units": "C",
    },
    24: {
        "name": "RTD #2 Temperature",
        "units": "C",
    },
    25: {
        "name": "RTD #3 Temperature",
        "units": "C",
    },
    26: {
        "name": "RTD #4 Temperature",
        "units": "C",
    },
    27: {
        "name": "RTD #5 Temperature",
        "units": "C",
    },
    28: {
        "name": "Motor Temperature",
        "units": "C",
    },
    29: {
        "name": "Torque Shudder",
        "units": "",
    },
    30: {
        "name": "Analog Input 1",
        "units": "V",
    },
    31: {
        "name": "Analog Input 2",
        "units": "V",
    },
    32: {
        "name": "Analog Input 3",
        "units": "V",
    },
    33: {
        "name": "Analog Input 4",
        "units": "V",
    },
    34: {
        "name": "Analog Input 5",
        "units": "V",
    },
    35: {
        "name": "Analog Input 6",
        "units": "V",
    },
    36: {
        "name": "Digital Input 1",
        "units": "",
    },
    37: {
        "name": "Digital Input 2",
        "units": "",
    },
    38: {
        "name": "Digital Input 3",
        "units": "",
    },
    39: {
        "name": "Digital Input 4",
        "units": "",
    },
    40: {
        "name": "Digital Input 5",
        "units": "",
    },
    41: {
        "name": "Digital Input 6",
        "units": "",
    },
    42: {
        "name": "Digital Input 7",
        "units": "",
    },
    43: {
        "name": "Digital Input 8",
        "units": "",
    },
    44: {
        "name": "Motor Angle (Electrical)",
        "units": "Deg",
    },
    45: {
        "name": "Motor Speed",
        "units": "RPM",
    },
    46: {
        "name": "Electrical Output Frequency",
        "units": "Hz",
    },
    47: {
        "name": "Delta Resolver Filtered",
        "units": "Deg",
    },
    48: {
        "name": "Phase A Current",
        "units": "A",
    },
    49: {
        "name": "Phase B Current",
        "units": "A",
    },
    50: {
        "name": "Phase C Current",
        "units": "A",
    },
    51: {
        "name": "DC Bus Current",
        "units": "A",
    },
    52: {
        "name": "DC Bus Voltage",
        "units": "V",
    },
    53: {
        "name": "Output Voltage",
        "units": "V",
    },
    54: {
        "name": "VAB_Vd Voltage",
        "units": "V",
    },
    55: {
        "name": "VBC_Vq Voltage",
        "units": "V",
    },
    56: {
        "name": "Flux Command",
        "units": "Wb",
    },
    57: {
        "name": "Flux Feedback",
        "units": "Wb",
    },
    58: {
        "name": "Id Feedback",
        "units": "A",
    },
    59: {
        "name": "Iq Feedback",
        "units": "A",
    },
    60: {
        "name": "1.5V Reference Voltage",
        "units": "V",
    },
    61: {
        "name": "2.5V Reference Voltage",
        "units": "V",
    },
    62: {
        "name": "5.0V Reference Voltage",
        "units": "V",
    },
    63: {
        "name": "12V System Voltage",
        "units": "V",
    },
    64: {
        "name": "VSM State",
        "units": "",
    },
    65: {
        "name": "Inverter State",
        "units": "",
    },
    66: {
        "name": "Relay State",
        "units": "",
    },
    67: {
        "name": "Inverter Run Mode",
        "units": "",
    },
    68: {
        "name": "Inverter Active Discharge State",
        "units": "",
    },
    69: {
        "name": "Inverter Command Mode",
        "units": "",
    },
    70: {
        "name": "Inverter Enable State",
        "units": "",
    },
    71: {
        "name": "Inverter Enable Lockout",
        "units": "",
    },
    72: { 
        "name": "Direction Command", 
        "units": "" 
    },
    73: {
        "name": "BMS Active",
        "units": "",
    },
    74: {
        "name": "BMS Limiting Torque",
        "units": "",
    },
    75: {
        "name": "POST Fault Lo",
        "units": "",
    },
    76: {
        "name": "POST Fault Hi",
        "units": "",
    },
    77: {
        "name": "Run Fault Lo",
        "units": "",
    },
    78: {
        "name": "Run Fault Hi",
        "units": "",
    },
    79: {
        "name": "Commanded Torque",
        "units": "N-m",
    },
    80: {
        "name": "Torque Feedback",
        "units": "N-m",
    },
    81: {
        "name": "Power on Timer",
        "units": "s",
    },
    82: {
        "name": "Torque Command",
        "units": "N-m",
    },
    83: {
        "name": "Speed Command",
        "units": "RPM",
    },
    84: {
        "name": "Direction Command",
        "units": "",
    },
    85: {
        "name": "Inverter Enable",
        "units": "",
    },
    86: {
        "name": "Inverter Discharge",
        "units": "",
    },
    87: {
        "name": "Speed Mode Enable",
        "units": "",
    },
    88: {
        "name": "Commanded Torque Limit",
        "units": "N-m",
    },
    89: {
        "name": "Pack DCL",
        "units": "A",
    },
    90: {
        "name": "Pack CCL",
        "units": "A",
    },
    91: {
        "name": "TCU X-Axis Acceleration",
        "units": "g",
    },
    92: {
        "name": "TCU Y-Axis Acceleration",
        "units": "g",
    },
    93: {
        "name": "TCU Z-Axis Acceleration",
        "units": "g",
    },
    94: {
        "name": "TCU Temperature C",
        "units": "C",
    },
    95: {
        "name": "TCU Temperature F",
        "units": "F",
    },
    96: {
        "name": "Relative Humidity",
        "units": "%",
    },
    97: {
        "name": "Cell Voltage Info",
        "units": "",
    },
    98: {
        "name": "GLV Current",
        "units": "A",
    },
    99: {
        "name": "Strain Gauge Voltage 1",
        "units": "V",
    },
    100: {
        "name": "Strain Gauge Voltage 2",
        "units": "V",
    },
    101: {
        "name": "Vehicle Speed",
        "units": "MPH",
    },
    102: {
        "name": "Wheel Knob 1",
        "units": "",
    },
    103: {
        "name": "Wheel Knob 2",
        "units": "",
    },
    104: {
        "name": "Wheel Buttons",
        "units": "",
    },
}

