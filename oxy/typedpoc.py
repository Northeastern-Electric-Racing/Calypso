from YAMLParser import YAMLParser
from RustSynth import RustSynth

decode_data = open("../src/decode_data.rs", "w")
master_mapping = open("../src/master_mapping.rs", "w")

result = RustSynth().parse_messages(YAMLParser().parse(open("mapping.yaml", "r")))

decode_data.write(result.decode_data)
decode_data.close()

master_mapping.write(result.master_mapping)
master_mapping.close()