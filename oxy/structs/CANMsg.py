from __future__ import annotations
from .CANField import CANField
from dataclasses import dataclass
from .NetworkEncoding import NetworkEncoding
from ruamel.yaml import Optional

@dataclass
class CANMsg:
    '''
    Represents a CAN message. Has an id, a description, and a number of individual fields.
    '''
    id: str
    desc: str
    networkEncoding: list[NetworkEncoding]

    def __post_init__(self) -> None:
        idx: int = 0
        for field in self.networkEncoding[0].fields:
            if (field.index is not None):
                field.index = idx
                idx += field.size

    def __setstate__(self, state):
        self.__init__(**state)
