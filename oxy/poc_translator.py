from ruamel.yaml import YAML
from functools import reduce


yaml: YAML = YAML(typ="safe")

'''
pub fn decode_accumulator_status(data: &[u8]) -> HashMap<u8, f32> {
    let mut result = HashMap::new();
    result.insert(1, (pd::big_endian(&data[0..2], 8) as f32) / 10.0);
    result.insert(
        2,
        pd::twos_comp(pd::big_endian(&data[2..4], 8) as u32, 16) as f32 / 10.0,
    );
    result.insert(3, pd::big_endian(&data[4..6], 8) as f32);
    result.insert(4, data[6] as f32);
    result.insert(5, data[7] as f32);
    result
}
'''
out_string: str = ""
data = yaml.load(open("mapping.yaml"))[1]
# print(data)

function_name: str = "decode" + "_" + "_".join(data['string'].split(" "))
args: str = "(data: &[u8])"
returnVal: str= " -> HashMap<u8, f32>"

signature: str = "pub fn " + function_name + args + returnVal + " {"
instantiate_hash_map: str = "    let mut result = HashMap::new();"
conclusion: str = "    result\n}"

decodings: list[str] = []
accumulated_size: int = 0
for field in data["fields"]: # result.insert(1, (pd::big_endian(&data[0..2], 8) as f32) / 10.0);
    field: dict = field
    id = field["field_id"]
    if field["size"] > 1: # we need to do some decoding, then
        to_decode: str = f"&data[{accumulated_size}..{accumulated_size+field['size']}]"
        _cf: str = field.get("correcting_factor", "")
        correcting_factor: str = f"{(' / ' if '/' in _cf else ' * ') if _cf else ''}{_cf.split('/')[-1]}"
        for decodingsetup in field["decoding"]:
            decodingsetup: dict[str, dict[str, str]] = {k: reduce(lambda x,y: x|y, v, {}) for k,v in decodingsetup.items()}
            for decoder, params in decodingsetup.items():
                match decoder:
                    case "big_endian":
                        to_decode = f"pd::big_endian({to_decode}, {params['bits']}) as {params['final_type']}"
                    case "twos_complement":
                        to_decode = f"pd::twos_comp({to_decode}, {params['bits']}) as {params['final_type']}"
        decoded = f"{id}, {to_decode}{correcting_factor}"
        decodings.append(decoded)
    else: # no decoding required!
        decoded = f"{id}, data[{accumulated_size}] as {field['final_type']}"
        decodings.append(decoded)
    accumulated_size += field["size"]

formatted_decodings = [f"    result.insert({i});" for i in decodings]

finals: list[str] = [signature, instantiate_hash_map] + formatted_decodings + [conclusion]
print("\n".join(finals))




'''{'id': 1,
'string': 'accumulator status',
'fields': [{
'field_id': 1,
'name': 'Pack Inst Voltage',
'units': 'V',
'size': 2,
'decoding': [{'correcting_factor': '1/10'},
  {'decoders': [{'big_endian': [{'final_type': 'f32'}]}]}]}, {'field_id': 2, 'name': 'Pack Current', 'units': 'A', 'size': 2, 'decoding': [{'correcting_factor': '1/10'}, {'decoders': [{'big_endian': [{'final_type': 'u32'}]}, {'twos_complement': [{'bits': 8}, {'final_type': 'f32'}]}]}]}, {'field_id': 3, 'name': 'Pack Amp-hours', 'units': 'Ah', 'size': 2, 'decoding': [{'decoders': [{'big_endian': [{'final_type': 'f32'}]}]}]}, {'field_id': 4, 'name': 'Pack SOC', 'units': '%', 'size': 1}, {'field_id': 5, 'name': 'Pack Health', 'units': '%', 'size': 1}]}'''
