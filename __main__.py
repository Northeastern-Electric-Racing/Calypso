import multiprocessing
from datetime import datetime

from os import listdir, path
import csv
import sys
from typing import List

from ner_processing.master_mapping import DATA_IDS
from ner_processing.thread import thread

DEFAULT_LOGS_DIRECTORY = "./logs/"
DEFAULT_OUTPUT_PATH = "./output.csv"
PROCESSORS = 8
PROCESS_CHUNK_SIZE = 85000  # Processes data in chunks, specified by this variable


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


def process_lines(lines, writer):
    """
    Processes a chunk of lines and writes the results to the CSV.
    """
    with multiprocessing.Pool(PROCESSORS) as p:
        out = p.map(thread, lines)
    lines.clear()
    for data in out:
        for sub_data in data:
            str_time = sub_data.timestamp.toString("yyyy-MM-ddTHH:mm:ss.zzzZ")
            writer.writerow([str_time, sub_data.id, DATA_IDS[sub_data.id]["name"], sub_data.value])


if __name__ == "__main__":
    """
    Processes the log files in the log folder and puts them in the output.csv file.
    Command line args:
        - arg 1 = output directory of the csv file
            - Must be a directory name (C:\Users\user1\Documents)
            - A file called 'output.csv' is created here
        - args 2... = space separated list of file paths to process
            - Each path must be a text file (C:\Users\user1\Documents\logs.txt)
    Default file paths are all those in "./logs/"
    Default output directory is the current location
    """

    start_time = datetime.now().strftime("%M:%S:%f").split(":")
    output_path = ""
    paths_to_process = []

    # If manually specifying the paths
    if len(sys.argv) > 1:
        # Formats the input file path strings correctly
        for i in range(1, len(sys.argv)):
            sys.argv[i] = sys.argv[i].replace("\\","/") 
        
        output_path = sys.argv[1] + "/output.csv"
        paths_to_process = sys.argv[2:]
    else:
        output_path = DEFAULT_OUTPUT_PATH
        paths_to_process = [DEFAULT_LOGS_DIRECTORY + name for name in listdir(DEFAULT_LOGS_DIRECTORY)]

    line_count = getLineCount(paths_to_process)
    print(f"Processing a total of {line_count} lines")

    current_line = 0
    manager = multiprocessing.Manager()
    return_dict = manager.dict()

    print(f"Writing to the CSV")
    header = ["time", "data_id", "description", "value"]

    with open(output_path, "w", encoding="UTF8", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(header)

        for fp in paths_to_process:
            with open(fp) as file:
                line_num = 0
                lines = []
                for line in file:
                    line_num += 1
                    current_line += 1
                    if current_line % 5000 == 0:
                        print(f"Line {line_num}")

                    lines.append(line)
                    try:
                        if line_num % PROCESS_CHUNK_SIZE == 0:
                            # When stored data reaches specified amount, use threads to decode data faster
                            process_lines(lines, writer)
                    except:
                        print(f"Error with line {line_num} in file {file.name}")
                        pass

                if lines:
                    # Handles leftover stored lines when the loop ends
                    process_lines(lines, writer)

    finish_time = datetime.now().strftime("%M:%S:%f").split(":")
    find_time(start_time, finish_time)
