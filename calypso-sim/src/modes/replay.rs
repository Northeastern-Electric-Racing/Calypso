use std::time::Duration;

use rumqttc::v5::AsyncClient;

use crate::keymap::{Scenario, run_action};

/// Deterministic replay (`--play <action>`): run one named action to completion
/// — following its invokes and `sleep_ms` waits — then return. The heartbeat (if
/// running) has already ceded the scenario's topics (see [`crate::ownership`]).
pub async fn run(client: AsyncClient, scenario: Scenario, action: &str) -> Result<(), String> {
    if !scenario.contains_key(action) {
        return Err(format!("no action named '{action}' in the scenario"));
    }

    println!("Playing action '{action}' ...");
    println!();

    run_action(&scenario, action, &client).await;

    // Allow the broker time to flush the last publishes before we return.
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}
