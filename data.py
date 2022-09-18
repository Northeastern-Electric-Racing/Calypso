from typing import List, Any
from PyQt6.QtCore import QDateTime


class Data:
    """
    Wrapper class for an individual piece of data.
    """

    def __init__(self, timestamp: QDateTime, id: int, value: Any):
        self.timestamp = timestamp
        self.id = id
        self.value = value

    def __str__(self):
        """
        Overrides the string representation of the class.
        """
        return f"ID {self.id} - {self.timestamp} - {self.value}"

    
class ProcessData:
    """
    Utility functions to process message data.
    """

    @staticmethod
    def groupBytes(data_bytes: List[int], group_length: int = 2) -> List[List[int]]:
        """
        Splits the given data bytes into lists of specified length.
        """
        return [data_bytes[i : i + group_length] for i in range(0, len(data_bytes), group_length)]

    @staticmethod
    def twosComp(val: int, bits: int = 16) -> int:
        """
        Computes the twos complement of the given value.
        """
        if (val & (1 << (bits - 1))) != 0:
            val = val - (1 << bits)
        return val

    @staticmethod
    def littleEndian(data_bytes: List[int], bits: int = 8) -> int:
        """
        Transforms the given data bytes into a value in little endian.
        Little Endian byte order stores low order bytes first.
        """
        result = 0
        for i in range(len(data_bytes)):
            result |= data_bytes[i] << (bits * i)
        return result

    @staticmethod
    def bigEndian(data_bytes: List[int], bits: int = 8) -> int:
        """
        Transforms the given data bytes into a value in big endian.
        Big Endian byte order stores low order bytes last.
        """
        result = 0
        for i in range(len(data_bytes)):
            result |= data_bytes[i] << (bits * (len(data_bytes) - i - 1))
        return result

    @staticmethod
    def defaultDecode(byte_vals: List[int]) -> List[int]:
        """
        Default decode structure seen by a majority of the messages.
        """
        grouped_vals = ProcessData.groupBytes(byte_vals)
        parsed_vals = [ProcessData.littleEndian(val) for val in grouped_vals]
        decoded_vals = [ProcessData.twosComp(val) for val in parsed_vals]
        return decoded_vals


class FormatData:
    """
    Utility functions to scale data values of a specific type.
    """

    @staticmethod
    def temperature(value):
        return value / 10

    @staticmethod
    def lowVoltage(value):
        return value / 100

    @staticmethod
    def torque(value):
        return value / 10

    @staticmethod
    def highVoltage(value):
        return value / 10

    @staticmethod
    def current(value):
        return value / 10

    @staticmethod
    def angle(value):
        return value / 10

    @staticmethod
    def angularVelocity(value):
        return -value

    @staticmethod
    def frequency(value):
        return value / 10

    @staticmethod
    def power(value):
        return value / 10

    @staticmethod
    def timer(value):
        return value * 0.003

    @staticmethod
    def flux(value):
        return value / 1000
