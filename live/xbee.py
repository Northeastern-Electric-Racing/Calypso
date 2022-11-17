from PyQt6.QtSerialPort import QSerialPort, QSerialPortInfo
from PyQt6.QtCore import QIODeviceBase, QDateTime

from ner_processing.message import Message, MessageFormatException
from ner_processing.live.live_input import LiveInput, LiveInputException, InputState
from ner_telhub.model.message_models import MessageModel


class XBee(LiveInput):
    """
    A class to represent an XBee wireless module as a live input.
    """

    START_TOKEN = "T"
    END_TOKEN = "\r"

    def __init__(self, model: MessageModel):
        """
        Initialize the serial port and message handling variables.
        """
        super().__init__(model)
        self._reset()

    def _reset(self) -> None:
        """
        Resets this XBee.
        """
        self.port = QSerialPort()
        self.port.setBaudRate(QSerialPort.BaudRate.Baud115200.value)
        self.port.setFlowControl(QSerialPort.FlowControl.HardwareControl)
        self.port.readyRead.connect(self._handle_read)
        self.message_started = False
        self.current_message = ""
        self.state = InputState.NONE

    def _validateState(self, desired_state: InputState) -> None:
        """
        Validates states and throws appropriate error messages.
        """
        if desired_state == InputState.NONE and self.state != InputState.NONE:
                raise LiveInputException("XBee is already connected.")
        elif desired_state == InputState.CONNECTED:
            if self.state == InputState.NONE:
                raise LiveInputException("XBee is not yet connected.")
            elif self.state == InputState.STARTED:
                raise LiveInputException("XBee has already started.")
        elif desired_state == InputState.STARTED:
            if self.state == InputState.NONE:
                raise LiveInputException("XBee is not yet connected.")
            elif self.state == InputState.CONNECTED:
                raise LiveInputException("XBee has not yet been started.")

    def connect(self, *args, **kwargs) -> None:
        """
        Overrides LiveInput.connect()
        """
        self._validateState(InputState.NONE)

        if len(args) != 1:
            raise TypeError("Missing port name argument.")
        if not isinstance(args[0], str):
            raise TypeError("Invalid input type for port name.")
        port_name = args[0]
 
        for port in QSerialPortInfo.availablePorts():
            if port.portName() == port_name:
                try:
                    self.port.setPort(port)
                    self.state = InputState.CONNECTED
                    return
                except Exception as e:
                    raise LiveInputException("Error while connecting to desired port")
        raise LiveInputException("Invalid port name")

    def disconnect(self, *args, **kwargs) -> None:
        """
        Overrides LiveInput.disconnect()
        """
        self._validateState(InputState.CONNECTED)
        self._reset()

    def start(self, *args, **kwargs) -> None:
        """
        Overrides LiveInput.start()
        """
        self._validateState(InputState.CONNECTED)
        self.port.open(QIODeviceBase.OpenModeFlag.ReadOnly)
        self.state = InputState.STARTED

    def stop(self) -> None:
        """
        Overrides LiveInput.stop()
        """
        self._validateState(InputState.STARTED)
        self.port.close()
        self.state = InputState.CONNECTED

    def parse(self, message: str) -> Message:
        """
        Overrides LiveInput.parse()
        """
        try:
            timestamp = QDateTime.fromMSecsSinceEpoch(int(message[0:13]))
            id = int(message[13:16], base=16)
            length = int(message[16])
            data = []
            for i in range(length):
                data.append(int(message[(17 + 2*i):(19 + 2*i)], base=16))
            return Message(timestamp, id, data)
        except:
            raise MessageFormatException("Error with message fields")

    def _handle_read(self):
        """
        Handles the reading of Xbee data.
        """
        try:
            buf = self.port.readAll()
            msgs = buf.data().decode()
        except:
            print("Error with receiving message")
            return

        for char in msgs:
            if char == self.START_TOKEN:
                self.message_started = True
                self.current_message = ""
            elif char == self.END_TOKEN:
                try:
                    msg = self.parse(self.current_message)
                    if msg is not None:
                        self._model.addMessage(self.parse(self.current_message))
                except MessageFormatException as e:
                    print(e.message)
                except RuntimeError:
                    self.stop()
                self.message_started = False
                self.current_message = ""
            elif self.message_started:
                self.current_message += char
                