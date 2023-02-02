from typing import List, Dict, Any

from PyQt6.QtCore import QDateTime

from ner_processing.data import Data
from ner_processing.master_mapping import MESSAGE_IDS


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

    def __init__(self, timestamp: QDateTime, id: int, data: List[int]):
        self.timestamp = timestamp
        self.id = id
        self.data = data

    def __str__(self):
        """
        Overrides the string representation of the class.
        """
        return f"[{self.timestamp.toString('yyyy-MM-ddTHH:mm:ss.zzzZ')}] {self.id} - {self.data}"

    def decode(self) -> List[Data]:
        """
        Processes this message's data into a list of data points.
        """
        return self.decodeMessage(self.timestamp, self.id, self.data)

    @staticmethod
    def decodeMessage(timestamp: QDateTime, id: int, data: List[int]) -> List[Data]:
        """
        Decodes the given message fields into their data points
        """
        try:
            decoded_data: Dict[int, Any] = MESSAGE_IDS[id]["decoder"](data)
        except:
            raise MessageFormatException(f"Invalid data format for can id {id}")
        
        return [Data(timestamp, data_id, decoded_data[data_id]) for data_id in decoded_data]