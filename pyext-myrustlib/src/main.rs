use std::io::Write;
use rayon::prelude::*;
use chrono::prelude::*;

use master_mapping::DATA_IDS;
use master_mapping::MESSAGE_IDS;

mod master_mapping;

const DEFAULT_LOGS_DIRECTORY: &str = "./logs/";
const DEFAULT_OUTPUT_PATH: &str = "./output/";
const PROCESSORS: u8 = 8;
const PROCESS_CHUNK_SIZE: u32 = 85000; //Processes data in chunks, specified by this variable

fn get_line_count(filepaths: &[String]) -> usize {
   /**
    * Gets the total line count of all the files in the list. 

    There is no native way to get line counts of files without looping, so 
    this function gets the total size and estimates the line count based
    on a subset of N lines. 
    */
   if filepaths.is_empty() {
      return 0;
   }

   const n: u8 = 20;
   let tested_lines: u8 = 0;
   let total_size: u8 = 0;
   let total_size: u64 = filepaths.iter().map(|filepath| fs::metadata(filepath).unwrap().len()).sum();

   for filepath in filepaths {
      let file = File::open(filepath).unwrap();
      let reader = BufReader::new(file);

      for line in reader.lines() {
         let line = line.unwrap();
         total_size += line.len();
         tested_lines += 1;

         if tested_lines >= n {
            return (total_size / tested_lines) * (total_size / tested_lines);
         }
      }
   }
   return (total_size / (tested_size / tested_lines))
}

fn find_time(start: &[&str], finish: &[&str]) {
   /**
    * Prints the difference between the two times provided
    Both inputs are lists of strings:
        - minutes being the zeroth index of the list
        - seconds being the first index of the list
        - microseconds being the second index of the list
    */

    let start_minutes = start[0].parse::<i32>().unwrap();
    let start_seconds = start[1].parse::<i32>().unwrap();
    let start_microseconds = start[2].parse::<i32>().unwrap();

    let finish_minutes = finish[0].parse::<i32>().unwrap();
    let finish_seconds = finish[1].parse::<i32>().unwrap();
    let finish_microseconds = finish[2].parse::<i32>().unwrap();

    let mut minutes = finish_minutes - start_minutes;
    let mut seconds = finish_seconds - start_seconds;
    let mut microseconds = finish_microseconds - start_microseconds;

    if microseconds < 0 {
        seconds -= 1;
        microseconds += 1_000_000;
    }

    if seconds < 0 {
        minutes -= 1;
        seconds += 60;
    }

    println!("Time to process (Minutes:Seconds.Microseconds): {}:{}.{:06}", minutes, seconds, microseconds);
}

fn process_lines(lines: &mut Vec<String>, writer: &mut csv::Writer<&mut dyn Write>) {
   /**
    * Processes a chunk of lines and writes the results to the CSV.
    */
    let out: Vec<Vec<_>> = lines.par_iter().map(|line| thread(line)).collect();
    lines.clear();
    for data in out {
        for sub_data in data {
            let str_time = sub_data.timestamp.format("%Y-%m-%dT%H:%M:%S.%fZ").to_string();
            writer.write_record(&[&str_time, &sub_data.id.to_string(), &DATA_IDS[sub_data.id]["name"], &sub_data.value.to_string()]).unwrap();
        }
    }
}
