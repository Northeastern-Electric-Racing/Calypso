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
    }
}

# Mapping from data ids to their description (potentially add format information)
DATA_IDS = {
    0: {
        "name": "Mock Data",
        "units": "",
    },
    1: {
        "name": "Pack Inst Voltage",
        "units": "FIND",
    },
    2: {
        "name": "Pack Current",
        "units": "FIND",
    },
    3: {
        "name": "Pack Amphours",
        "units": "FIND",
    },
    4: {
        "name": "Pack SOC",
        "units": "FIND",
    },
    5: {
        "name": "Pack Health",
        "units": "FIND",
    },
    6: {
        "name": "Pack Current",
        "units": "FIND",
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
        "units": "FIND",
    },
    10: {
        "name": "Average Temp",
        "units": "FIND",
    },
    11: {
        "name": "Internal Temp",
        "units": "FIND",
    },
    12: {
        "name": "MPE State",
        "units": "FIND",
    },
    13: {
        "name": "High Cell Voltage",
        "units": "FIND",
    },
    14: {
        "name": "High Cell Voltage ID",
        "units": "FIND",
    },
    15: {
        "name": "Low Cell Voltage",
        "units": "FIND",
    },
    16: {
        "name": "Low Cell Voltage ID",
        "units": "FIND",
    },
    17: {
        "name": "Average Cell Voltage",
        "units": "FIND",
    },
    18: {
        "name": "Module A Temperature",
        "units": "Degrees C",
    },
    19: {
        "name": "Module B Temperature",
        "units": "Degrees C",
    },
    20: {
        "name": "Module C Temperature",
        "units": "Degrees C",
    },
    21: {
        "name": "Gate Driver Board Temperature",
        "units": "Degrees C",
    },
    22: {
        "name": "Control Board Temperature",
        "units": "Degrees C",
    },
    23: {
        "name": "RTD #1 Temperature",
        "units": "Degrees C",
    },
    24: {
        "name": "RTD #2 Temperature",
        "units": "Degrees C",
    },
    25: {
        "name": "RTD #3 Temperature",
        "units": "Degrees C",
    },
    26: {
        "name": "RTD #4 Temperature",
        "units": "Degrees C",
    },
    27: {
        "name": "RTD #5 Temperature",
        "units": "Degrees C",
    },
    28: {
        "name": "Motor Temperature",
        "units": "Degrees C",
    },
    29: {
        "name": "Torque Shudder",
        "units": "Degrees C",
    },
    30: {
        "name": "Analog Input 1",
        "units": "",
    },
    31: {
        "name": "Analog Input 2",
        "units": "",
    },
    32: {
        "name": "Analog Input 3",
        "units": "",
    },
    33: {
        "name": "Analog Input 4",
        "units": "",
    },
    34: {
        "name": "Analog Input 5",
        "units": "",
    },
    35: {
        "name": "Analog Input 6",
        "units": "",
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
        "units": "degrees",
    },
    45: {
        "name": "Motor Speed",
        "units": "rpm",
    },
    46: {
        "name": "Electrical Output Frequency",
        "units": "Hz",
    },
    47: {
        "name": "Delta Resolver Filtered",
        "units": "degrees",
    },
    48: {
        "name": "Phase A Current",
        "units": "amps",
    },
    49: {
        "name": "Phase B Current",
        "units": "amps",
    },
    50: {
        "name": "Phase C Current",
        "units": "amps",
    },
    51: {
        "name": "DC Bus Current",
        "units": "amps",
    },
    52: {
        "name": "DC Bus Voltage",
        "units": "volts",
    },
    53: {
        "name": "Output Voltage",
        "units": "volts",
    },
    54: {
        "name": "VAB_Vd Voltage",
        "units": "volts",
    },
    55: {
        "name": "VBC_Vq Voltage",
        "units": "volts",
    },
    56: {
        "name": "Flux Command",
        "units": "Webers",
    },
    57: {
        "name": "Flux Feedback",
        "units": "Webers",
    },
    58: {
        "name": "Id Feedback",
        "units": "amps",
    },
    59: {
        "name": "Iq Feedback",
        "units": "amps",
    },
    60: {
        "name": "1.5V Reference Voltage",
        "units": "volts",
    },
    61: {
        "name": "2.5V Reference Voltage",
        "units": "volts",
    },
    62: {
        "name": "5.0V Reference Voltage",
        "units": "volts",
    },
    63: {
        "name": "12V System Voltage",
        "units": "volts",
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
        "units": "sec",
    },
    82: {
        "name": "Torque Command",
        "units": "N-m",
    },
    83: {
        "name": "Speed Command",
        "units": "rpm",
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
        "units": "FIND",
    },
    90: {
        "name": "Pack CCL",
        "units": "FIND",
    },
    91: {
        "name": "TCU X-Axis Acceleration",
        "units": "FIND",
    },
    92: {
        "name": "TCU Y-Axis Acceleration",
        "units": "FIND",
    },
    93: {
        "name": "TCU Z-Axis Acceleration",
        "units": "FIND",
    },
    94: {
        "name": "TCU Temperature C",
        "units": "Degrees C",
    },
    95: {
        "name": "TCU Temperature F",
        "units": "Degrees F",
    },
    96: {
        "name": "Relative Humidity",
        "units": "FIND",
    },
    97: {
        "name": "Cell Voltage Info",
        "units": "",
    },
    98: {
        "name": "GLV Current",
        "units": "FIND",
    },
    99: {
        "name": "Strain Gauge Voltage 1",
        "units": "FIND",
    },
    100: {
        "name": "Strain Gauge Voltage 2",
        "units": "FIND",
    },
    101: {
        "name": "Vehicle Speed",
        "units": "mph",
    },
}

