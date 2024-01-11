from structs.CANField import CANField
from structs.CANMsg import CANMsg
from structs.Messages import Messages
from structs.Result import Result

class RustSynth:
    '''
    A class to synthesize Rust from a given CANMsg spec.
    '''

    ignore_clippy: str = "#![allow(clippy::all)]\n"
    decode_data_import: str = "use super::data::{Data,FormatData as fd, ProcessData as pd}; \n"

    decode_return_type: str = "Vec::<Data>"
    decode_return_value: str = f"    let result = vec!["
    decode_close: str = "    ]; \n    result\n}\n"

    decode_mock: str = """
pub fn decode_mock(_data: &[u8]) -> Vec::<Data> {
    let result = vec![
    Data::new(0.0, "Mock", "")
    ];
    result
}
"""

    master_mapping_import: str = "use super::decode_data::*; \nuse super::data::Data; \n"

    master_mapping_signature: str = "pub fn get_message_info(id: &u32) -> MessageInfo { \n   match id {"

    master_mapping_closing: str = "    _ => MessageInfo::new(decode_mock), \n    }\n}"

    message_info = """
pub struct MessageInfo {
    pub decoder: fn(data: &[u8]) -> Vec<Data>,
} 

impl MessageInfo {
    pub fn new(decoder: fn(data: &[u8]) -> Vec<Data>) -> Self {
        Self {
            decoder
        }
    }
}
"""

    def parse_messages(self, msgs: Messages) -> Result:
        result = Result("", "")
        result.decode_data += self.ignore_clippy
        result.decode_data += self.decode_data_import
        result.decode_data += self.decode_mock

        result.master_mapping += self.master_mapping_import
        result.master_mapping += self.message_info
        result.master_mapping += self.master_mapping_signature

        for msg in msgs.msgs:
            result.decode_data += self.synthesize(msg) + "\n"
            result.master_mapping += self.map_msg_to_decoder(msg)

        result.master_mapping += self.master_mapping_closing
        return result

    def map_msg_to_decoder(self, msg: CANMsg) -> str:
        return f"    {msg.id} => MessageInfo::new({self.function_name(msg.desc)}),\n"    

    def synthesize(self, msg: CANMsg) -> str:
        signature: str = self.signature(msg.desc)
        generated_lines: list[str] = []
        for field in msg.fields:
            generated_lines.append(self.finalize_line(field.name, field.unit, f"{self.format_data(field, self.parse_decoders(field))}"))
        total_list: list[str] = [signature, self.decode_return_value] + generated_lines + [self.decode_close]
        return "\n".join(total_list)
    
    def function_name(self, desc: str) -> str:
        return f"decode_{desc.replace(' ', '_').lower()}"

    def signature(self, desc: str) -> str:
        return f"pub fn {self.function_name(desc)}(data: &[u8]) -> {self.decode_return_type} {{"

    def finalize_line(self, topic: str, unit: str, val: str) -> str:
        return f"    Data::new({val}, \"{topic}\", \"{unit}\"),"

    def parse_decoders(self, field: CANField) -> str:
        if isinstance(field.decodings, type(None)):
            return f"data[{field.index}] as {field.final_type}"
        else:
            base: str
            if field.size == 1:
                base = f"data[{field.index}]"
            else :
                base = f"&data[{field.index}..{field.index + field.size}]"
            for decoder in field.decodings:
                base = f"pd::{decoder.repr}({base} as {decoder.entry_type}, {decoder.bits})"
            return f"{base} as {field.final_type}"

    def format_data(self, field:CANField, decoded_data: str) -> str:
        cf = decoded_data
        if field.format:
            cf = f"fd::{field.format}({decoded_data})"
        return cf
