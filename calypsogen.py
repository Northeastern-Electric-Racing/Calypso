import importlib.util
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
master_mapping = open("./src/master_mapping.rs", "w")

bms_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/bms.yaml", "r"))
mpu_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/mpu.yaml", "r"))
wheel_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/wheel.yaml", "r"))

bms_messages.msgs.extend(mpu_messages.msgs)
bms_messages.msgs.extend(wheel_messages.msgs)

result = cangen.RustSynth().parse_messages(bms_messages.msgs)

decode_data.write(result.decode_data)
decode_data.close()

master_mapping.write(result.master_mapping)
master_mapping.close()
