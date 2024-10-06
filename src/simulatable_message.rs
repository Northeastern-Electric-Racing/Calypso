use super::data::DecodeData;
use rand::prelude::*;
use std::time::Instant;

/**
 * Wrapper class for an individual message.
 */
pub struct SimulatedComponents {
    pub value: Vec<f32>,      // DecodeData.value
    pub topic: String,        // DecodeData.topic
    pub unit: String,         // DecodeData.unit
    pub last_update: Instant, // when the data was last updated
    pub n_canpoints: u32,     // number of can points
    pub sim_min: f32,         // min value
    pub sim_max: f32,         // max value
    pub sim_inc_min: f32,     // min increment step
    pub sim_inc_max: f32,     // max increment step
    pub sim_freq: f32,        // Frequency in ms
    pub format: String,       // e.g. "divide10"
    pub id: String,           // e.g. "0x80" (or should this be a uint32?)
    pub signed: bool,         // is the value signed?
}

/**
 * Implementation of SimulatedComponents.
 */
impl SimulatedComponents {
    pub fn should_update(&self) -> bool {
        self.last_update.elapsed().as_millis() > self.sim_freq as u128
    }

    pub fn new(
        topic: String,
        unit: String,
        sim_min: f32,
        sim_max: f32,
        sim_inc_min: f32,
        sim_inc_max: f32,
        sim_freq: f32,
        n_canpoints: u32,
        format: String,
        id: String,
        signed: bool,
    ) -> Self {
        println!("[SimulatedComponents.initialize] Initializing simulated components");

        let mut value = vec![0.0; n_canpoints as usize];

        // initialize value with random values between sim_min and sim_max
        let mut rng = rand::thread_rng();
        for i in 0..n_canpoints as usize {
            value[i] = rng.gen_range(sim_min..sim_max);
        }

        Self {
            value,
            topic,
            unit,
            last_update: Instant::now(),
            n_canpoints,
            sim_min,
            sim_max,
            sim_inc_min,
            sim_inc_max,
            sim_freq,
            format,
            id,
            signed,
        }
    }

    pub fn update(&mut self) {
        println!("[SimulatedComponents.update] Updating simulated components");
        println!("[SimulatedComponents.update] id: {}, format: {}, sim_min: {}, sim_max: {}, sim_inc_min: {}, sim_inc_max: {}, sim_freq: {}", self.id, self.format, self.sim_min, self.sim_max, self.sim_inc_min, self.sim_inc_max, self.sim_freq);
        if self.should_update() {
            self.last_update = Instant::now();
            let mut rng = rand::thread_rng();
            for i in 0..self.value.len() {
                let mut rand_offset: f32;

                // handle the case where sim_inc_min == sim_inc_max
                if self.sim_inc_min == self.sim_inc_max {
                    rand_offset = self.sim_inc_min;
                } else {
                    rand_offset = rng.gen_range(self.sim_inc_min..self.sim_inc_max);
                }
                let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                
                // getting a in-range new value
                let mut new_value = self.value[i] + sign * rand_offset;
                while new_value < self.sim_min || new_value > self.sim_max {
                    if self.sim_inc_min == self.sim_inc_max {
                        rand_offset = self.sim_inc_min;
                    } else {
                        rand_offset = rng.gen_range(self.sim_inc_min..self.sim_inc_max);
                    }
                    let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                    new_value = self.value[i] + sign * rand_offset;
                }
                self.value[i] = new_value;
            }
        }
    }

    pub fn get_data(&self) -> DecodeData {
        println!("[SimulatedComponents.get_data] Retrieving simulated data");
        DecodeData::new(self.value.clone(), &self.topic, &self.unit)
    }
}
