"""
This file specifies methods to decode message fields (timestamp, id, data bytes) from 
a line in a log file. 
"""

from datetime import datetime
from enum import Enum
from typing import Tuple, List

class LogFormat(Enum):
    TEXTUAL1 = 1
    TEXTUAL2 = 2
    BINARY = 3


def process_line(line: str, format: LogFormat) -> Tuple[datetime, int, int, List[int]]:
    if format == LogFormat.TEXTUAL1:
        return process_textual1_line(line)
    elif format == LogFormat.TEXTUAL2:
        return process_textual2_line(line)
    elif format == LogFormat.BINARY:
        return process_binary_log(line)
    else:
        raise ValueError("Invalid file format.")


def process_textual1_line(line: str) -> Tuple[datetime, int, int, List[int]]:
    """Processes a line of data in the format "Timestamp id length [data1,data2,...]"
    Example line format: 2021-01-01T00:00:00.003Z 514 8 [54,0,10,0,0,0,0,0]
    """

    fields = line.strip().split(" ")
    timestamp = datetime.strptime(fields[0], "%Y-%m-%dT%H:%M:%S.%fZ")
    id = int(fields[1])
    length = int(fields[2])
    data = fields[3][1:-1].split(",") # remove commas and brackets at start and end
    int_data = [int(x) for x in data]
    return timestamp, id, length, int_data


def process_textual2_line(line: str) -> Tuple[datetime, int, int, List[int]]:
    """Processes a line of data in the format "Timestamp id length data1 data2 ..."
    Example line format: 1659901910.121 514 8 54 0 10 0 0 0 0 0
    """

    fields = line.strip().split(" ")
    timestamp = datetime.fromtimestamp(float(fields[0]))
    id = int(fields[1])
    length = int(fields[2])
    data = [int(x) for x in fields[3:3+length]]
    return timestamp, id, length, data


def process_binary_log(line: str) -> Tuple[datetime, int, int, List[int]]:
    raise RuntimeError("Binary files not currently supported.")


