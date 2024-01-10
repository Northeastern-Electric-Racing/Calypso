from YAMLParser import YAMLParser
from RustSynth import RustSynth

print(RustSynth().synthesize(YAMLParser().parse(open("mapping.yaml", "r"))))
