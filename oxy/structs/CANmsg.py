from __future__ import annotations
from ruamel.yaml import Optional, MappingNode
from structs.CorrectingFactor import CorrectingFactor
from .CANField import CANField
from dataclasses import dataclass

@dataclass
class CANMsg:
    '''
    Represents a CAN message. Has an id, a description, and a number of individual fields.
    '''
    id: int
    desc: str
    fields: list[CANField]

    def __post_init__(self) -> None:
        idx: int = 0
        for field in self.fields:
            field.index = idx
            idx += field.size


    def __setstate__(self, state):
        self.__init__(**state)
