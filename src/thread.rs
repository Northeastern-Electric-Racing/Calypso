use decode_files::{LogFormat, processLine};
use message::Message;

const FORMAT: LogFormat = LogFormat::Textual1;

fn thread(line: &str) -> Result<Vec<Data>, MessageFormatException> {
    let message = processLine(line, FORMAT)?;
    message.decode()
}