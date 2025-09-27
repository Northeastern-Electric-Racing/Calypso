use proc_macro2::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};

// TODO: Implement MsgType

// Classes to represent levels of the CAN hierarchy
// For more specific descriptions, refer to the README
// in Embedded-Base/cangen

/**
 *  Class representing a CAN message
 */
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANMsg {
    pub id: String,
    pub desc: String,
    pub points: Vec<CANPoint>,
    pub fields: Vec<NetField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ext: Option<bool>,
    #[serde(default)]
    pub bidir_mode: BidirMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_freq: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<u16>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all(deserialize = "lowercase", serialize = "PascalCase"))]
#[derive(Default)]
pub enum BidirMode {
    Oneshot,
    #[default]
    Broadcast,
}

impl quote::ToTokens for BidirMode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path: TokenStream = match self {
            BidirMode::Broadcast => quote!(::calypso_cangen::can_types::BidirMode::Broadcast),
            BidirMode::Oneshot => quote!(::calypso_cangen::can_types::BidirMode::Oneshot),
        };
        tokens.extend(path);
    }
}

/**
 *  Class representing a NetField of a CAN message
 */
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetField {
    pub name: String,
    pub unit: String,
    pub values: Vec<usize>,
}

/**
 *  Class representing a CAN point of a NetField
 */
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANPoint {
    pub size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endianness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<Formatter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ieee754_f32: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim: Option<Sim>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Formatter {
    pub key: String,
    pub arg: f32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum Sim {
    SimRange {
        min: f32,
        max: f32,
        inc_min: f32,
        inc_max: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        round: Option<bool>,
    },
    SimDiscrete {
        options: Vec<[f32; 2]>,
    },
}
