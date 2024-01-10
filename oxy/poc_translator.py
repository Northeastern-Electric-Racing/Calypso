from ruamel.yaml import YAML, Any
from functools import reduce


yaml: YAML = YAML(typ="safe")

out_string: str = ""
data: dict[str, Any] = yaml.load(open("mapping.yaml"))
print(data)
print(type(data))

function_name: str = "decode" + "_" + "_".join(data['string'].split(" "))
args: str = "(data: &[u8])"
returnVal: str= " -> HashMap<u8, f32>"

signature: str = "pub fn " + function_name + args + returnVal + " {"
instantiate_hash_map: str = "    let mut result = HashMap::new();"
conclusion: str = "    result\n}"

decodings: list[str] = []
accumulated_size: int = 0
for field in data["fields"]: # result.insert(1, (pd::big_endian(&data[0..2], 8) as f32) / 10.0);
    field: dict
    decoded: str
    id = field["field_id"]
    if field["size"] > 1: # we need to do some decoding, then
        to_decode: str = f"&data[{accumulated_size}..{accumulated_size+field['size']}]"
        _cf: str = field.get("correcting_factor", "")
        correcting_factor: str = f"{' ' + ('/' if '/' in _cf else '*') + ' ' if 'correcting_factor' in field.keys() else ''}{_cf.split('/')[-1]}"
        for decodingsetup in field["decoding"]:
            decodingsetup: dict[str, dict[str, str]] = {k: reduce(lambda x,y: x|y, v, {}) for k,v in decodingsetup.items()}
            for decoder, params in decodingsetup.items():
                match decoder:
                    case "big_endian":
                        to_decode = f"pd::big_endian({to_decode}, {params['bits']}) as {params['final_type']}"
                    case "twos_complement":
                        to_decode = f"pd::twos_comp({to_decode}, {params['bits']}) as {params['final_type']}"
        decoded = f"{id}, {to_decode}{correcting_factor}"
    else: # no decoding required!
        decoded = f"{id}, data[{accumulated_size}] as {field['final_type']}"

    decodings.append(decoded)
    accumulated_size += field["size"]

formatted_decodings = [f"    result.insert({i});" for i in decodings]

finals: list[str] = [signature, instantiate_hash_map] + formatted_decodings + [conclusion]
print("\n".join(finals))
