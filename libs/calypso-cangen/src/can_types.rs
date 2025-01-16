use serde::Deserialize;

// TODO: Implement MsgType

// Classes to represent levels of the CAN hierarchy
// For more specific descriptions, refer to the README
// in Embedded-Base/cangen

/**
 *  Class representing a CAN message
 */
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANMsg {
    pub id: String,
    pub desc: String,
    pub points: Vec<CANPoint>,
    pub fields: Vec<NetField>,
    pub key: Option<String>,
    pub is_ext: Option<bool>,
    pub sim_freq: Option<f32>,
}

/**
 *  Class representing a NetField of a CAN message
 */
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetField {
    pub name: String,
    pub unit: String,
    pub value: u8;
}

/**
 *  Class representing a CAN point of a NetField
 */
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANPoint {
    pub size: usize,
    pub parse: Option<bool>,
    pub signed: Option<bool>,
    pub endianness: Option<String>,
    pub format: Option<String>,
    pub default: Option<f32>,
    pub ieee754_f32: Option<bool>,
    pub sim: Option<Sim>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum Sim {
    SimSweep {
        min: f32,
        max: f32,
        inc_min: f32,
        inc_max: f32,
        round: Option<bool>,
    },
    SimEnum {
        options: Vec<[f32; 2]>,
    },
}
