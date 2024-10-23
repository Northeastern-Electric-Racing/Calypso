use serde::Deserialize;

// enum MsgType {
//     encode,
//     decode,
//     raw
// }

/**
 *  Class representing a CAN message
 */
#[derive(Deserialize, Debug)]
pub struct CANMsg {
    pub id: String,
    pub desc: String,
    pub fields: Vec<NetField>,
    pub key: Option<String>,
    pub is_ext: Option<bool>,
}

/**
 *  Class representing a NetField of a CAN message
 */
#[derive(Deserialize, Debug)]
pub struct NetField {
    pub name: String,
    pub unit: String,
    pub points: Vec<CANPoint>,
    pub send: Option<bool>,
    pub topic_append: Option<bool>,
}

/**
 *  Class representing a CAN point of a NetField
 */
#[derive(Deserialize, Debug)]
pub struct CANPoint {
    pub size: usize,
    pub signed: Option<bool>,
    pub endianness: Option<String>,
    pub format: Option<String>,
    pub default: Option<f32>,
}
