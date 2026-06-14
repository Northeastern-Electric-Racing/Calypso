use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Classes to represent levels of the CAN hierarchy
// For more specific descriptions, refer to the README
// in Embedded-Base/cangen

// See https://nerdocs.atlassian.net/wiki/spaces/NER/pages/1162018881/Odyssey+Car+Configuration+Framework
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged, expecting = "CANMsg")]
pub enum OdysseyMsg {
    Can(CANMsg),
    Meta(MetaMsg), // to be indexed, but not recieved - MUST GO LAST
}

#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetaMsg {
    pub desc: String,
    pub fields: Vec<DumbNetField>,
}

/**
 *  Class representing a CAN message
 */
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
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

#[derive(JsonSchema, Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all(deserialize = "lowercase", serialize = "PascalCase"))]
#[derive(Default)]
pub enum BidirMode {
    Oneshot,
    #[default]
    Broadcast,
    Configuration,
}

impl quote::ToTokens for BidirMode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path: TokenStream = match self {
            BidirMode::Broadcast => {
                quote!(::calypso_cangen::can_types::BidirMode::Broadcast)
            }
            BidirMode::Oneshot => quote!(::calypso_cangen::can_types::BidirMode::Oneshot),
            BidirMode::Configuration => {
                panic!("Configuration sendable message is not available yet")
            }
        };
        tokens.extend(path);
    }
}

/**
 *  Class representing a `NetField` of a CAN message
 */
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetField {
    pub name: String,
    pub unit: String,
    pub doc: String,
    pub desc: Option<String>,
    pub values: Vec<usize>,
}

/**
 *  Class representing a `NetField` of a Meta message (there are no values)
 */
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DumbNetField {
    pub name: String,
    pub unit: String,
    pub doc: String,
    pub desc: Option<String>,
}

/**
 *  Class representing a CAN point of a `NetField`
 */
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANPoint {
    pub size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_type: Option<String>,
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

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
pub struct Formatter {
    pub key: String,
    pub arg: f32,
}

#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
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
