"""
This file specifies methods to decode messages into the many pieces of data they contain.
"""

from typing import Any, Dict, List

from data import (
    ProcessData as pd,
    FormatData as fd,
)


def decodeMock(data: List[int]) -> Dict[int, Any]:
    return {
        0: 0
    }

def decodeAccumulatorStatus(data: List[int]) -> Dict[int, Any]:
    return {
        1: pd.bigEndian(data[0:2]),
        2: pd.twosComp(pd.bigEndian(data[2:4])) / 10,
        3: pd.bigEndian(data[4:6]),
        4: data[6],
        5: data[7]
    }

def decodeBMSStatus(data: List[int]) -> Dict[int, Any]:
    return {
        106: data[0],
        107: pd.littleEndian(data[1:5]),
        10: pd.twosComp(data[5], 8),
        11: pd.twosComp(data[6], 8)
    }

def decode3(data: List[int]) -> Dict[int, Any]:
    return {
        12: data[0]
    }

def decodeCellVoltages(data: List[int]) -> Dict[int, Any]:
    high_cell_volt_chip_number = (data[2] >> 0) & 15
    high_cell_volt_cell_number = (data[2] >> 4) & 15
    low_cell_volt_chip_number = (data[5] >> 0) & 15
    low_cell_volt_cell_number = (data[5] >> 4) & 15
    return {
        13: pd.bigEndian(data[0:2]),
        121: high_cell_volt_chip_number,
        122: high_cell_volt_cell_number,
        15: pd.bigEndian(data[3:5]),
        123: low_cell_volt_chip_number,
        124: low_cell_volt_cell_number,
        17: pd.bigEndian(data[6:8])
    }

def decode5(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.temperature(d) for d in decoded_data]
    return {
        18: final_data[0],
        19: final_data[1],
        20: final_data[2],
        21: final_data[3]
    }

def decode6(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.temperature(d) for d in decoded_data]
    return {
        22: final_data[0],
        23: final_data[1],
        24: final_data[2],
        25: final_data[3]
    }

def decode7(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.temperature(d) for d in decoded_data[:3]]
    return {
        26: final_data[0],
        27: final_data[1],
        28: final_data[2],
        29: fd.torque(decoded_data[3])
    }

# TODO: Fill this method out (complicated with bit shifts)
def decode8(data: List[int]) -> Dict[int, Any]:
    return {
        30: 0,
        31: 0,
        32: 0,
        33: 0,
        34: 0,
        35: 0
    }

def decode9(data: List[int]) -> Dict[int, Any]:
    return {
        36: data[0],
        37: data[1],
        38: data[2],
        39: data[3],
        40: data[4],
        41: data[5],
        42: data[6],
        43: data[7]
    }

def decode10(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    motor_speed = fd.angularVelocity(decoded_data[1])
    vehicle_speed = motor_speed * 0.013048225
    return {
        44: fd.angle(decoded_data[0]),
        45: motor_speed,
        46: fd.frequency(decoded_data[2]),
        47: fd.angle(decoded_data[3]),
        101: vehicle_speed
    }

def decode11(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.current(d) for d in decoded_data]
    return {
        48: final_data[0],
        49: final_data[1],
        50: final_data[2],
        51: final_data[3]
    }

def decode12(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.highVoltage(d) for d in decoded_data]
    return {
        52: final_data[0],
        53: final_data[1],
        54: final_data[2],
        55: final_data[3]
    }

def decode13(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    return {
        56: fd.flux(decoded_data[0]),
        57: fd.flux(decoded_data[1]),
        58: fd.current(decoded_data[2]),
        59: fd.current(decoded_data[3])
    }

def decode14(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.lowVoltage(d) for d in decoded_data]
    return {
        60: final_data[0],
        61: final_data[1],
        62: final_data[2],
        63: final_data[3]
    }

def decode15(data: List[int]) -> Dict[int, Any]:
    return {
        64: pd.littleEndian(data[0:2]),
        65: data[2],
        66: data[3],
        67: data[4] & 1,
        68: (data[4] >> 5) & 7,
        69: data[5] & 1,
        70: data[6] & 1,
        71: (data[6] >> 7) & 1,
        72: data[7] & 1,
        73: (data[7] >> 1) & 1,
        74: (data[7] >> 2) & 1
    }

def decode16(data: List[int]) -> Dict[int, Any]:
    grouped_data = [pd.littleEndian(d) for d in pd.groupBytes(data)]
    return {
        75: grouped_data[0],
        76: grouped_data[1],
        77: grouped_data[2],
        78: grouped_data[3]
    }

def decode17(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data[:4])
    timer_data = pd.littleEndian(data[4:])
    return {
        79: fd.torque(decoded_data[0]),
        80: fd.torque(decoded_data[1]),
        81: fd.timer(timer_data)
    }

def decode18(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    return {
        82: fd.torque(decoded_data[0]),
        83: fd.angularVelocity(decoded_data[1]),
        84: data[4],
        85: data[5] & 1,
        86: (data[5] >> 1) & 1,
        87: (data[5] >> 2) & 1,
        88: fd.torque(decoded_data[3])
    }

def decode19(data: List[int]) -> Dict[int, Any]:
    return {
        89: pd.littleEndian(data[0:2]),
        90: pd.littleEndian(data[2:4])
    }

def decode20(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    return {
        91: decoded_data[0],
        92: decoded_data[1],
        93: decoded_data[2]
    }

def decode21(data: List[int]) -> Dict[int, Any]:
    temp = pd.littleEndian(data[0:2])
    humid = pd.littleEndian(data[2:4])
    tempF = -49 + (315 * temp / 65535.0)
    tempC = -45 + (175 * temp / 65535.0)
    relHumid = 100 * humid / 65535.0 
    return {
        94: tempC,
        95: tempF,
        96: relHumid
    }

def decode22(data: List[int]) -> Dict[int, Any]:
    cell_id = data[0]
    instant_voltage = pd.bigEndian(data[1:3])
    internal_resistance = pd.bigEndian(data[3:5]) & 32767 # clear last bit
    shunted = (data[3] >> 7) & 1 # get last bit
    open_voltage = pd.bigEndian(data[5:7])
    return {
        97: f"{cell_id} {instant_voltage} {open_voltage} {internal_resistance} {shunted}"
    }

def decode29(data: List[int]) -> Dict[int, Any]:
    glv_current = pd.twosComp(pd.littleEndian(data), 32)
    return {
        98: glv_current / 1000000.0 # undo 10^6 scale factor from car
    }

def decode34(data: List[int]) -> Dict[int, Any]:
    voltage1 = pd.twosComp(pd.littleEndian(data[0:4]), 32)
    voltage2 = pd.twosComp(pd.littleEndian(data[4:]), 32)
    return {
        99: voltage1 / 1000000.0, # undo 10^6 scale factor from car
        100: voltage2 / 1000000.0 
    }

def decode35(data: List[int]) -> Dict[int, Any]:
    return {
        102: pd.bigEndian(data[0:2]),
        103: pd.bigEndian(data[2:4]),
        104: data[4]
    }

def decodeMPUDashboardInfo(data: List[int]) -> Dict[int, Any]:
    return {
        105: data[0]
    }


def decodeGPS1(data: List[int]) -> Dict[int, Any]:
    return {
        108: pd.twosComp(pd.littleEndian(data[0:4]), 32) / 10000000, # Longitude in degrees * 1e-7 (Get rid of multiplier)
        109: pd.twosComp(pd.littleEndian(data[4:8]), 32) / 10000000 # Latitude in degrees * 1e-7 (Get rid of multiplier)
    }

def decodeGPS2(data: List[int]) -> Dict[int, Any]:
    return {
        110: pd.twosComp(pd.littleEndian(data[0:4]), 32),
        111: pd.twosComp(pd.littleEndian(data[4:8]), 32) / 1000 # Altitude in mm (transform to m)
    }

def decodeGPS3(data: List[int]) -> Dict[int, Any]:
    return {
        112: pd.twosComp(pd.littleEndian(data[0:4]), 32) / 1000, # Ground speed in mm/sec (transform to m/s)
        113: pd.twosComp(pd.littleEndian(data[4:8]), 32) / 100000 # Heading in degrees * 1e-5 (Get rid of multiplier)
    }

def decodeCellTemps(data: List[int]) -> Dict[int, Any]:
    high_cell_temp_chip_number = (data[2] >> 0) & 15
    high_cell_temp_cell_number = (data[2] >> 4) & 15
    low_cell_temp_chip_number = (data[5] >> 0) & 15
    low_cell_temp_cell_number = (data[5] >> 4) & 15

    return {
        114: pd.bigEndian(data[0:2]),
        115: high_cell_temp_chip_number,
        116: high_cell_temp_cell_number,
        117: pd.bigEndian(data[3:5]),
        118: low_cell_temp_chip_number,
        119: low_cell_temp_cell_number,
        120: pd.bigEndian(data[6:8]),
    }

def decodeLoggingStatus(data: List[int]) -> Dict[int, Any]:
    return {
        129: data[0]
    }