use std::time::Duration;

use rumqttc::v5::AsyncClient;

use crate::keymap::{claim_keymap_topics, load_states, publish_injection};
use crate::registry::SharedRegistry;

/// Run a scripted sequence from a text file, then return. Each line is
/// either a single character (fires that key from the keymap) or
/// `sleep <ms>`. Blank lines and lines starting with `#` are ignored.
pub async fn run(
    client: AsyncClient,
    key_map_path: &str,
    script_path: &str,
    registry: SharedRegistry,
) -> Result<(), String> {
    let mut states = load_states(key_map_path)?;
    claim_keymap_topics(&states, &registry).await;

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
            // Deterministic replay: a malformed directive aborts the run rather
            // than silently skipping it and throwing off downstream timing.
            let ms: u64 = rest.trim().parse().map_err(|_| {
                format!(
                    "script: line {}: bad sleep arg '{}'",
                    lineno + 1,
                    rest.trim()
                )
            })?;
            tokio::time::sleep(Duration::from_millis(ms)).await;
            continue;
        }
        let mut chars = line.chars();
        let Some(ch) = chars.next() else { continue };
        if chars.next().is_some() {
            return Err(format!(
                "script: line {}: expected single char or 'sleep N', got '{line}'",
                lineno + 1
            ));
        }
        let Some(state) = states.get_mut(&ch) else {
            return Err(format!(
                "script: line {}: no binding for key '{ch}'",
                lineno + 1
            ));
        };
        publish_injection(ch, state, &client, &registry).await;
    }

    // Allow broker time to flush.
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}
