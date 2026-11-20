use std::time::Duration;

use crate::keymap::{Scenario, run_action};
use zenoh::Session;

/// Deterministic replay (`--play <action>`): run one named action to completion
/// — following its invokes and `sleep_ms` waits — then return. The heartbeat, if
/// running, may publish some of the same topics; mute its copy with
/// `--disable-topic` if that matters (see [`crate::filter`]).
pub async fn run(session: Session, scenario: Scenario, action: &str) -> Result<(), String> {
    if !scenario.contains_key(action) {
        return Err(format!("no action named '{action}' in the scenario"));
    }

    println!("Playing action '{action}' ...");
    println!();

    run_action(&scenario, action, &session).await;

    // Give Zenoh a moment to flush the last puts before the session drops.
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}
