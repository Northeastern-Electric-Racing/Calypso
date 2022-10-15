"""
This file specifies methods to decode messages into the many pieces of data they contain.
"""

from typing import Any, Dict, List

from ner_processing.data import (
    ProcessData as pd, 
    FormatData as fd, 
)


def decodeMock(data: List[int]) -> Dict[int, Any]:
    return {
        0: 0
    }

def decode1(data: List[int]) -> Dict[int, Any]:
    return {
        1: pd.bigEndian(data[0:2]),
        2: pd.twosComp(pd.bigEndian(data[2:4])),
        3: pd.bigEndian(data[4:6]),
        4: data[6],
        5: data[7]
    }

def decode2(data: List[int]) -> Dict[int, Any]:
    return {
        6: "{:08b}".format(data[0]),
        7: "{:08b}".format(data[1]),
        8: "{:016b}".format(pd.bigEndian(data[2:4])),
        9: "{:016b}".format(pd.bigEndian(data[4:6])),
        10: pd.twosComp(data[6], 8),
        11: pd.twosComp(data[7], 8)
    }

def decode3(data: List[int]) -> Dict[int, Any]:
    return {
        12: data[0]
    }

def decode4(data: List[int]) -> Dict[int, Any]:
    return {
        13: pd.bigEndian(data[0:2]),
        14: data[2],
        15: pd.bigEndian(data[3:5]),
        16: data[5],
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
    return {
        44: fd.angle(decoded_data[0]),
        45: fd.angularVelocity(decoded_data[1]),
        46: fd.frequency(decoded_data[2]),
        47: fd.angle(decoded_data[3])
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
    decoded_data = ["{:08b}".format(d) for d in data[3:]]
    return {
        64: pd.littleEndian(data[0:2]),
        65: data[2],
        66: decoded_data[0],
        67: decoded_data[1][0],
        68: decoded_data[1][5:],
        69: decoded_data[2],
        70: decoded_data[3][0],
        71: decoded_data[3][7],
        72: decoded_data[4][0],
        73: decoded_data[4][1],
        74: decoded_data[4][2]
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
