import json
import os
from pathlib import Path
from typing import List, Dict, Optional

CANGEN_PATH = "./dummy-can-messages"

def parse_file(fpath):
    try:
        with open(fpath, "r") as f:
            return json.load(f)
    except json.JSONDecodeError as e:
        print(f"Error parsing {fpath}: {e}")
    except Exception as e:
        print(f"Error reading {fpath}: {e}")

if __name__ == "__main__":
    print(parse_file(f"{CANGEN_PATH}/calypso_cmd.json")) 
