"""
This file specifies methods to decode message fields (timestamp, id, data bytes) from 
a line in a log file. 
"""

from enum import Enum
from PyQt6.QtCore import QDateTime

from ner_processing.message import Message


class LogFormat(Enum):
    TEXTUAL1 = 1
    TEXTUAL2 = 2
    BINARY = 3


def processLine(line: str, format: LogFormat) -> Message:
    """
    Processes a line of textual data according to a given format.
    """
    if format == LogFormat.TEXTUAL1:
        return _processTextual1(line)
    elif format == LogFormat.TEXTUAL2:
        return _processTextual2(line)
    elif format == LogFormat.BINARY:
        return _processBinary(line)
    else:
        raise ValueError("Invalid file format.")


def _processTextual1(line: str) -> Message:
    """
    Processes a line of data in the format "Timestamp id length [data1,data2,...]"
    Example line format: 2021-01-01T00:00:00.003Z 514 8 [54,0,10,0,0,0,0,0]
    """
    fields = line.strip().split(" ")
    timestamp = QDateTime.fromString(fields[0], "yyyy-MM-ddTHH:mm:ss.zzzZ")
    id = int(fields[1])
    length = int(fields[2])
    data = fields[3][1:-1].split(",") # remove commas and brackets at start and end
    int_data = [int(x) for x in data]
    return Message(timestamp, id, int_data)


def _processTextual2(line: str) -> Message:
    """
    Processes a line of data in the format "Timestamp id length data1 data2 ..."
    Example line format: 1659901910.121 514 8 54 0 10 0 0 0 0 0
    """
    fields = line.strip().split(" ")
    timestamp = QDateTime.fromMSecsSinceEpoch(int(float(fields[0])*1000))
    id = int(fields[1])
    length = int(fields[2])
    data = [int(x) for x in fields[3:3+length]]
    return Message(timestamp, id, data)


def _processBinary(line: str) -> Message:
    raise RuntimeError("Binary files not currently supported.")


