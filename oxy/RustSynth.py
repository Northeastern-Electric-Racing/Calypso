from structs.CANField import CANField
from structs.CANMsg import CANMsg
from structs.Decoding import Decoding

from typing import Optional

class RustSynth:
    '''
    A class to synthesize Rust from a given CANMsg spec.
    '''

    inst_hashmap: str = "    let mut result = HashMap::new();"
    closing: str = "    result\n}"

    def synthesize(self, msg: CANMsg) -> str:
        signature: str = self.signature(msg.desc)
        generated_lines: list[str] = []
        for field in msg.fields:
            generated_lines.append(self.finalize_line(field.id, f"({self.parse_decoders(field)}){self.correcting_factor(field)}"))
        total_list: list[str] = [signature, self.inst_hashmap] + generated_lines + [self.closing]
        return "\n".join(total_list)

    def signature(self, to_decode: str) -> str:
        return f"pub fn decode_{to_decode.replace(' ', '_')}(data: &[u8]) -> HashMap<u8, f32> {{"

    def finalize_line(self, id: int, val: str) -> str:
        return f"    result.insert({id}, {val});"

    def parse_decoders(self, field: CANField) -> str:
        if isinstance(field.decodings, type(None)):
            return f"data[{field.index}] as f32"
        else:
            base: str = f"&data[{field.index}..{field.index + field.size}]"
            for decoder in field.decodings:
                base = f"pd::{decoder.repr}({base}, {decoder.bits}) as {decoder.final_type}"
            return base

    def correcting_factor(self, field:CANField) -> str:
        cf: str = ""
        if field.correcting_factor:
            cf = f" {field.correcting_factor.op} {field.correcting_factor.const}"
        return cf
