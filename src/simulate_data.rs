// TODO: Convert Sim to new spec
//
#![allow(clippy::all)]
use daedalus::gen_simulate_data;
use std::time::Instant;

use crate::simulatable_message::SimComponent;

gen_simulate_data!();

// /// Base properties of every simulated message
// pub struct SimComponent {
//     values: Vec<f32>,      // DecodeData.value
//     topic: String,        // DecodeData.topic
//     unit: String,         // DecodeData.unit
//     last_update: Instant, // when the data was last updated
//     #[allow(dead_code)]
//     n_canpoints: u32, // number of can points
//     sim_freq: f32,        // Frequency in ms
//     #[allow(dead_code)]
//     id: String, // e.g. "0x80"

//                           // signed: bool,        // is the value signed?
//                           // size: u8,            // size of the value in bits
//                           // format: String,      // e.g. "divide10"
// }


fn create_simulated_components() -> Vec<SimComponent> {
    let mut components: Vec<SimComponent> = Vec::new();

    // let dummy = SimComponent {
    //     values: vec![0.0],
    //     topic: "BMS/PerCell/Beta/{0}/Faults/VA_OV".to_string(),
    //     unit: "".to_string(),
    //     last_update: Instant::now(),
    //     n_canpoints: 0,
    //     sim_freq: 700.0,
    //     id: "0x6F0".to_string(),
    // };


    components
}