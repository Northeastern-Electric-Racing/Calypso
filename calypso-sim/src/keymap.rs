use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;

use crate::simulatable_message::{SimComponent, SimValue};
use crate::simulate_data::create_simulated_components;
use rumqttc::v5::AsyncClient;
use serde::Deserialize;

use crate::publish::publish_data;
use crate::raw_mode::line_end;
use crate::registry::{Owner, SharedRegistry};

/// Build the topic states from a keymap file, erroring if the resulting set
/// is empty.
pub fn load_states(key_map_path: &str) -> Result<HashMap<char, KeyState>, String> {
    let key_map = load_key_map(key_map_path)?;
    if key_map.is_empty() {
        return Err("Key map is empty".into());
    }
    let states = build_topic_states(key_map);
    if states.is_empty() {
        return Err("No matching topics found for any key mapping".into());
    }
    Ok(states)
}

/// Claim every topic referenced by `states` for `Owner::Stream` so the
/// autonomous heartbeat (if running) yields ownership.
pub async fn claim_keymap_topics(states: &HashMap<char, KeyState>, registry: &SharedRegistry) {
    let mut reg = registry.write().await;
    for state in states.values() {
        match &state.mode {
            KeyMode::Sequence { steps } => {
                for step in steps {
                    reg.set(&step.topic, Owner::Stream);
                }
            }
            _ => {
                reg.set(&state.topic, Owner::Stream);
            }
        }
    }
}

/// A keymap entry. Four forms:
/// * Bare topic string — random value within sim bounds (requires the topic
///   to be in the auto-generated simulated-components list).
/// * Object with `value` — pins that exact number on every keypress.
///   `unit` is required for topics not in the generated simulated-components
///   list.
/// * Object with `step` — increment mode. Publishes `value` (or `min`, or 0)
///   on first press, then advances by `step` each press, wrapping
///   independently when each bound is supplied. As with pinned mode, a known
///   topic uses the sim component's unit; the `unit` field applies only to
///   topics not in the generated simulated-components list.
/// * Object with `sequence` — publishes a scripted series of (topic, value)
///   pairs on each keypress, with optional per-step `delay_ms` before publish.
///
/// Every object form also accepts an optional `desc` string that is shown in
/// the startup listing and inline with each publish log line.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KeyEntry {
    TopicOnly(String),
    /// Sequence is distinguished by the `sequence` field — check first.
    Sequence {
        sequence: Vec<SequenceStep>,
        #[serde(default)]
        desc: Option<String>,
    },
    /// Checked before Pinned: if `step` is present, this is increment mode.
    Increment {
        topic: String,
        value: Option<f32>,
        step: f32,
        min: Option<f32>,
        max: Option<f32>,
        unit: Option<String>,
        #[serde(default)]
        desc: Option<String>,
    },
    Pinned {
        topic: String,
        value: f32,
        unit: Option<String>,
        #[serde(default)]
        desc: Option<String>,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct SequenceStep {
    pub topic: String,
    pub value: f32,
    pub unit: Option<String>,
    /// Milliseconds to wait *before* publishing this step. Defaults to 0.
    #[serde(default)]
    pub delay_ms: u64,
}

#[derive(Debug, Clone)]
pub enum KeyMode {
    Random,
    Pinned {
        value: f32,
    },
    Increment {
        current: f32,
        step: f32,
        min: Option<f32>,
        max: Option<f32>,
    },
    Sequence {
        steps: Vec<SequenceStep>,
    },
}

#[derive(Debug, Clone)]
pub struct KeyState {
    pub topic: String,
    pub unit: String,
    pub component: Option<SimComponent>,
    pub mode: KeyMode,
    pub desc: Option<String>,
}

/// Format `" [unit]"` suffix, or empty string when the unit is empty/missing.
pub fn unit_suffix(unit: &str) -> String {
    if unit.is_empty() {
        String::new()
    } else {
        format!(" [{unit}]")
    }
}

/// Format `"  — desc"` suffix, or empty string when desc is missing/empty.
pub fn desc_suffix(desc: Option<&str>) -> String {
    desc.filter(|d| !d.is_empty())
        .map(|d| format!("  — {d}"))
        .unwrap_or_default()
}

pub fn parse_key_map(content: &str) -> Result<HashMap<char, KeyEntry>, String> {
    let raw: HashMap<String, KeyEntry> =
        serde_json::from_str(content).map_err(|e| format!("Invalid key map JSON: {e}"))?;
    raw.into_iter()
        .map(|(key_str, entry)| {
            let mut chars = key_str.chars();
            let (Some(ch), None) = (chars.next(), chars.next()) else {
                return Err(format!(
                    "Key mapping keys must be single characters, got: '{key_str}'"
                ));
            };
            Ok((ch, entry))
        })
        .collect()
}

pub fn load_key_map(path: &str) -> Result<HashMap<char, KeyEntry>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read key map file '{path}': {e}"))?;
    parse_key_map(&content)
}

pub fn build_topic_states(key_map: HashMap<char, KeyEntry>) -> HashMap<char, KeyState> {
    let components = create_simulated_components();
    let mut result = HashMap::new();
    for (key, entry) in key_map {
        let (topic, mode, unit_override, desc) = match entry {
            KeyEntry::TopicOnly(t) => (t, KeyMode::Random, None, None),
            KeyEntry::Pinned {
                topic,
                value,
                unit,
                desc,
            } => (topic, KeyMode::Pinned { value }, unit, desc),
            KeyEntry::Increment {
                topic,
                value,
                step,
                min,
                max,
                unit,
                desc,
            } => {
                let start = value.or(min).unwrap_or(0.0);
                (
                    topic,
                    KeyMode::Increment {
                        current: start,
                        step,
                        min,
                        max,
                    },
                    unit,
                    desc,
                )
            }
            KeyEntry::Sequence { sequence, desc } => {
                if sequence.is_empty() {
                    eprintln!("Warning: sequence for key '{key}' is empty — skipping");
                    continue;
                }
                let summary_topic = format!("<sequence of {} steps>", sequence.len());
                (
                    summary_topic,
                    KeyMode::Sequence { steps: sequence },
                    None,
                    desc,
                )
            }
        };

        let component = components.iter().find(|c| c.name == topic).cloned();

        if matches!(mode, KeyMode::Random) && component.is_none() {
            eprintln!(
                "Warning: random-mode key '{key}' maps to topic '{topic}' which is \
                 not in the generated sim components — skipping"
            );
            continue;
        }

        let unit = match component.as_ref().map(|c| c.unit.clone()).or(unit_override) {
            Some(u) => u,
            // Sequence mode publishes per-step topics with their own units;
            // the top-level unit is unused, so missing it is fine.
            None if matches!(mode, KeyMode::Sequence { .. }) => String::new(),
            None => {
                eprintln!(
                    "Warning: key '{key}' maps to unknown topic '{topic}' \
                     with no `unit` provided — skipping"
                );
                continue;
            }
        };

        result.insert(
            key,
            KeyState {
                topic,
                unit,
                component,
                mode,
                desc,
            },
        );
    }
    result
}

/// Generate a fresh random value within each point's defined bounds. Delegates
/// to `SimValue::initialize` (which ignores any `default` and always
/// randomizes), then clamps `Range` values back into `[min, max]` in case the
/// inc/round snapping pushed them just outside.
pub fn randomize_component(component: &mut SimComponent) {
    for point in &mut component.points {
        point.value.initialize();
        if let SimValue::Range {
            min, max, current, ..
        } = &mut point.value
        {
            *current = current.clamp(*min, *max);
        }
    }
}

/// Advance an increment-mode state and return the value to publish *before*
/// the advance (so the first press emits the starting value).
pub fn advance_increment(current: &mut f32, step: f32, min: Option<f32>, max: Option<f32>) -> f32 {
    let emitted = *current;
    let mut next = *current + step;
    if let Some(hi) = max
        && next > hi
    {
        next = min.unwrap_or(hi);
    }
    if let Some(lo) = min
        && next < lo
    {
        next = max.unwrap_or(lo);
    }
    *current = next;
    emitted
}

/// Resolve the (topic, unit, values) for single-shot modes. Sequence mode
/// returns `None` and is handled by the caller.
fn resolve_single_publish(state: &mut KeyState) -> Option<(String, String, Vec<f32>)> {
    match &mut state.mode {
        KeyMode::Random => {
            let component = state.component.as_mut()?;
            randomize_component(component);
            let data = component.get_decode_data();
            Some((data.topic, data.unit, data.value))
        }
        KeyMode::Pinned { value } => Some((state.topic.clone(), state.unit.clone(), vec![*value])),
        KeyMode::Increment {
            current,
            step,
            min,
            max,
        } => {
            let v = advance_increment(current, *step, *min, *max);
            Some((state.topic.clone(), state.unit.clone(), vec![v]))
        }
        KeyMode::Sequence { .. } => None,
    }
}

/// Resolve the value(s) for this keypress and publish to the broker.
/// Sequence mode walks the scripted steps with per-step delays; other modes
/// emit a single message. Logs each publish to stdout.
///
/// Topics owned by `Silenced` in the registry are skipped silently.
pub async fn publish_injection(
    ch: char,
    state: &mut KeyState,
    client: &AsyncClient,
    registry: &SharedRegistry,
) {
    if let KeyMode::Sequence { steps } = &state.mode {
        if let Some(desc) = state.desc.as_deref() {
            let nl = line_end();
            print!("[{ch}] {desc}{nl}");
            let _ = std::io::stdout().flush();
        }
        let steps = steps.clone();
        for step in steps {
            if !registry.read().await.driver_may_publish(&step.topic) {
                continue;
            }
            if step.delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(step.delay_ms)).await;
            }
            let unit = step.unit.clone().unwrap_or_default();
            log_and_publish(ch, &step.topic, &unit, &[step.value], None, client).await;
        }
        return;
    }

    if !registry.read().await.driver_may_publish(&state.topic) {
        return;
    }

    if let Some((topic, unit, values)) = resolve_single_publish(state) {
        log_and_publish(ch, &topic, &unit, &values, state.desc.as_deref(), client).await;
    }
}

async fn log_and_publish(
    ch: char,
    topic: &str,
    unit: &str,
    values: &[f32],
    desc: Option<&str>,
    client: &AsyncClient,
) {
    let nl = line_end();
    let desc_s = desc_suffix(desc);

    match publish_data(client, topic, unit, values).await {
        Ok(_) => {
            let values_str: Vec<String> = values.iter().map(|v| format!("{v:.2}")).collect();
            print!(
                "[{ch}] {topic} = [{}] {unit}{desc_s}{nl}",
                values_str.join(", ")
            );
        }
        Err(e) => {
            print!("[{ch}] error publishing {topic}: {e}{nl}");
        }
    }
    let _ = std::io::stdout().flush();
}
