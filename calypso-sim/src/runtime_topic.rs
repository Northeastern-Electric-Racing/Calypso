//! Adding and removing simulated topics while the sim runs.
//!
//! The heartbeat's topic set is otherwise fixed at compile time: the
//! `gen_simulate_data!()` macro expands the CAN spec into a `Vec<SimComponent>`
//! literal, and a message with no `sim_freq` is skipped wholesale — so most of
//! the spec's fields are absent from the binary entirely. The
//! [`crate::filter`] can only mask components that already exist, so it cannot
//! reach them. This module can: it builds a [`SimComponent`] from caller-supplied
//! parameters at runtime.
//!
//! Parameters come from the caller rather than the CAN spec on purpose. The
//! runtime Docker image ships only the binaries, so there is no spec to read
//! where the sim actually runs, and most unsimulated fields are missing a
//! frequency, a range, or both — there would be little to read even if there were.
//!
//! The value shapes mirror the spec's own `sim` block, so a definition can be
//! copied straight out of Odyssey-Definitions:
//!
//! ```json
//! {"min": -550, "max": 550, "inc_min": 1, "inc_max": 50}
//! {"options": [[1, 0.1], [2, 0.5], [7, 0.4]]}
//! ```

use std::time::Instant;

use serde::Deserialize;
use tokio::sync::{mpsc, oneshot};

use crate::simulatable_message::{SimComponent, SimPoint, SimValue};

/// A mutation of the mock heartbeat's component list.
///
/// These are *events*, so they travel over an `mpsc` rather than the `watch`
/// the filter uses: `watch` keeps only the latest value, which would silently
/// drop the first of two adds in flight.
///
/// Each carries a `oneshot` reply because only the mock task holds the
/// component list — duplicate detection and the removed count cannot be
/// answered by the caller.
#[derive(Debug)]
pub enum SimCommand {
    Add {
        /// Boxed: `SimComponent` is large, and clippy flags the size disparity
        /// between this variant and `Remove` otherwise.
        component: Box<SimComponent>,
        /// `Err` when a component with the same name is already simulated.
        reply: oneshot::Sender<Result<(), String>>,
    },
    Remove {
        name: String,
        /// How many components were dropped — see [`build`]'s note on names
        /// not being unique.
        reply: oneshot::Sender<usize>,
    },
    /// Snapshot the live `(name, unit)` pairs. Needed because the compiled
    /// component list is only the starting point once topics can be added.
    List {
        reply: oneshot::Sender<Vec<(String, String)>>,
    },
}

pub type SimCommandTx = mpsc::Sender<SimCommand>;
pub type SimCommandRx = mpsc::Receiver<SimCommand>;

/// Sent when the mock task is not running, so nothing holds the receiver.
pub const NO_MOCK: &str = "mock heartbeat is not running; start the sim with --mock";

/// Hand a command to the mock task and wait for its answer.
///
/// `make` receives the reply channel so each caller names only the variant it
/// wants; the channel pairing and the "heartbeat isn't running" mapping live
/// here rather than in every control surface. A failed send and a dropped reply
/// mean the same thing in practice — nothing holds the receiver, so there is no
/// component list to read or mutate.
pub async fn request<T>(
    cmd_tx: &SimCommandTx,
    make: impl FnOnce(oneshot::Sender<T>) -> SimCommand,
) -> Result<T, String> {
    let (reply, answer) = oneshot::channel();
    cmd_tx
        .send(make(reply))
        .await
        .map_err(|_| NO_MOCK.to_string())?;
    answer.await.map_err(|_| NO_MOCK.to_string())
}

/// A topic to start simulating, as supplied by a caller.
#[derive(Debug, Deserialize)]
pub struct TopicSpec {
    pub topic: String,
    #[serde(default)]
    pub unit: String,
    pub sim_freq: f32,
    pub sim: ValueSpec,
}

/// How to make up a value each tick. Shape-disambiguated: `Range` needs all four
/// bounds, `Discrete` needs `options`, so there is no order-dependent matching.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValueSpec {
    /// Drift continuously within `min..max` — sensors.
    Range {
        min: f32,
        max: f32,
        inc_min: f32,
        inc_max: f32,
        #[serde(default)]
        round: bool,
    },
    /// Pick from a fixed set — states, modes, fault flags. `[value, weight]`,
    /// exactly as the CAN spec writes it; [`build`] converts the weights.
    Discrete { options: Vec<(f32, f32)> },
}

/// Build a runnable [`SimComponent`], or explain why the spec is unusable.
///
/// Note the built component is not tied to any CAN message: only `name`, `unit`,
/// `sim_freq` and the point's value reach the publish path, so the CAN-shape
/// fields (`size`, `signed`, `endianness`, …) are left empty. They exist for the
/// shared codegen and are dead weight in this crate, which has no encode path.
pub fn build(spec: TopicSpec) -> Result<SimComponent, String> {
    let name = spec.topic.trim().to_string();
    if name.is_empty() {
        return Err("topic must not be empty".into());
    }
    // `topic_values_inject` substitutes `{}` from a parallel `points_intopic`
    // vector that a runtime topic has no way to supply. Left unset, the topic
    // publishes with a literal `{}` in it and says nothing about why.
    if name.contains("{}") {
        return Err(format!(
            "topic '{name}' contains '{{}}': in-topic placeholders are not supported for runtime topics"
        ));
    }
    if !spec.sim_freq.is_finite() || spec.sim_freq <= 0.0 {
        // `should_update` is `elapsed > sim_freq`, so 0 is "due every tick"
        // (a 5ms firehose), not "never".
        return Err(format!(
            "sim_freq must be a positive number of milliseconds, got {}",
            spec.sim_freq
        ));
    }

    let value = build_value(spec.sim)?;

    let mut component = SimComponent {
        id: "runtime".to_string(),
        points: vec![SimPoint {
            size: 0,
            parse: None,
            signed: None,
            endianness: None,
            // Must stay None: a `Some` overwrites `current` without clamping it
            // into range, and a `current` outside `min..max` can never move
            // again — `update` gives up after 10 failed attempts and returns.
            default: None,
            ieee754_f32: None,
            value,
        }],
        points_intopic: None,
        unit: spec.unit,
        name,
        last_update: Instant::now(),
        desc: "added at runtime".to_string(),
        sim_freq: spec.sim_freq,
    };
    // Nothing else in this crate calls `initialize` — only the generated
    // component list does. Without it `current` stays at 0.0, outside the range
    // for any topic whose minimum is above zero.
    component.initialize();
    Ok(component)
}

fn build_value(spec: ValueSpec) -> Result<SimValue, String> {
    match spec {
        ValueSpec::Range {
            min,
            max,
            inc_min,
            inc_max,
            round,
        } => {
            // Degenerate and inverted ranges are already handled downstream, but
            // a non-finite span is not: the sampler only tests `max - min > eps`,
            // so an infinite span reaches `random_range`, which panics on
            // non-finite bounds. `f32::MIN..f32::MAX` overflows to infinity too,
            // hence checking the span and not just the bounds.
            for (label, v) in [
                ("min", min),
                ("max", max),
                ("inc_min", inc_min),
                ("inc_max", inc_max),
            ] {
                if !v.is_finite() {
                    return Err(format!("{label} must be finite, got {v}"));
                }
            }
            if !(max - min).is_finite() {
                return Err(format!(
                    "range {min}..{max} is too wide to sample; narrow it to a finite span"
                ));
            }
            if min > max {
                return Err(format!("min ({min}) must not exceed max ({max})"));
            }
            if inc_min < 0.0 {
                return Err(format!("inc_min must not be negative, got {inc_min}"));
            }
            if inc_max < inc_min {
                return Err(format!(
                    "inc_max ({inc_max}) must not be below inc_min ({inc_min})"
                ));
            }
            Ok(SimValue::Range {
                min,
                max,
                inc_min,
                inc_max,
                round,
                current: 0.0,
            })
        }
        ValueSpec::Discrete { options } => {
            if options.is_empty() {
                return Err("options must not be empty".into());
            }
            for (value, weight) in &options {
                if !value.is_finite() {
                    return Err(format!("option value must be finite, got {value}"));
                }
                if !weight.is_finite() || *weight < 0.0 {
                    return Err(format!(
                        "option weight must be finite and non-negative, got {weight}"
                    ));
                }
            }
            let total: f32 = options.iter().map(|(_, w)| *w).sum();
            if total <= 0.0 {
                return Err("option weights must sum to more than zero".into());
            }
            Ok(SimValue::Discrete {
                options: accumulate(&options, total),
                current: 0.0,
            })
        }
    }
}

/// Convert `[value, weight]` pairs into the running ceilings `SimValue::Discrete`
/// expects — the same prescan the codegen does when expanding the CAN spec:
///
/// ```text
/// [(0, 0.3), (1, 0.3), (2, 0.4)]  ->  [(0, 0.3), (1, 0.6), (2, 1.0)]
/// ```
///
/// Unlike the codegen this also normalises by the total, so the last ceiling is
/// always 1.0. `update` picks a value by finding the first ceiling above a
/// `0..1` sample and yields -1.0 when none is, so weights that sum to less than
/// one would otherwise publish -1.0 for the shortfall.
fn accumulate(options: &[(f32, f32)], total: f32) -> Vec<(f32, f32)> {
    let mut running = 0.0;
    let last = options.len() - 1;
    options
        .iter()
        .enumerate()
        .map(|(i, (value, weight))| {
            running += weight / total;
            // Pin the final ceiling rather than trusting the accumulated
            // division to land exactly on 1.0.
            (*value, if i == last { 1.0 } else { running })
        })
        .collect()
}
