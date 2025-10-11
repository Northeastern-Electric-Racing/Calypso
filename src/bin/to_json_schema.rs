use calypso_cangen::can_types::CANMsg;
use schemars::schema_for;



fn main() {
    let schema = schema_for!(Vec<CANMsg>);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}