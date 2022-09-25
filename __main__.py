from os import listdir, path
import csv
from typing import List

from ner_processing.master_mapping import DATA_IDS
from ner_processing.decode_files import LogFormat, processLine
from ner_processing.message import Message
from ner_processing.data import Data

LOGS_DIRECTORY = "./logs/"
OUTPUT_PATH = "./output.csv"
FORMAT = LogFormat.TEXTUAL1


def getLineCount(filepaths: List[str]) -> int:
    """
    Gets the total line count of all the files in the list.
    
    There is no native way to get line counts of files without looping, so 
    this function gets the total size and estimates the line count based
    on a subset of N lines.
    """
    if len(filepaths) == 0:
        return 0

    N = 20
    tested_lines = 0
    tested_size = 0
    total_size = sum(path.getsize(fp) for fp in filepaths)

    for fp in filepaths:
        with open(fp) as file:
            for line in file:
                tested_lines += 1
                tested_size += len(line)
                if tested_lines >= N:
                    return int(total_size / (tested_size / tested_lines))
    return int(total_size / (tested_size / tested_lines))


if __name__ == "__main__":
    line_count = getLineCount([LOGS_DIRECTORY + name for name in listdir(LOGS_DIRECTORY)])
    current_line = 0
    print(f"Processing a total of {line_count} lines")

    processed_data: List[Data] = []
    for fp in listdir(LOGS_DIRECTORY):
        with open(LOGS_DIRECTORY + fp) as file:
            line_num = 0
            for line in file:
                line_num += 1
                current_line += 1
                if current_line % 5000 == 0:
                    print(f"Line {line_num}")
                try:
                    message: Message = processLine(line, FORMAT)
                    processed_data.extend(message.decode())
                except:
                    print(f"Error with line {line_num} in file {file.name}")
                    pass

    print(f"Writing to the CSV")
    header = ["time", "data_id", "description", "value"]
    with open(OUTPUT_PATH, "w", encoding="UTF8", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(header)
        for data in processed_data:
            str_time = data.timestamp.toString("yyyy-MM-ddTHH:mm:ss.zzzZ")
            writer.writerow([str_time, data.id, DATA_IDS[data.id], data.value])




    

    
