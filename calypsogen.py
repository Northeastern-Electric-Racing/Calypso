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
decode_master_mapping = open("./src/decode_master_mapping.rs", "w")

encode_data = open("./src/encode_data.rs", "w")
encode_master_mapping = open("./src/encode_master_mapping.rs", "w")

format_data = open("./src/format_data.rs", "w")


bms_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/bms.yaml", "r"))
mpu_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/mpu.yaml", "r"))
wheel_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/wheel.yaml", "r"))
dti_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/dti.yaml", "r"))
calypso_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/calypso_cmd.yaml", "r"))
charger_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/charger.yaml", "r"))
msb_messages = cangen.YAMLParser().parse(open(f"{EMBEDDED_BASE_PATH}/{module_name}/can-messages/msb.yaml", "r"))


bms_messages.msgs.extend(mpu_messages.msgs)
bms_messages.msgs.extend(wheel_messages.msgs)
bms_messages.msgs.extend(dti_messages.msgs)
bms_messages.msgs.extend(charger_messages.msgs)
bms_messages.msgs.extend(calypso_messages.msgs)
bms_messages.msgs.extend(msb_messages.msgs)

result = cangen.RustSynth().parse_messages(bms_messages.msgs)

decode_data.write(result.decode_data)
decode_data.close()

decode_master_mapping.write(result.decode_master_mapping)
decode_master_mapping.close()

encode_data.write(result.encode_data)
encode_data.close()

encode_master_mapping.write(result.encode_master_mapping)
encode_master_mapping.close()

format_data.write(result.format_data)
format_data.close()

