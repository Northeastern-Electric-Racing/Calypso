from datetime import datetime
from typing import List, Any


class Data:
    def __init__(self, timestamp: datetime, id: int, value: Any):
        self.timestamp = timestamp
        self.id = id
        self.value = value

    def __str__(self):
        """Overrides the string representation of the class."""
        return f"ID {self.id} - {self.timestamp} - {self.value}"

    


class ProcessData:
    """Utility functions to process message data."""

    @staticmethod
    def group_bytes(data_bytes: List[int], group_length: int = 2) -> List[List[int]]:
        """Splits the given data bytes into lists of specified length."""
        return [data_bytes[i : i + group_length] for i in range(0, len(data_bytes), group_length)]

    @staticmethod
    def twos_comp(val: int, bits: int = 16) -> int:
        """Computes the twos complement of the given value."""

        if (val & (1 << (bits - 1))) != 0:
            val = val - (1 << bits)
        return val

    @staticmethod
    def little_endian(data_bytes: List[int], bits: int = 8) -> int:
        """Transforms the given data bytes into a value in little endian.
        Little Endian byte order stores low order bytes first."""

        result = 0
        for i in range(len(data_bytes)):
            result |= data_bytes[i] << (bits * i)
        return result

    @staticmethod
    def big_endian(data_bytes: List[int], bits: int = 8) -> int:
        """Transforms the given data bytes into a value in big endian.
        Big Endian byte order stores low order bytes last."""

        result = 0
        for i in range(len(data_bytes)):
            result |= data_bytes[i] << (bits * (len(data_bytes) - i - 1))
        return result

    @staticmethod
    def default_decode(byte_vals: List[int]) -> List[int]:
        """Default decode structure seen by a majority of the messages."""

        grouped_vals = ProcessData.group_bytes(byte_vals)
        parsed_vals = [ProcessData.little_endian(val) for val in grouped_vals]
        decoded_vals = [ProcessData.twos_comp(val) for val in parsed_vals]
        return decoded_vals


class FormatData:
    """Utility functions to scale data values of a specific type."""

    @staticmethod
    def temperature(value):
        return value / 10

    @staticmethod
    def torque(value):
        return value / 10

    @staticmethod
    def angle(value):
        return value / 10

    @staticmethod
    def frequency(value):
        return value / 10

    @staticmethod
    def angular_velocity(value):
        return value

    @staticmethod
    def current(value):
        return value / 10

    @staticmethod
    def high_voltage(value):
        return value / 10

    @staticmethod
    def timer(value):
        return value * 0.003