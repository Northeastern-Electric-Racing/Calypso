#![warn(clippy::pedantic)]
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

pub mod data;
pub mod decode_data;
pub mod encode_data;
pub mod proto;
pub mod simulatable_message;
pub mod simulate_data;
