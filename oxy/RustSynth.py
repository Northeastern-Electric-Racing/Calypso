from structs.CANField import CANField
from structs.CANMsg import CANMsg
class RustSynth:
    '''
    A class to synthesize Rust from a given CANMsg spec.
    '''

    return_type: str = "Vec<Data>"
    inst_hashmap: str = f"    let mut result = {return_type}::new();"
    closing: str = "    result\n}"

    def synthesize(self, msg: CANMsg) -> str:
        signature: str = self.signature(msg.desc)
        generated_lines: list[str] = []
        for field in msg.fields:
            generated_lines.append(self.finalize_line(field.name, field.units, f"({self.format_data(field, self.parse_decoders(field))})"))
        total_list: list[str] = [signature, self.inst_hashmap] + generated_lines + [self.closing]
        return "\n".join(total_list)

    def signature(self, to_decode: str) -> str:
        return f"pub fn decode_{to_decode.replace(' ', '_')}(data: &[u8]) -> {self.return_type} {{"

    def finalize_line(self, topic: str, unit: str, val: str) -> str:
        return f"    result.push(Data::new({val}, \"{topic}\", \"{unit}\"));"

    def parse_decoders(self, field: CANField) -> str:
        if isinstance(field.decodings, type(None)):
            return f"data[{field.index}] as f32"
        else:
            base: str = f"&data[{field.index}..{field.index + field.size}]"
            for decoder in field.decodings:
                base = f"pd::{decoder.repr}({base}, {decoder.bits}) as {decoder.final_type}"
            return base

    def format_data(self, field:CANField, decoded_data: str) -> str:
        cf = decoded_data
        if field.format:
            cf = f"fd::{field.format}({decoded_data})"
        return cf
