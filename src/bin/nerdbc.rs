use std::{env, fs::File, io::Read};

use can_dbc::{ByteOrder, NumericValue};
use definition_rs::{CANMsg, CANPoint, Formatter, NetField, Sim};

/// decode a DBC file and paste (mostly) compliant JSON to stdout
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).expect("Must provide DBC file!");

    let mut dbc_file = File::open(arg).expect("Couldnt open DBC file");

    let mut dbc_buffer = String::new();
    dbc_file
        .read_to_string(&mut dbc_buffer)
        .expect("Couldnt read DBC file");

    let dbc = can_dbc::Dbc::try_from(dbc_buffer.as_str()).expect("Could not parse DBC file");

    let mut can_msgs = Vec::new();
    for message in dbc.messages {
        let node_name: String = match message.transmitter {
            can_dbc::Transmitter::NodeName(n) => n,
            can_dbc::Transmitter::VectorXXX => (args
                .get(2)
                .expect("Must provide node name as one isnt found"))
            .clone(),
        };
        let mut idex = 0;
        let mut points = Vec::new();
        let mut fields = Vec::new();

        for signal in message.signals {
            // idex 1 indexed
            idex += 1;
            let formatter = if (signal.factor - 1.0).abs() < 0.001 {
                None
            } else {
                Some(Formatter {
                    key: "divide".to_owned(),
                    arg: signal.factor as f32,
                })
            };

            let is_signed = match signal.min {
                NumericValue::Uint(_) => false,
                NumericValue::Int(i) => i < 0,
                NumericValue::Double(d) => d < 0.0f64,
            };

            let min = match signal.min {
                NumericValue::Uint(u) => u as f32,
                NumericValue::Int(i) => i as f32,
                NumericValue::Double(d) => d as f32,
            };

            let max = match signal.max {
                NumericValue::Uint(u) => u as f32,
                NumericValue::Int(i) => i as f32,
                NumericValue::Double(d) => d as f32,
            };

            points.push(CANPoint {
                size: signal.size as usize,
                name: None,
                c_type: None,
                parse: None,
                signed: Some(is_signed),
                endianness: map_endianness(signal.byte_order),
                formatter,
                default: None,
                ieee754_f32: None,
                sim: Some(Sim::SimRange {
                    min,
                    max,
                    inc_min: 1.0,
                    inc_max: 2.0,
                    round: None,
                }),
            });

            fields.push(NetField {
                name: format!("{node_name}/{}/{}", message.name, signal.name),
                unit: signal.unit.clone(),
                values: vec![idex],
                doc: String::new(),
                desc: None,
            });
        }

        can_msgs.push(CANMsg {
            id: message.id.raw().to_string(),
            desc: format!("{node_name}_{}", message.name),
            points,
            fields,
            key: None,
            is_ext: None,
            sim_freq: None,
            clients: None,
            bidir_mode: definition_rs::BidirMode::Broadcast,
        });
    }

    let res = serde_json::to_string_pretty(&can_msgs).expect("Could not build JSON");

    println!("{res}");
}

fn map_endianness(bo: ByteOrder) -> Option<String> {
    match bo {
        ByteOrder::LittleEndian => Some("little".to_owned()),
        ByteOrder::BigEndian => None,
    }
}
