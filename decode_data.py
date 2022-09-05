"""
This file specifies methods to decode messages into the many pieces of data they contain.
"""

from typing import Any, Dict, List

from ner_processing.data import (
    ProcessData as pd, 
    FormatData as fd, 
)


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

def decode8(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    return {
        30: fd.angle(decoded_data[0]),
        31: fd.angularVelocity(decoded_data[1]),
        32: fd.frequency(decoded_data[2]),
        33: fd.angle(decoded_data[3])
    }

def decode9(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.current(d) for d in decoded_data]
    return {
        34: final_data[0],
        35: final_data[1],
        36: final_data[2],
        37: final_data[3]
    }

def decode10(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    final_data = [fd.high_voltage(d) for d in decoded_data]
    return {
        38: final_data[0],
        39: final_data[1],
        40: final_data[2],
        41: final_data[3]
    }

def decode11(data: List[int]) -> Dict[int, Any]:
    decoded_data = ["{:08b}".format(d) for d in data[3:]]
    return {
        42: pd.littleEndian(data[0:2]),
        43: data[2],
        44: decoded_data[0],
        45: decoded_data[1][0],
        46: decoded_data[1][5:],
        47: decoded_data[2],
        48: decoded_data[3][0],
        49: decoded_data[3][7],
        50: decoded_data[4][0],
        51: decoded_data[4][1],
        52: decoded_data[4][2]
    }

def decode12(data: List[int]) -> Dict[int, Any]:
    grouped_data = [pd.littleEndian(d) for d in pd.groupBytes(data)]
    return {
        53: grouped_data[0],
        54: grouped_data[1],
        55: grouped_data[2],
        56: grouped_data[3]
    }

def decode13(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data[:4])
    timer_data = pd.littleEndian(data[4:])
    return {
        57: fd.torque(decoded_data[0]),
        58: fd.torque(decoded_data[1]),
        59: fd.timer(timer_data)
    }

def decode14(data: List[int]) -> Dict[int, Any]:
    return {
        60: pd.littleEndian(data[0:2]),
        61: pd.littleEndian(data[2:4])
    }

def decode15(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)
    return {
        62: decoded_data[0],
        63: decoded_data[1],
        64: decoded_data[2]
    }

def decode16(data: List[int]) -> Dict[int, Any]:
    temp = pd.littleEndian(data[0:2])
    humid = pd.littleEndian(data[2:4])
    tempF = -49 + (315 * temp / 65535.0)
    tempC = -45 + (175 * temp / 65535.0)
    relHumid = 100 * humid / 65535.0 
    return {
        65: tempC,
        66: tempF,
        67: relHumid
    }

def decode17(data: List[int]) -> Dict[int, Any]:
    decoded_data = pd.defaultDecode(data)

    return {
        68: fd.torque(decoded_data[0]),
        69: fd.angularVelocity(decoded_data[1]),
        70: data[4],
        71: data[5] & 1,
        72: (data[5] >> 1) & 1,
        73: (data[5] >> 2) & 1,
        74: fd.torque(decoded_data[3])
    }