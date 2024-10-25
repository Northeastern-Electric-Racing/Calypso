import importlib.util
import json
import sys

# Full path to the directory containing the cangen module
EMBEDDED_BASE_PATH = "./Embedded-Base"
module_name = "cangen"

# Full path to the cangen module file
module_path = f"{EMBEDDED_BASE_PATH}/{module_name}/__init__.py"

# Add the cangen directory to the system's path
sys.path.append(EMBEDDED_BASE_PATH)

# Load the module
spec = importlib.util.spec_from_file_location(module_name, module_path)
cangen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cangen)

decode_data = open("./src/decode_data.rs", "w")
decode_master_mapping = open("./src/decode_master_mapping.rs", "w")

encode_data = open("./src/encode_data.rs", "w")
encode_master_mapping = open("./src/encode_master_mapping.rs", "w")

simulate_data = open("./src/simulate_data.rs", "w")

format_data = open("./src/format_data.rs", "w")


bms_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/bms.json", "r"))
mpu_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/mpu.json", "r"))
wheel_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/wheel.json", "r"))
dti_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/dti.json", "r"))
calypso_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/calypso_cmd.json", "r"))
charger_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/charger.json", "r"))
msb_messages = json.load(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/msb.json", "r"))


bms_messages.extend(mpu_messages)
bms_messages.extend(wheel_messages)
bms_messages.extend(dti_messages)
bms_messages.extend(charger_messages)
bms_messages.extend(calypso_messages)
bms_messages.extend(msb_messages)

result = cangen.RustSynthFromJSON().parse_messages(bms_messages)

decode_data.write(result.decode_data)
decode_data.close()

decode_master_mapping.write(result.decode_master_mapping)
decode_master_mapping.close()

encode_data.write(result.encode_data)
encode_data.close()

encode_master_mapping.write(result.encode_master_mapping)
encode_master_mapping.close()

simulate_data.write(result.simulate_data)
simulate_data.close()

format_data.write(result.format_data)
format_data.close()

