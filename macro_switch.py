import os
import sys

## A SIMPLE SCRIPT FOR SWITCHING BETWEEN THE ORIGINAL AND EXPANDED MACRO FILE

simdatars_original = """
#![allow(dead_code)] // TODO: Cleanup
#![allow(unused_imports)]
#![allow(unused_variables)]

#![allow(clippy::all)]
use daedalus::gen_simulate_data;
gen_simulate_data!();
"""

simdatars_base = """
#![allow(dead_code)] // TODO: Cleanup
#![allow(unused_imports)]
#![allow(unused_variables)]

#![allow(clippy::all)]
"""

if __name__ == "__main__":
    simrs_path = "./src/simulate_data.rs"
    expanded_path = "./macro_expanded.rs"

    mode = None
    if len(sys.argv) > 1:
        mode = sys.argv[1].lower()
        if mode not in ['original', 'expanded']:
            print("Invalid argument. Use 'original' or 'expanded'")
            sys.exit(1)

    with open(simrs_path, "r") as f:
        simrs = f.read()
        
        if mode == 'original' or (mode is None and simrs != simdatars_original):
            with open(simrs_path, "w") as f:
                f.write(simdatars_original)
            print("Switched to original file")
        elif mode == 'expanded' or (mode is None and simrs == simdatars_original):
            with open(expanded_path, "r") as f:
                expanded = f.read()
                with open(simrs_path, "w") as f:
                    f.write(simdatars_base + "\n\n" + expanded)
            os.system(f"rustfmt -q {simrs_path}") # TODO: Format that file
            print("Switched to expanded macro file")