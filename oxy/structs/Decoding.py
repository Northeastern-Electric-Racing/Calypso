from dataclasses import dataclass

@dataclass
class Decoding:
    '''
    This is an abstract class (well, what passes for one in Python)
    that represents a decoding to be applied to a slice of data.
    '''
    bits: int
    final_type: str
    repr: str = "*"*42

@dataclass
class BigEndian(Decoding):
    repr: str = "big_endian"

@dataclass
class LittleEndian(Decoding):
    repr: str = "little_endian"

@dataclass
class TwosComplement(Decoding):
    repr: str = "twos_comp"
