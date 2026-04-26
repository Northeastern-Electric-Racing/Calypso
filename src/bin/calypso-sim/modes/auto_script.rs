use std::time::Duration;

use rumqttc::v5::AsyncClient;

use crate::keymap::{KeyMode, build_topic_states, load_key_map, publish_injection};
use crate::registry::{Owner, SharedRegistry};

/// Run a scripted sequence from a text file, then return. Each line is
/// either a single character (fires that key from the keymap) or
/// `sleep <ms>`. Blank lines and lines starting with `#` are ignored.
pub async fn run(
    client: AsyncClient,
    key_map_path: &str,
    script_path: &str,
    registry: SharedRegistry,
) -> Result<(), String> {
    let key_map = load_key_map(key_map_path)?;
    if key_map.is_empty() {
        return Err("Key map is empty".into());
    }
    let mut states = build_topic_states(key_map);
    if states.is_empty() {
        return Err("No matching topics found for any key mapping".into());
    }

    {
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

    let script = std::fs::read_to_string(script_path)
        .map_err(|e| format!("Failed to read script '{script_path}': {e}"))?;

    println!("Running scripted sequence from {script_path} ...");
    println!();

    for (lineno, raw_line) in script.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("sleep ") {
            match rest.trim().parse::<u64>() {
                Ok(ms) => tokio::time::sleep(Duration::from_millis(ms)).await,
                Err(_) => eprintln!("script: line {}: bad sleep arg '{rest}'", lineno + 1),
            }
            continue;
        }
        let mut chars = line.chars();
        let Some(ch) = chars.next() else { continue };
        if chars.next().is_some() {
            eprintln!(
                "script: line {}: expected single char or 'sleep N', got '{line}'",
                lineno + 1
            );
            continue;
        }
        if let Some(state) = states.get_mut(&ch) {
            publish_injection(ch, state, &client, &registry).await;
        } else {
            eprintln!("script: line {}: no binding for key '{ch}'", lineno + 1);
        }
    }

    // Allow broker time to flush.
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}
