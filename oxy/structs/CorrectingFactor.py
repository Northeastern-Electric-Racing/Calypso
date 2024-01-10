from dataclasses import dataclass

@dataclass
class CorrectingFactor:
    '''
    Represents a correcting factor to be applied to data after decoding.
    '''
    const: float
    op: str
