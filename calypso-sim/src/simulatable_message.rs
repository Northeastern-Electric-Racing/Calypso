#![allow(dead_code)]
// The simulation data model. This lives only in calypso-sim: the main calypso
// decoder never drove the simulate path, so it was removed there when the sim
// was extracted. Some struct fields are read only by the codegen-expanded
// `create_simulated_components` initializer, not by hand-written sim code
// paths — hence the crate-level dead_code allow.

use calypso_cangen::data::DecodeData;
use rand::prelude::*;
use regex::Regex;
use std::sync::LazyLock;
use std::time::Instant;

/**
 * A `SimComponent` roughly corresponds to a `NetField` with properties inherited from `CANMsg`
 */
#[derive(Debug, Clone)]
pub struct SimComponent {
    pub id: String,
    pub points: Vec<SimPoint>,
    pub points_intopic: Option<Vec<SimPoint>>,
    pub unit: String,
    pub name: String,
    pub last_update: Instant,
    pub desc: String,
    pub sim_freq: f32,
}

/**
 * Corresponds to `CANPoint` of a `NetField`
 */
#[derive(Debug, Clone)]
pub struct SimPoint {
    pub size: usize,
    pub parse: Option<bool>,
    pub signed: Option<bool>,
    pub endianness: Option<String>,
    pub default: Option<f32>,
    pub ieee754_f32: Option<bool>,
    pub value: SimValue,
}

/**
 * The mode of simulation and the real-time value of the `CANPoint`
 */
#[derive(Debug, Clone)]
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

impl SimComponent {
    pub fn initialize(&mut self) {
        self.points.iter_mut().for_each(SimPoint::initialize);
        if let Some(points_intopic) = &mut self.points_intopic {
            points_intopic.iter_mut().for_each(SimPoint::initialize);
        }
    }

    #[must_use]
    pub fn should_update(&self) -> bool {
        self.last_update.elapsed().as_millis() > self.sim_freq as u128
    }

    #[must_use]
    pub fn get_decode_data(&self) -> DecodeData {
        let topic_name = topic_values_inject(self);
        DecodeData::new(
            self.points.iter().map(SimPoint::get_value).collect(),
            &topic_name,
            &self.unit,
            None,
        )
    }

    pub fn update(&mut self) {
        self.last_update = Instant::now();
        self.points.iter_mut().for_each(SimPoint::update);
        if let Some(points_intopic) = &mut self.points_intopic {
            points_intopic.iter_mut().for_each(SimPoint::update);
        }
    }
}

impl SimPoint {
    fn initialize(&mut self) {
        match self.default {
            Some(default_val) => match &mut self.value {
                SimValue::Range { current, .. } | SimValue::Discrete { current, .. } => {
                    *current = default_val;
                }
            },
            None => self.value.initialize(),
        }
    }

    #[must_use]
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
                let sampled = Self::sample_range_or_low(&mut rng, *min, *max, f32::EPSILON);
                *current = Self::quantize(sampled, *inc_min, *round);
            }
            SimValue::Discrete { options, current } => {
                // `choose` is empty-safe (returns None); direct indexing panics.
                if let Some(&(v, _)) = options.choose(&mut rng) {
                    *current = v;
                }
            }
        }
    }

    #[must_use]
    pub fn get_value(&self) -> f32 {
        match self {
            SimValue::Range { current, .. } | SimValue::Discrete { current, .. } => *current,
        }
    }

    /// Snap `value` to the nearest multiple of `inc_min` (a no-op when `inc_min`
    /// is 0), then optionally round to a whole number.
    fn quantize(value: f32, inc_min: f32, round: bool) -> f32 {
        let snapped = if inc_min == 0.0 {
            value
        } else {
            (value / inc_min).round() * inc_min
        };
        if round { snapped.round() } else { snapped }
    }

    /// Sample uniformly from `lo..hi`, falling back to `lo` when the range is
    /// empty or inverted — `random_range` panics on those, and the spec
    /// validator rejects neither. `eps` is the width below which the range is
    /// treated as degenerate.
    fn sample_range_or_low(rng: &mut impl Rng, lo: f32, hi: f32, eps: f32) -> f32 {
        if hi - lo > eps {
            rng.random_range(lo..hi)
        } else {
            lo
        }
    }

    /// A random offset in `inc_min..inc_max` with a random sign, snapped to a
    /// multiple of `inc_min` (or just `inc_min` when the range is degenerate).
    fn get_rand_offset(inc_min: f32, inc_max: f32) -> f32 {
        let mut rng = rand::rng();
        let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
        let sampled = Self::sample_range_or_low(&mut rng, inc_min, inc_max, 0.0001);
        Self::quantize(sampled, inc_min, false) * sign
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

                let mut new_value = *current + SimValue::get_rand_offset(*inc_min, *inc_max);
                let mut attempts = 0;
                while (new_value < *min || new_value > *max) && attempts < MAX_ATTEMPTS {
                    new_value = *current + SimValue::get_rand_offset(*inc_min, *inc_max);
                    attempts += 1;
                }

                // Range too tight to land in; keep the current value.
                if attempts >= MAX_ATTEMPTS {
                    return;
                }

                *current = Self::quantize(new_value, *inc_min, *round);
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

/// Placeholder pattern (`{}`) for in-topic value injection, compiled once and reused.
static PLACEHOLDER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{\}").unwrap());

/**
 * This helper function takes a `SimComponent`, injects the associated `CANPoint` values into the topic string
 * e.g. "Hello/{}/World/{}" -> "Hello/{4}/World{5}"
 *
 * # Panics
 *  Panics if Regex compilation fails.
 *
 */
#[must_use]
pub fn topic_values_inject(component: &SimComponent) -> String {
    if let Some(points_intopic) = &component.points_intopic {
        let component_name = &component.name;
        // check: placeholder count lines up with in point vector array length
        if points_intopic.len() != PLACEHOLDER_RE.find_iter(component_name).count() {
            eprintln!(
                "[error] in-topic points vector length does not line up with placeholder count"
            );
            return component_name.clone();
        }

        // Replace {} placeholders with values, pulling each in-topic point's
        // value on the fly (no intermediate Vec).
        let mut values = points_intopic.iter().map(|p| p.get_value() as u32);
        PLACEHOLDER_RE
            .replace_all(component_name, |_: &regex::Captures| {
                values.next().map_or("{}".to_string(), |v| v.to_string())
            })
            .into_owned()
    } else {
        component.name.clone()
    }
}
