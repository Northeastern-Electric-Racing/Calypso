use super::data::DecodeData;
use rand::prelude::*;
use regex::Regex;
use std::time::Instant;

/********************* SIMULATE_MESSAGE.H *********************/

/**
 * A SimComponent roughly corresponds to a NetField with properties inherited from CANMsg
 */
#[derive(Debug)]
pub struct SimComponent {
    pub id: String,
    pub points: Vec<SimPoint>,
    pub points_intopic: Option<Vec<SimPoint>>,
    pub unit: String,
    pub name: String,
    pub last_update: Instant,
    pub desc: String,
    pub key: Option<String>, // the "FirstOff" and "SecondOff" thing
    pub is_ext: Option<bool>,
    pub sim_freq: f32,
}

/**
 * Corresponds to CANPoint of a NetField
 */
#[derive(Debug)]
pub struct SimPoint {
    pub size: usize,
    pub parse: Option<bool>,
    pub signed: Option<bool>,
    pub endianness: Option<String>,
    pub format: Option<String>,
    pub default: Option<f32>,
    pub ieee754_f32: Option<bool>,
    pub value: SimValue,
}

/**
 * The mode of simulation and the real-time value of the CANPoint
 */
#[derive(Debug)]
pub enum SimValue {
    /// Ranged mode where the value is within a min/max range and can include increment parameters.
    Range {
        min: f32,
        max: f32,
        inc_min: f32,
        inc_max: f32,
        round: bool,
        current: f32, // current value in range mode
    },
    /// Options mode where the value is selected from a set of predefined options.
    Discrete {
        options: Vec<(f32, f32)>, // List of option pairs.
        current: f32,             // currently selected option
    },
}

/********************* SIMULATE_MESSAGE.C *********************/

impl SimComponent {
    pub fn initialize(&mut self) {
        self.points.iter_mut().for_each(|p| p.initialize());
        if let Some(points_intopic) = &mut self.points_intopic {
            points_intopic.iter_mut().for_each(|p| p.initialize());
        }
    }

    pub fn should_update(&self) -> bool {
        self.last_update.elapsed().as_millis() > self.sim_freq as u128
    }

    pub fn get_decode_data(&self) -> DecodeData {
        let topic_name = topic_values_inject(self);
        DecodeData::new(
            self.points.iter().map(|p| p.get_value()).collect(),
            &topic_name,
            &self.unit,
        )
    }

    pub fn update(&mut self) {
        self.last_update = Instant::now();
        self.points.iter_mut().for_each(|p| p.update());
        if let Some(points_intopic) = &mut self.points_intopic {
            points_intopic.iter_mut().for_each(|p| p.update());
        }
    }
}

impl SimPoint {
    fn initialize(&mut self) {
        match self.default {
            Some(default_val) => match &mut self.value {
                SimValue::Range { current, .. } => *current = default_val,
                SimValue::Discrete { current, .. } => *current = default_val,
            },
            None => self.value.initialize(),
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value.get_value()
    }

    fn update(&mut self) {
        self.value.update();
    }
}

impl SimValue {
    pub fn initialize(&mut self) {
        let mut rng = rand::rng();
        match self {
            SimValue::Range {
                min,
                max,
                inc_min,
                round,
                current,
                ..
            } => {
                *current = rng.random_range(*min..*max);
                if *inc_min != 0.0 {
                    *current = (*current / *inc_min).round() * *inc_min; // Round to nearest inc_min
                }
                if *round {
                    *current = current.round(); // Round to nearest whole number
                }
            }
            SimValue::Discrete { options, current } => {
                let idx = rng.random_range(0..options.len());
                *current = options[idx].0;
            }
        }
    }

    pub fn get_value(&self) -> f32 {
        match self {
            SimValue::Range { current, .. } => *current,
            SimValue::Discrete { current, .. } => *current,
        }
    }

    /**  
     * Get a random offset within the range of sim_inc_min and sim_inc_max with a random sign.
     * Use sim_inc_min as the offset if sim_inc_min == sim_inc_max.
     * Rounds the offset to the nearest sim_inc_min if sim_inc_min is not 0.
     */
    fn get_rand_offset(inc_min: f32, inc_max: f32) -> f32 {
        let mut rng = rand::rng();
        let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

        let offset: f32 = if inc_min == inc_max {
            inc_min
        } else {
            let rand_offset = rng.random_range(inc_min..inc_max);
            if inc_min != 0.0 {
                (rand_offset / inc_min).round() * inc_min
            } else {
                rand_offset
            }
        };
        offset * sign
    }

    fn update(&mut self) {
        match self {
            SimValue::Range {
                min,
                max,
                inc_min,
                inc_max,
                round,
                current,
            } => {
                const MAX_ATTEMPTS: u8 = 10;

                let cur = *current;
                let min_val = *min;
                let max_val = *max;
                let inc_min_val = *inc_min;
                let inc_max_val = *inc_max;

                // First, call get_rand_offset without partially borrowing each field
                // let mut new_value = cur + SimValue::get_rand_offset(inc_min_val, inc_max_val);
                let mut new_value = cur + SimValue::get_rand_offset(inc_min_val, inc_max_val);

                let mut attempts = 0;
                while (new_value < min_val || new_value > max_val) && attempts < MAX_ATTEMPTS {
                    new_value = cur + SimValue::get_rand_offset(inc_min_val, inc_max_val);
                    attempts += 1;
                }

                if attempts >= MAX_ATTEMPTS {
                    return;
                }

                if inc_min_val != 0.0 {
                    new_value = (new_value / inc_min_val).round() * inc_min_val;
                }

                if *round {
                    new_value = new_value.round();
                }

                *current = new_value;
            }
            SimValue::Discrete { options, current } => {
                let mut rng = rand::rng();
                let prob = rng.random_range(0f32..1f32);
                let mut new_value = None;

                for i in 0..options.len() {
                    let prob_floor = if i == 0 { 0f32 } else { options[i - 1].1 };
                    let prob_ceiling = options[i].1;
                    if prob >= prob_floor && prob <= prob_ceiling {
                        new_value = Some(options[i].0);
                        break;
                    }
                }

                *current = new_value.unwrap_or(-1f32);
            }
        }
    }
}

/**
 * This helper function takes a SimComponent, injects the associated CANPoint values into the topic string
 * e.g. "Hello/{}/World/{}" -> "Hello/{4}/World{5}"
 */
pub fn topic_values_inject(component: &SimComponent) -> String {
    if let Some(points_intopic) = &component.points_intopic {
        let component_name = &component.name;
        // check: placeholder count lines up with in point vector array length
        let re = Regex::new(r"\{\}").unwrap();
        if points_intopic.len() != re.find_iter(component_name).count() {
            eprintln!(
                "[error] in-topic points vector length does not line up with placeholder count"
            );
            return component_name.to_string();
        }
        let in_topic_values: Vec<u32> = points_intopic
            .iter()
            .map(|p| p.get_value() as u32)
            .collect();

        // Replace {} placeholders with values
        let mut value_iter = in_topic_values.iter();
        re.replace_all(component_name, |_: &regex::Captures| {
            value_iter
                .next()
                .map_or("{}".to_string(), |val| val.to_string())
        })
        .into_owned()
    } else {
        component.name.to_string()
    }
}
