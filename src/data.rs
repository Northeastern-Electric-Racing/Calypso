//! CAN data-model types. The definitions live in `calypso-cangen` so `calypso-sim`
//! can share them without depending on this (Linux-only) crate; they are
//! re-exported here so `crate::data::…` and the decode/encode codegen macros are
//! unchanged.

pub use calypso_cangen::data::{DecodeData, EncodeData, FormatData};
