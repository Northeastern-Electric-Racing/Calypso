from typing import Any

from python.data import Data


# Mapping from data IDs to the status bits they encode
# Each data ID contains a dict with keys that are bit names and values that are the indexes
STATUS_MAP = {
    # 6: { # Failsafe Statuses
    # },
    # 7: { # DTC Status 1
    # },
    # 8: { # DTC Status 2
    # },
    # 9: { # Current Limits Status
    # },
    # 12: { # MPE State
    # },
    64: { # VSM State
        "VSM Start State": 0,
        "Pre-charge Init State": 1,
        "Pre-charge Active State": 2,
        "Pre-charge Complete State": 3,
        "VSM Wait State": 4,
        "VSM Ready State": 5,
        "Motor Running State": 6,
        "Blink Fault Code State": 7
    },
    65: { # Inverter State
        "Power on State": 0,
        "Stop State": 1,
        "Open Loop State": 2,
        "Closed Loop State": 3,
        "Wait State": 4,
        "Idle Run State": 8,
        "Idle Stop State": 9
    },
    66: { # Relay State
        "Relay 1 Status": 0,
        "Relay 2 Status": 1,
        "Relay 3 Status": 2,
        "Relay 4 Status": 3,
        "Relay 5 Status": 4,
        "Relay 6 Status": 5,
    },
    67: { # Inverter Run Mode
        "Inverter Run Mode": 0
    },
    69: { # Inverter Command Mode
        "Inverter Command Mode": 0
    },
    70: { # Inverter Enable State
        "Inverter Enable State": 0
    },
    71: { # Inverter Enable Lockout
        "Inverter Enable Lockout": 0
    },
    72: { # Direction Command
        "Direction Command": 0
    },
    73: { # BMS Active
        "BMS Active": 0
    },
    74: { # BMS Limiting Torque
        "BMS Limiting Torque": 0
    },
    75: { # POST Fault Lo
        "Hardware Gate/Desaturation Fault": 0,
        "HW Over-current Fault": 1,
        "Accelerator Shorted": 2,
        "Accelerator Open": 3,
        "Current Sensor Low": 4,
        "Current Sensor High": 5,
        "Module Temperature Low": 6,
        "Module Temperature High": 7,
        "Control PCB Temperature Low": 8,
        "Control PCB Temperature High": 9,
        "Gate Drive PCB Temperature Low": 10,
        "Gate Drive PCB Temperature High": 11,
        "5V Sense Voltage Low": 12,
        "5V Sense Voltage High": 13,
        "12V Sense Voltage Low": 14,
        "12V Sense Voltage High": 15
    },
    76: { # POST Fault Hi
        "2.5V Sense Voltage Low": 0,
        "2.5V Sense Voltage High": 1,
        "1.5V Sense Voltage Low": 2,
        "1.5V Sense Voltage High": 3,
        "DC Bus Voltage High": 4,
        "DC Bus Voltage Low": 5,
        "Pre-charge Timeout": 6,
        "Pre-charge Voltage Failure": 7,
        "Brake Shorted": 14,
        "Brake Open": 15
    },
    77: { # Run Fault Lo
        "Motor Over-speed Fault": 0,
        "Over-current Fault": 1,
        "Over-voltage Fault": 2,
        "Inverter Over-temperature Fault": 3,
        "Accelerator Input Shorted Fault": 4,
        "Accelerator Input Open Fault": 5,
        "Direction Command Fault": 6,
        "Inverter Response Time-out Fault": 7,
        "Hardware Gate/Desaturation Fault": 8,
        "Hardware Over-current Fault": 9,
        "Under-voltage Fault": 10,
        "CAN Command Message Lost Fault": 11,
        "Motor Over-temperature Fault": 12
    },
    78: { # Run Fault Hi
        "Brake Input Shorted Fault": 0,
        "Brake Input Open Fault": 1,
        "Module A Over-temperature Fault": 2,
        "Module B Over temperature Fault": 3,
        "Module C Over-temperature Fault": 4,
        "PCB Over-temperature Fault": 5,
        "Gate Drive Board 1 Over-temperature Fault": 6,
        "Gate Drive Board 2 Over-temperature Fault": 7,
        "Gate Drive Board 3 Over-temperature Fault": 8,
        "Current Sensor Fault": 9,
        "Hardware Over-Voltage Fault": 11,
        "Resolver Not Connected": 14,
        "Inverter Discharge Active": 15
    },
    84: { # Direction Command
        "Direction Command": 0
    },
    85: { # Inverter Enable
        "Inverter Enable": 0
    },
    86: { # Inverter Discharge
        "Inverter Discharge": 0
    },
    87: { # Speed Mode Enable
        "Speed Mode Enable": 0
    },
    # 97: { # Cell Voltage Info
    # }
    107: { # BMS Faults
        "Cells Not Balancing": 0,
        "Cell Voltage Too High": 1,
        "Cell Voltage Too Low": 2,
        "Pack Too High": 3,
        "Open Wiring Fault": 4,
        "Internal Software Fault": 5,
        "Internal Thermal Fault": 6,
        "Internal Cell Comm Fault": 7,
        "Current Sensor Fault": 8,
        "Charge Reading Mismatch": 9,
        "Low Cell Voltage": 10,
        "Weak Pack Fault": 11,
        "External Can Fault": 12,
        "Discharge Limit Enforcement Fault": 13,
        "Charger Safety Relay": 14,
        "Battery Can Fault": 15,
        "Charger Can Fault": 16,
        "Charge Limit Enforcement Fault": 17
    },
}


def getStatus(data_id: int, data_value: Any, name: str) -> int:
    """
    Gets the specified status of the given data piece. Returns a bit value.
    """
    if data_id not in STATUS_MAP:
        raise KeyError("Data ID has no associated status mapping")
    bitmap = STATUS_MAP[data_id]

    if name not in bitmap:
        raise KeyError("Status name could not be found in the given data point")
    index = bitmap[name]

    return (data_value >> index) & 1


def getStatuses(data_id: int, data_value: Any) -> Any:
    """
    Gets all the statuses for the given data piece.
    """
    if data_id not in STATUS_MAP:
        raise KeyError("Data ID has no associated status mapping")
    bitmap = STATUS_MAP[data_id]

    # Convert each dict value to the bit value at the index
    return {name:(data_value >> index) & 1 for (name, index) in bitmap.items()}