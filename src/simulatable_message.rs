use super::data::DecodeData;
use rand::prelude::*;
use std::time::Instant;

/// Base properties of every simulated message
pub struct SimComponent {
    value: Vec<f32>,      // DecodeData.value
    topic: String,        // DecodeData.topic
    unit: String,         // DecodeData.unit
    last_update: Instant, // when the data was last updated
    #[allow(dead_code)]
    n_canpoints: u32, // number of can points
    sim_freq: f32,        // Frequency in ms
    #[allow(dead_code)]
    id: String, // e.g. "0x80"

                          // signed: bool,        // is the value signed?
                          // size: u8,            // size of the value in bits
                          // format: String,      // e.g. "divide10"
}

/// A wrapper struct for giving properties of a message from the macros to simulator
pub struct SimComponentAttr {
    pub sim_freq: f32,
    pub n_canpoints: u32,
    pub id: String,
}

/// A simulation mode where a value is selected from a list at a given frequency
pub struct SimEnum {
    component: SimComponent,
    /// note the frequency here is not an individual pecentage but a running total of the probability
    enum_ls: Vec<[f32; 2]>,
}

/// A simulation mode where a value is choosen within a range with a range of possible increments
pub struct SimSweep {
    component: SimComponent,
    min: f32,
    max: f32,
    inc_min: f32,
    inc_max: f32,
    round: bool,
}

/// Shared functions of a simulator
pub trait SimShared {
    fn update(&mut self);
    fn should_update(&self) -> bool {
        self.get_component().should_update()
    }
    fn get_value(&self) -> DecodeData {
        self.get_component().get_value()
    }
    fn get_component(&self) -> &SimComponent;
}

/// Base implementations of a simulator
impl SimComponent {
    pub fn should_update(&self) -> bool {
        self.last_update.elapsed().as_millis() > self.sim_freq as u128
    }

    pub fn get_value(&self) -> DecodeData {
        DecodeData::new(self.value.clone(), &self.topic, &self.unit)
    }
}

/// Sweep specific logic
impl SimSweep {
    pub fn new(
        topic: String,
        unit: String,
        attr: SimComponentAttr,
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
            component: SimComponent {
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
        self.component.last_update = Instant::now();
        for i in 0..self.component.value.len() {
            let mut new_value = self.component.value[i] + self.get_rand_offset();

            // ensuring value is within range AND limit to 10 attempts
            let mut attempts = 0;
            while (new_value < self.min || new_value > self.max) && attempts < MAX_ATTEMPTS {
                new_value = self.component.value[i] + self.get_rand_offset();
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

            self.component.value[i] = new_value;
        }
    }

    fn get_component(&self) -> &SimComponent {
        &self.component
    }
}

/// Enum specific logic
impl SimEnum {
    pub fn new(
        topic: String,
        unit: String,
        attr: SimComponentAttr,
        enum_ls: Vec<[f32; 2]>, // <value, probability> (summed to 1)
    ) -> Self {
        let sim_freq: f32 = attr.sim_freq;
        let n_canpoints: u32 = attr.n_canpoints;
        let id: String = attr.id;

        // placeholder value
        let value = vec![0.0; n_canpoints as usize];

        Self {
            component: SimComponent {
                value,
                topic,
                unit,
                last_update: Instant::now(),
                n_canpoints,
                sim_freq,
                id,
            },
            enum_ls,
        }
    }
}

/// Base logic for enum sim
impl SimShared for SimEnum {
    fn update(&mut self) {
        self.component.last_update = Instant::now();
        let mut rng = rand::thread_rng();
        for i in 0..self.component.value.len() {
            let prob = rng.gen_range(0f32..1f32);
            let mut new_value: Option<f32> = None;
            for i in 0..self.enum_ls.len() {
                let prob_floor = if i == 0 { 0f32 } else { self.enum_ls[i - 1][1] };
                let prob_ceiling = self.enum_ls[i][1];
                if prob >= prob_floor && prob <= prob_ceiling {
                    new_value = Some(self.enum_ls[i][0]);
                    break;
                }
            }
            self.component.value[i] = new_value.unwrap_or(-1f32);
        }
    }

    fn get_component(&self) -> &SimComponent {
        &self.component
    }
}
