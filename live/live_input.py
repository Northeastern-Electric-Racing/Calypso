from typing import Callable, List, Tuple
from enum import Enum

from PyQt6.QtSerialPort import QSerialPortInfo

from ner_processing.message import Message
from ner_telhub.model.message_models import MessageModel


class LiveInputException(Exception):
    """
    Defines an exception related to live input issues.
    """
    def __init__(self, message):
        self.message = message


class InputState(Enum):
    """
    State enum for connections.
    """
    NONE = 0
    CONNECTED = 1
    STARTED = 2


class LiveInput():
    """
    Parent class representing a live input that can be connected to.
    """

    def __init__(self, model: MessageModel):
        self._model = model
        self._callbacks = {}

    def addCallback(self, name: str, callback: Callable[[Message], None]) -> None:
        """
        Adds a callback to the data source. The callback is a data consumer, taking 
        in the processed message with no return value.
        """
        if name in self._callbacks.keys():
            raise ValueError("Already contain a callback with that name.")
        self._callbacks[name] = callback

    def removeCallback(self, name: str) -> None:
        """
        Removes a callback from the data source by its name key.
        """
        if name not in self._callbacks.keys():
            raise ValueError("No callback found with that name.")
        self._callbacks.pop(name)

    def connect(self, *args, **kwargs) -> None:
        """
        Connects to the data source.
        NOTE: Implemented in subclasses.
        """
        raise NotImplementedError("'connect' is not implemented")

    def disconnect(self, *args, **kwargs) -> None:
        """
        Disconnects from the data source.
        NOTE: Implemented in subclasses.
        """
        raise NotImplementedError("'disconnect' is not implemented")

    def start(self, *args, **kwargs) -> None:
        """
        Starts receiving data from the input.
        NOTE: Implemented in subclasses.
        """
        raise NotImplementedError("'start' is not implemented")

    def stop(self, *args, **kwargs) -> None:
        """
        Stops receiving data from the input.
        NOTE: Implemented in subclasses.
        """
        raise NotImplementedError("'stop' is not implemented")

    def parse(self, message: str) -> Message:
        """
        Processes a string of received data. 
        NOTE: Implemented in subclasses.
        """
        raise NotImplementedError("'parse' is not implemented")

    @staticmethod
    def serialPorts() -> List[Tuple[str, str]]:
        """
        Gets information on the computers serial ports.
        """
        return [(p.portName(), p.description()) for p in QSerialPortInfo.availablePorts()]


