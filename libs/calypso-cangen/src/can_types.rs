use serde::Deserialize;

// TODO: Implement MsgType

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
    pub sim_freq: Option<f32>,
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
    pub sim: Option<Sim>,
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
    pub default_value: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
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
