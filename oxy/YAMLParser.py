from ruamel.yaml import YAML, Any

from structs.CANMsg import CANMsg
from structs.CANField import CANField
from structs.Format import Format
from structs.Decoding import Decoding
from structs.Messages import Messages

class YAMLParser:
    '''
    A class to parse a given YAML string or file. Most of the heavy lifting
    is done by the internals of ruamel.yaml.
    '''

    def __init__(self):
        self.yaml = YAML()
        self.yaml.register_class(Messages)
        self.yaml.register_class(CANMsg)
        self.yaml.register_class(CANField)
        for decoding in Decoding.__subclasses__():
            self.yaml.register_class(decoding)


    def parse(self, file: Any) -> CANMsg:
        return self.yaml.load(file)