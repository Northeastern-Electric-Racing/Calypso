from decode_files import LogFormat, processLine
from message import Message

FORMAT = LogFormat.TEXTUAL1


def thread(line):
    message: Message = processLine(line, FORMAT)
    return message.decode()