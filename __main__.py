import multiprocessing
from datetime import datetime

from os import listdir, path
import csv
from typing import List

from ner_processing.master_mapping import DATA_IDS
from ner_processing.data import Data
from ner_processing.thread import thread

LOGS_DIRECTORY = "./logs/"
OUTPUT_PATH = "./output.csv"


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


def find_time(start, finish):
    """
    Prints the difference between the two times provided
    Both inputs are lists of strings:
        - minutes being the zeroth index of the list
        - seconds being the first index of the list
        - microseconds being the second index of the list
    """

    minutes = int(finish[0]) - int(start[0])
    seconds = int(finish[1]) - int(start[1])
    microseconds = int(finish[2]) - int(start[2])

    if microseconds < 0:
        seconds -= 1
        microseconds += 1000000

    if seconds < 0:
        minutes -= 1
        seconds += 60

    print("Time to process (Minutes:Seconds.Microseconds): " + str(minutes) + ":" + str(seconds) + "." + str(
        microseconds))


if __name__ == "__main__":
    """
    Processes the log files in the log folder and puts them in the output.csv file
    """

    start_time = datetime.now().strftime("%M:%S:%f").split(":")
    line_count = getLineCount([LOGS_DIRECTORY + name for name in listdir(LOGS_DIRECTORY)])
    current_line = 0
    manager = multiprocessing.Manager()
    return_dict = manager.dict()
    print(f"Processing a total of {line_count} lines")
    processors = 4
    process_chunk_size = 50000  # Processes data in chunks, specified by this variable

    print(f"Writing to the CSV")
    header = ["time", "data_id", "description", "value"]

    with open(OUTPUT_PATH, "w", encoding="UTF8", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(header)

        for fp in listdir(LOGS_DIRECTORY):
            with open(LOGS_DIRECTORY + fp) as file:
                line_num = 0
                lines = []
                for line in file:
                    line_num += 1
                    current_line += 1
                    if current_line % 5000 == 0:
                        print(f"Line {line_num}")

                    lines.append(line)
                    try:
                        if line_num % process_chunk_size == 0:
                            """
                            When stored data reaches specified amount, use threads to decode data faster
                            """

                            with multiprocessing.Pool(processors) as p:
                                out = p.map(thread, lines)
                                lines = []
                            for data in out:
                                for sub_data in data:
                                    str_time = sub_data.timestamp.toString("yyyy-MM-ddTHH:mm:ss.zzzZ")
                                    writer.writerow([str_time, sub_data.id, DATA_IDS[sub_data.id], sub_data.value])

                    except:
                        print(f"Error with line {line_num} in file {file.name}")
                        pass

                if lines:
                    """
                    Handles leftover stored lines when the loop ends
                    """
                    with multiprocessing.Pool(processors) as p:
                        out = p.map(thread, lines)
                    for data in out:
                        for sub_data in data:
                            str_time = sub_data.timestamp.toString("yyyy-MM-ddTHH:mm:ss.zzzZ")
                            writer.writerow([str_time, sub_data.id, DATA_IDS[sub_data.id], sub_data.value])

    finish_time = datetime.now().strftime("%M:%S:%f").split(":")
    find_time(start_time, finish_time)
