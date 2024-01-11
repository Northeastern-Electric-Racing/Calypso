from __future__ import annotations
from .Decoding import *
from ruamel.yaml import Optional
from dataclasses import dataclass
from .Format import Format

@dataclass
class CANField:
    '''
    Represents a field in a CAN message. Has an id, a name, units, a size,
    and an optional CorrectingFactor and Decodings. Also knows its own
    index within its parent CANMsg, which is assigned at load from YAML.
    '''
    id: int
    name: str
    units: str
    size: int
    index: int = -1
    decodings: Optional[list[Decoding]] = None
    format: Optional[str] = None
