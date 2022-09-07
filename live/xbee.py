from PyQt6.QtSerialPort import QSerialPort, QSerialPortInfo

from PyQt6.QtCore import QIODeviceBase, QDateTime

from ner_processing.message import Message, MessageFormatException
from ner_processing.live.live_input import LiveInput


class XBeeException(Exception):
    def __init__(self, message):
        self.message = message


class XBee(LiveInput):
    """
    A class to represent an XBee wireless module.
    """

    start_token = "s"

    def __init__(self, model):
        self.port = QSerialPort()
        self.model = model
        self.port.readyRead.connect(self._handle_read)
        self.connected = False
        self.message_started = False
        self.current_message = ""

    def start(self, port_name):
        """
        Starts a connection to the given serial port.
        """
        if self.connected:
            raise XBeeException("XBee is already connected")
        
        # Attempt to connect to available ports if no connection is valid
        for avail in QSerialPortInfo.availablePorts():
            print("Trying port: ", avail.portName())
            if avail.portName() == port_name:
                try:
                    self.port.setPort(avail)
                    self.port.open(QIODeviceBase.OpenModeFlag.ReadWrite)
                    self.port.write("sCONNECT".encode())
                    # TODO: Add more code for connection message/handling
                    self.connected = True
                    return
                except Exception as e:
                    raise XBeeException("Error while connecting to desired port")
                    
        raise XBeeException("Invalid port name")

    def stop(self):
        """
        Stops the current connection.
        """
        if not self.connected:
            raise XBeeException("XBee is not connected")
        
        self.port.close()
        self.connected = False

    @staticmethod
    def parse_message(msg):
        """Parses a string message into its fields.

        Parameters
        ----------
        msg : str
            Message to parse, in the form: "timestamp id length data1 data2 ..."

        Returns
        -------
        Message
            Message object formed from the parsed fields

        Raises
        ------
        MessageFormatException
            If the string cannot be parsed correctly
        """

        fields = msg.strip().split(" ")
        if len(fields) < 3:
            raise MessageFormatException("Not enough fields in the message", 0)
        
        try:
            timestamp = QDateTime.fromMSecsSinceEpoch(int(float(fields[0])*1000))
            id = int(fields[1])
            length = int(fields[2])  # get the data length to use for processing data array
            data = []

            if len(fields) < (3 + length):
                raise MessageFormatException("Too few data bytes", 0)
            elif len(fields) > (3 + length):
                raise MessageFormatException("Too many data bytes", 2)

            for i in range(length):
                data.append(int(fields[i + 3]))

            return Message(timestamp, id, data)
        except ValueError:
            raise MessageFormatException("Message has malformed fields", 1)

    @staticmethod
    def port_info():
        return [(p.portName(), p.description()) for p in QSerialPortInfo.availablePorts()]

    def _handle_read(self):
        try:
            buf = self.port.readAll()
            msgs = buf.data().decode()
        except:
            return
        print("-----------")
        print(msgs)
        msgs = msgs.split(self.start_token) # split messages received in same bunch

        # Discard a message if we haven't received the start token
        if not self.message_started and len(msgs[0]) > 0 and msgs[0][0] != "s":
            msgs.pop(0)

        # Handle all messages
        for i in range(len(msgs)):
            # Only append the msg if its the first and one has been started
            if i == 0 and self.message_started:
                self.current_message += msgs[i]
            else:
                self.message_started = True
                self.current_message = msgs[i]

            # Try to parse the current message and add to model
            try:
                print(self.current_message)
                msg = self.parse_message(self.current_message)
                self.model.addMessage(msg)
                self.message_started = False
            except MessageFormatException as mfe:
                # Status of 0 means more data is needed, other means an error
                if mfe.status != 0:
                    print(mfe.message)
                    self.message_started = False

    def send_data(self, data):
        # if self._ser.is_open == False:
        #     raise XBeeException("Serial port is not open")
        # try:
        #     self._ser.write(data.encode('utf-8'))
        # except SerialTimeoutException as ste:
        #     raise XBeeException("Error writing the data")
        pass # TODO: implement

