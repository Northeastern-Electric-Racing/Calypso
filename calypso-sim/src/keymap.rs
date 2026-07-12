//! Loading, validating, and running scenarios. The plain data types live in
//! the [`model`] submodule and are re-exported here, so every existing
//! `crate::keymap::…` path keeps working.

mod model;
pub use model::{Scenario, Step};

use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::time::Duration;

use rumqttc::v5::AsyncClient;

use crate::publish::publish_data;
use crate::raw_mode::line_end;
use crate::registry::{Owner, SharedRegistry};

/// Parse a scenario from JSON. Does not validate — call [`validate`] (or use
/// [`load_scenario`], which does both) before running it.
pub fn parse_scenario(content: &str) -> Result<Scenario, String> {
    serde_json::from_str(content).map_err(|e| format!("Invalid scenario JSON: {e}"))
}

/// Read a scenario file from `path`, parse it, and validate it.
pub fn load_scenario(path: &str) -> Result<Scenario, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read scenario file '{path}': {e}"))?;
    let scenario = parse_scenario(&content)?;
    validate(&scenario)?;
    Ok(scenario)
}

/// Validate a scenario up front so a bad file fails fast with a clear message
/// instead of misbehaving mid-run:
/// * the scenario is non-empty,
/// * every publish step sets exactly one of `value` / `values`,
/// * every invoke step names an action that exists, and
/// * the invoke graph is acyclic, so every action terminates.
pub fn validate(scenario: &Scenario) -> Result<(), String> {
    if scenario.is_empty() {
        return Err("Scenario is empty".into());
    }
    for (name, action) in scenario {
        for step in &action.steps {
            match step {
                Step::Invoke(target) if !scenario.contains_key(target) => {
                    return Err(format!("action '{name}' invokes unknown action '{target}'"));
                }
                Step::Publish {
                    topic,
                    value,
                    values,
                    ..
                } => check_publish(name, topic, value.as_ref(), values.as_deref())?,
                Step::Invoke(_) | Step::Sleep { .. } => {}
            }
        }
    }
    for name in scenario.keys() {
        detect_cycle(scenario, name, &mut Vec::new())?;
    }
    Ok(())
}

fn check_publish(
    action: &str,
    topic: &str,
    value: Option<&f32>,
    values: Option<&[f32]>,
) -> Result<(), String> {
    match (value, values) {
        (Some(_), Some(_)) => Err(format!(
            "action '{action}': step for '{topic}' sets both `value` and `values`"
        )),
        (None, None) => Err(format!(
            "action '{action}': step for '{topic}' sets neither `value` nor `values`"
        )),
        (None, Some([])) => Err(format!(
            "action '{action}': step for '{topic}' has empty `values`"
        )),
        _ => Ok(()),
    }
}

/// Depth-first search tracking the current invoke path; a name already on the
/// path is a cycle. Runs before any action executes, so [`flatten`] can then
/// recurse without a visited-set.
fn detect_cycle(scenario: &Scenario, name: &str, path: &mut Vec<String>) -> Result<(), String> {
    if path.iter().any(|n| n == name) {
        path.push(name.to_string());
        return Err(format!("cyclic action invocation: {}", path.join(" -> ")));
    }
    path.push(name.to_string());
    if let Some(action) = scenario.get(name) {
        for step in &action.steps {
            if let Step::Invoke(target) = step {
                detect_cycle(scenario, target, path)?;
            }
        }
    }
    path.pop();
    Ok(())
}

/// A flattened, executable step: invokes have been resolved away, leaving only
/// publishes and waits.
enum Prim {
    Publish {
        topic: String,
        values: Vec<f32>,
        unit: String,
    },
    Sleep(u64),
}

/// Expand `name` into a linear list of [`Prim`]s, inlining every invoked
/// action. Safe against infinite recursion because [`validate`] has already
/// proven the graph acyclic and every invoke target present.
fn flatten(scenario: &Scenario, name: &str, out: &mut Vec<Prim>) {
    let Some(action) = scenario.get(name) else {
        return;
    };
    for step in &action.steps {
        match step {
            Step::Invoke(target) => flatten(scenario, target, out),
            Step::Publish {
                topic,
                value,
                values,
                unit,
            } => out.push(Prim::Publish {
                topic: topic.clone(),
                values: values
                    .clone()
                    .or_else(|| value.map(|v| vec![v]))
                    .unwrap_or_default(),
                unit: unit.clone().unwrap_or_default(),
            }),
            Step::Sleep { sleep_ms } => out.push(Prim::Sleep(*sleep_ms)),
        }
    }
}

/// Every topic `name` publishes to, following invokes. Used to claim ownership
/// so the mock heartbeat yields those topics.
#[must_use]
pub fn collect_topics(scenario: &Scenario, name: &str) -> BTreeSet<String> {
    let mut prims = Vec::new();
    flatten(scenario, name, &mut prims);
    prims
        .into_iter()
        .filter_map(|p| match p {
            Prim::Publish { topic, .. } => Some(topic),
            Prim::Sleep(_) => None,
        })
        .collect()
}

/// The `key -> action name` bindings for interactive mode, erroring if two
/// actions claim the same key.
pub fn key_bindings(scenario: &Scenario) -> Result<HashMap<char, String>, String> {
    let mut map = HashMap::new();
    for (name, action) in scenario {
        if let Some(key) = action.key
            && let Some(prev) = map.insert(key, name.clone())
        {
            return Err(format!(
                "key '{key}' is bound to both '{prev}' and '{name}'"
            ));
        }
    }
    Ok(map)
}

/// Claim every topic reachable from `action_names` for [`Owner::Stream`], so a
/// running mock heartbeat yields those topics to this driver.
pub async fn claim_topics<'a>(
    scenario: &Scenario,
    action_names: impl IntoIterator<Item = &'a str>,
    registry: &SharedRegistry,
) {
    let mut topics = BTreeSet::new();
    for name in action_names {
        topics.extend(collect_topics(scenario, name));
    }
    let mut reg = registry.write().await;
    for topic in &topics {
        reg.set(topic, Owner::Stream);
    }
}

/// Run `name`'s steps in order: publishes go to the broker (skipping any topic
/// the registry has silenced), sleeps wait, invokes are inlined. Logs each
/// publish to stdout.
pub async fn run_action(
    scenario: &Scenario,
    name: &str,
    client: &AsyncClient,
    registry: &SharedRegistry,
) {
    if let Some(desc) = scenario.get(name).and_then(|a| a.desc.as_deref()) {
        print!("[{name}] {desc}{}", line_end());
        let _ = std::io::stdout().flush();
    }

    let mut prims = Vec::new();
    flatten(scenario, name, &mut prims);
    for prim in prims {
        match prim {
            Prim::Sleep(ms) if ms > 0 => tokio::time::sleep(Duration::from_millis(ms)).await,
            Prim::Sleep(_) => {}
            Prim::Publish {
                topic,
                values,
                unit,
            } => {
                if registry.read().await.driver_may_publish(&topic) {
                    log_and_publish(name, &topic, &unit, &values, client).await;
                }
            }
        }
    }
}

async fn log_and_publish(
    action: &str,
    topic: &str,
    unit: &str,
    values: &[f32],
    client: &AsyncClient,
) {
    let nl = line_end();
    let unit_s = if unit.is_empty() {
        String::new()
    } else {
        format!(" {unit}")
    };
    match publish_data(client, topic, unit, values).await {
        Ok(_) => {
            let vals: Vec<String> = values.iter().map(|v| format!("{v:.2}")).collect();
            print!("[{action}] {topic} = [{}]{unit_s}{nl}", vals.join(", "));
        }
        Err(e) => print!("[{action}] error publishing {topic}: {e}{nl}"),
    }
    let _ = std::io::stdout().flush();
}

/// Print the interactive key listing: each key-bound action with its step
/// count and description.
pub fn print_listing(scenario: &Scenario, keys: &HashMap<char, String>) {
    println!("Key bindings:");
    let mut bound: Vec<(char, &String)> = keys.iter().map(|(k, name)| (*k, name)).collect();
    bound.sort_unstable_by_key(|(k, _)| *k);
    for (key, name) in bound {
        let action = &scenario[name];
        let desc = action
            .desc
            .as_deref()
            .filter(|d| !d.is_empty())
            .map(|d| format!(" — {d}"))
            .unwrap_or_default();
        let n = action.steps.len();
        let plural = if n == 1 { "" } else { "s" };
        println!("  {key} → {name} ({n} step{plural}){desc}");
    }
    println!();
}
