from YAMLParser import YAMLParser
from RustSynth import RustSynth

f = open("../src/decode_data.rs", "w")

f.write(RustSynth().parse_messages(YAMLParser().parse(open("mapping.yaml", "r"))))

f.close()
