from io import TextIOWrapper
from ruamel.yaml import YAML, Any

from structs.CANmsg import CANmsg
from structs.CANField import CANField
from structs.CorrectingFactor import CorrectingFactor
import structs.Decoding

class YAMLParser:
    '''
    A class to parse a given YAML string or file. Most of the heavy lifting
    is done by the internals of ruamel.yaml.
    '''

    def __init__(self):
        self.yaml = YAML()
        self.yaml.register_class(CANmsg)
        self.yaml.register_class(CANField)
        self.yaml.register_class(CorrectingFactor)
        for decoding in structs.Decoding.Decoding.__subclasses__():
            self.yaml.register_class(decoding)


    def parse(self, file: Any) -> CANmsg:
        return self.yaml.load(file)
