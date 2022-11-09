from ner_processing.decode_files import LogFormat, processLine
from ner_processing.message import Message

FORMAT = LogFormat.TEXTUAL1


def thread(line):
    message: Message = processLine(line, FORMAT)
    return message.decode()