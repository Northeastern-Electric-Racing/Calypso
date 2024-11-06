use super::data::DecodeData;
use rand::prelude::*;
use std::time::Instant;

/// Base properties of every simulated message
pub struct SimulatedComponent {
    value: Vec<f32>,      // DecodeData.value
    topic: String,        // DecodeData.topic
    unit: String,         // DecodeData.unit
    last_update: Instant, // when the data was last updated
    #[allow(dead_code)]
    n_canpoints: u32, // number of can points
    sim_freq: f32,        // Frequency in ms
    // format: String,       // e.g. "divide10"
    #[allow(dead_code)]
    id: String, // e.g. "0x80" (or should this be a uint32?)
                // signed: bool,         // is the value signed?
                // size: u8,            // size of the value in bits
}

/// A wrapper struct for giving properties of a message from the macros to simulator
pub struct SimulatedComponentAttr {
    pub sim_freq: f32,
    pub n_canpoints: u32,
    pub id: String,
}

/// A simulation mode where a value is selected from a list at a given frequency
pub struct SimEnum {
    data: SimulatedComponent,
    /// note the frequency here is not an individual pecentage but a running total of the probability
    options: Vec<[f32; 2]>,
}

/// A simulation mode where a value is choosen within a range with a range of possible increments
pub struct SimSweep {
    data: SimulatedComponent,
    min: f32,
    max: f32,
    inc_min: f32,
    inc_max: f32,
    round: bool,
}

/// Shared functions of a simulator
pub trait SimShared {
    fn update(&mut self);

    fn fetch_data(&self) -> &SimulatedComponent;
}

/// Base implementations of a simulator
impl SimulatedComponent {
    pub fn should_update(&self) -> bool {
        self.last_update.elapsed().as_millis() > self.sim_freq as u128
    }

    pub fn get_data(&self) -> DecodeData {
        DecodeData::new(self.value.clone(), &self.topic, &self.unit)
    }
}

/// Sweep specific logic
impl SimSweep {
    pub fn new(
        topic: String,
        unit: String,
        attr: SimulatedComponentAttr,
        sweep_settings: (f32, f32, f32, f32), // min, max, inc_min, inc_max
        round: bool,
    ) -> Self {
        let sim_freq: f32 = attr.sim_freq;
        let n_canpoints: u32 = attr.n_canpoints;
        let id: String = attr.id;

        let mut value = vec![0.0; n_canpoints as usize];

        // initialize value with random values between sim_min and sim_max
        let mut rng = rand::thread_rng();
        for item in value.iter_mut().take(n_canpoints as usize) {
            *item = rng.gen_range(sweep_settings.0..sweep_settings.1);
            if sweep_settings.2 != 0.0 {
                *item = (*item / sweep_settings.2).round() * sweep_settings.2;
            }
        }

        Self {
            data: SimulatedComponent {
                value,
                topic,
                unit,
                last_update: Instant::now(),
                n_canpoints,
                sim_freq,
                id,
            },
            min: sweep_settings.0,
            max: sweep_settings.1,
            inc_min: sweep_settings.2,
            inc_max: sweep_settings.3,
            round,
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
        let offset: f32 = if self.inc_min == self.inc_max {
            self.inc_min
        } else {
            let rand_offset = rng.gen_range(self.inc_min..self.inc_max);
            if self.inc_min != 0.0 {
                (rand_offset / self.inc_min).round() * self.inc_min
            } else {
                rand_offset
            }
        };
        offset * sign
    }
}

/// Base logic impl for sweeped sim
impl SimShared for SimSweep {
    /**
     * Update the value of the simulated component.
     * Ensures the value is within the range of sim_min and sim_max, and rounds the value to the nearest sim_inc_min if sim_inc_min is not 0.
     */
    fn update(&mut self) {
        const MAX_ATTEMPTS: u8 = 10;
        self.data.last_update = Instant::now();
        for i in 0..self.data.value.len() {
            let mut new_value = self.data.value[i] + self.get_rand_offset();

            // ensuring value is within range AND limit to 5 attempts
            let mut attempts = 0;
            while (new_value < self.min || new_value > self.max) && attempts < MAX_ATTEMPTS {
                new_value = self.data.value[i] + self.get_rand_offset();
                attempts += 1;
            }

            // give up if all attempts failed
            if attempts >= MAX_ATTEMPTS {
                return;
            }

            // rounding the new value
            if self.inc_min != 0.0 {
                new_value = (new_value / self.inc_min).round() * self.inc_min;
            }

            // additional rounding override to whole number if enabled
            if self.round {
                new_value = new_value.round();
            }

            self.data.value[i] = new_value;
        }
    }

    fn fetch_data(&self) -> &SimulatedComponent {
        &self.data
    }
}

/// Enum specific logic
impl SimEnum {
    pub fn new(
        topic: String,
        unit: String,
        attr: SimulatedComponentAttr,
        enum_ls: Vec<[f32; 2]>, // value, probability (summed to 1)
    ) -> Self {
        let sim_freq: f32 = attr.sim_freq;
        let n_canpoints: u32 = attr.n_canpoints;
        let id: String = attr.id;

        // placeholder value
        let value = vec![0.0; n_canpoints as usize];

        Self {
            data: SimulatedComponent {
                value,
                topic,
                unit,
                last_update: Instant::now(),
                n_canpoints,
                sim_freq,
                id,
            },
            options: enum_ls,
        }
    }
}

/// Base logic for enum sim
impl SimShared for SimEnum {
    fn update(&mut self) {
        for i in 0..self.data.value.len() {
            let prob = rand::thread_rng().gen_range(0f32..1f32);
            let mut new_value: Option<f32> = None;
            for i in 0..self.options.len() {
                let prev_opts_add = if i == 0 { 0f32 } else { self.options[i - 1][1] };
                if prob <= self.options[i][1] && prev_opts_add <= prob {
                    new_value = Some(self.options[i][0]);
                }
            }
            self.data.value[i] = new_value.unwrap_or(-1f32);
        }
    }

    fn fetch_data(&self) -> &SimulatedComponent {
        &self.data
    }
}
