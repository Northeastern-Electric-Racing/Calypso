from __future__ import annotations
from .CANField import CANField
from dataclasses import dataclass

@dataclass
class CANMsg:
    '''
    Represents a CAN message. Has an id, a description, and a number of individual fields.
    '''
    id: str
    desc: str
    fields: list[CANField]

    def __post_init__(self) -> None:
        idx: int = 0
        for field in self.fields:
            if (field.index is not None):
                field.index = idx
                idx += field.size


    def __setstate__(self, state):
        self.__init__(**state)
