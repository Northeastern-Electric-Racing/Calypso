#![allow(dead_code)]
// The simulation data model. `create_simulated_components` (generated in
// `simulate_data.rs`) builds one `SimComponent` per simulated topic from the CAN
// spec. Some fields mirror the spec but are never read, hence the dead_code allow.

use rand::prelude::*;
use std::time::Instant;

/**
 * A `SimComponent` roughly corresponds to a `NetField` with properties inherited from `CANMsg`
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
    pub sim_freq: f32,
}

/**
 * Corresponds to `CANPoint` of a `NetField`
 */
#[derive(Debug)]
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

    /// The topic to publish on, with any in-topic values injected.
    #[must_use]
    pub fn topic(&self) -> String {
        topic_values_inject(self)
    }

    /// The current value of each point, in order.
    #[must_use]
    pub fn values(&self) -> Vec<f32> {
        self.points.iter().map(SimPoint::get_value).collect()
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
                *current = Self::clamp(Self::quantize(sampled, *inc_min, *round), *min, *max);
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

    /// Clamp `value` to `min..=max`, leaving it as is when the bounds are
    /// inverted or NaN (`f32::clamp` panics on those).
    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if min <= max {
            value.clamp(min, max)
        } else {
            value
        }
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

    pub(crate) fn update(&mut self) {
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
                if new_value < *min || new_value > *max {
                    return;
                }

                *current = Self::clamp(Self::quantize(new_value, *inc_min, *round), *min, *max);
            }
            SimValue::Discrete { options, current } => {
                let mut rng = rand::rng();
                let prob = rng.random_range(0f32..1f32);
                // Ceilings that sum to slightly under 1.0 (float rounding in
                // the spec's weights) leave a gap at the top: fall back to
                // the last option rather than publishing a bogus value.
                *current = options
                    .iter()
                    .find(|(_, ceiling)| prob <= *ceiling)
                    .or(options.last())
                    .map_or(-1f32, |(value, _)| *value);
            }
        }
    }
}

/// Fill each `{}` in the component's name with the matching in-topic point's
/// value, e.g. `"Hello/{}/World/{}"` -> `"Hello/4/World/5"`. Returns the name
/// unchanged if the placeholder and point counts differ.
#[must_use]
pub fn topic_values_inject(component: &SimComponent) -> String {
    let name = &component.name;
    let Some(points_intopic) = &component.points_intopic else {
        return name.clone();
    };
    if points_intopic.len() != name.matches("{}").count() {
        eprintln!("[error] in-topic points vector length does not line up with placeholder count");
        return name.clone();
    }

    let mut parts = name.split("{}");
    let mut topic = parts.next().unwrap_or_default().to_string();
    for (point, part) in points_intopic.iter().zip(parts) {
        topic.push_str(&(point.get_value() as u32).to_string());
        topic.push_str(part);
    }
    topic
}
