from typing import List, Dict, Any
from datetime import datetime

from python.data import Data
from python.master_mapping import MESSAGE_IDS


class MessageFormatException(Exception):
    """
    A class to represent exceptions related to invalid message formats.
    """

    def __init__(self, message: str):
        self.message = message


class Message:
    """
    Wrapper class for an individual message.
    """

    def __init__(self, timestamp: datetime, id: int, data: List[int]):
        self.timestamp = timestamp
        self.id = id
        self.data = data

    def __str__(self):
        """
        Overrides the string representation of the class.
        """
        return f"[{self.timestamp.strftime('%Y-%m-%dT%H:%M:%S.%fZ')}] {self.id} - {self.data}"

    def decode(self) -> List[Data]:
        """
        Processes this message's data into a list of data points.
        """
        return self.decodeMessage(self.timestamp, self.id, self.data)

    @staticmethod
    def decodeMessage(timestamp: datetime, id: int, data: List[int]) -> List[Data]:
        """
        Decodes the given message fields into their data points
        """
        try:
            decoded_data: Dict[int, Any] = MESSAGE_IDS[id]["decoder"](data)
        except:
            raise MessageFormatException(f"Invalid data format for can id {id}")
        
        return [Data(timestamp, data_id, decoded_data[data_id]) for data_id in decoded_data]