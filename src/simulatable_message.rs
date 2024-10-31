use super::data::DecodeData;
use rand::prelude::*;
use std::time::Instant;

/**
 * Wrapper class for an individual message.
 */
pub struct SimulatedComponent {
    value: Vec<f32>,      // DecodeData.value
    topic: String,        // DecodeData.topic
    unit: String,         // DecodeData.unit
    last_update: Instant, // when the data was last updated
    #[allow(dead_code)]
    n_canpoints: u32,     // number of can points
    sim_min: f32,         // min value
    sim_max: f32,         // max value
    sim_inc_min: f32,     // min increment step
    sim_inc_max: f32,     // max increment step
    sim_freq: f32,        // Frequency in ms
    // format: String,       // e.g. "divide10"
    #[allow(dead_code)]
    id: String,           // e.g. "0x80" (or should this be a uint32?)
    // signed: bool,         // is the value signed?
    // size: u8,            // size of the value in bits
}


pub struct SimulatedComponentAttr {
    pub sim_min: f32,
    pub sim_max: f32,
    pub sim_inc_min: f32,
    pub sim_inc_max: f32,
    pub sim_freq: f32,
    pub n_canpoints: u32,
    pub id: String,
}

/**
 * Implementation of SimulatedComponents.
 */
impl SimulatedComponent {
    pub fn should_update(&self) -> bool {
        self.last_update.elapsed().as_millis() > self.sim_freq as u128
    }

    pub fn new(
        topic: String,
        unit: String,
        attr: SimulatedComponentAttr
    ) -> Self {
            
        let sim_min: f32 = attr.sim_min;
        let sim_max: f32 = attr.sim_max;
        let sim_inc_min: f32 = attr.sim_inc_min;
        let sim_inc_max: f32 = attr.sim_inc_max;
        let sim_freq: f32 = attr.sim_freq;
        let n_canpoints: u32 = attr.n_canpoints;
        let id: String = attr.id;

        let mut value = vec![0.0; n_canpoints as usize];
        
        // initialize value with random values between sim_min and sim_max
        let mut rng = rand::thread_rng();
        eprintln!("{}: {} {} {} {}", id, sim_min, sim_max, sim_inc_min, sim_inc_max);
        for item in value.iter_mut().take(n_canpoints as usize) {
            *item = rng.gen_range(sim_min..sim_max);
            if sim_inc_min != 0.0 {
                *item = (*item / sim_inc_min).round() * sim_inc_min;
            }
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
            id,
        }
    }

    /**
     * Get a random offset within the range of sim_inc_min and sim_inc_max with a random sign.
     * Use sim_inc_min as the offset if sim_inc_min == sim_inc_max.
     * Rounds the offset to the nearest sim_inc_min if sim_inc_min is not 0.
     */
    fn get_rand_offset(&self) -> f32 {
        let mut rng = rand::thread_rng();
        let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let offset: f32 = if self.sim_inc_min == self.sim_inc_max {
            self.sim_inc_min
        } else {
            let rand_offset = rng.gen_range(self.sim_inc_min..self.sim_inc_max);
            if self.sim_inc_min != 0.0 {
                (rand_offset / self.sim_inc_min).round() * self.sim_inc_min
            }
            else {
                rand_offset
            }
        };
        offset * sign
    }

    /**
     * Update the value of the simulated component.
     * Ensures the value is within the range of sim_min and sim_max, and rounds the value to the nearest sim_inc_min if sim_inc_min is not 0.
     */
    pub fn update(&mut self) {
            const MAX_ATTEMPTS: u8 = 10;
            self.last_update = Instant::now();
            for i in 0..self.value.len() {                
                let mut new_value = self.value[i] + self.get_rand_offset();

                // ensuring value is within range AND limit to 5 attempts
                let mut attempts = 0;
                while (new_value < self.sim_min || new_value > self.sim_max) && attempts < MAX_ATTEMPTS {
                    new_value = self.value[i] + self.get_rand_offset();
                    attempts += 1;
                }

                // give up if all attempts failed
                if attempts >= MAX_ATTEMPTS {
                    return; 
                }

                // rounding the new value
                if self.sim_inc_min != 0.0 {
                    new_value =  (new_value / self.sim_inc_min).round() * self.sim_inc_min;
                }

                self.value[i] = new_value;
            }
    }

    pub fn get_data(&self) -> DecodeData {
        DecodeData::new(self.value.clone(), &self.topic, &self.unit)
    }
}
