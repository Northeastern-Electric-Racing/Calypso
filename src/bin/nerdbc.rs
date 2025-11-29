use std::{env, fs::File, io::Read};

use calypso_cangen::can_types::{CANMsg, CANPoint, Formatter, NetField, Sim};
use can_dbc::ByteOrder;

/// decode a DBC file and paste (mostly) compliant JSON to stdout
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1).expect("Must provide DBC file!");

    let mut dbc_file = File::open(arg).expect("Couldnt open DBC file");

    let mut dbc_buffer = Vec::new();
    dbc_file
        .read_to_end(&mut dbc_buffer)
        .expect("Couldnt read DBC file");

    let dbc = can_dbc::DBC::from_slice(&dbc_buffer).expect("Could not parse DBC file");

    let mut can_msgs = Vec::new();
    for message in dbc.messages() {
        let node_name = match message.transmitter() {
            can_dbc::Transmitter::NodeName(n) => n,
            can_dbc::Transmitter::VectorXXX => args
                .get(2)
                .expect("Must provide node name as one isnt found"),
        };
        let mut idex = 0;
        let mut points = Vec::new();
        let mut fields = Vec::new();

        for signal in message.signals() {
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

            points.push(CANPoint {
                size: signal.signal_size as usize,
                parse: None,
                signed: Some(signal.min < 0.0),
                endianness: map_endianness(*signal.byte_order()),
                formatter,
                default: None,
                ieee754_f32: None,
                sim: Some(Sim::SimRange {
                    min: signal.min as f32,
                    max: signal.max as f32,
                    inc_min: 1.0,
                    inc_max: 2.0,
                    round: None,
                }),
            });

            fields.push(NetField {
                name: format!("{node_name}/{}/{}", message.message_name(), signal.name()),
                unit: signal.unit().clone(),
                values: vec![idex],
            });
        }

        can_msgs.push(CANMsg {
            id: message.message_id().raw().to_string(),
            desc: format!("{node_name}_{}", message.message_name()),
            points,
            fields,
            key: None,
            is_ext: None,
            sim_freq: None,
            clients: None,
            bidir_mode: calypso_cangen::can_types::BidirMode::Broadcast,
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
