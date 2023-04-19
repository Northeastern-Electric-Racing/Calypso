from python.decode_files import LogFormat, processLine
from python.message import Message

FORMAT = LogFormat.TEXTUAL1


def thread(line):
    message: Message = processLine(line, FORMAT)
    return message.decode()