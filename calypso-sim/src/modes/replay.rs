use std::time::Duration;

use rumqttc::v5::AsyncClient;

use crate::keymap::{claim_topics, load_scenario, run_action};
use crate::registry::SharedRegistry;

/// Deterministic replay (`--play <action>`): load the scenario, run one named
/// action to completion — following its invokes and `sleep_ms` waits — then
/// return. Claims the action's topics first so a running mock heartbeat yields.
pub async fn run(
    client: AsyncClient,
    scenario_path: &str,
    action: &str,
    registry: SharedRegistry,
) -> Result<(), String> {
    let scenario = load_scenario(scenario_path)?;
    if !scenario.contains_key(action) {
        return Err(format!("no action named '{action}' in {scenario_path}"));
    }

    claim_topics(&scenario, std::iter::once(action), &registry).await;

    println!("Playing action '{action}' from {scenario_path} ...");
    println!();

    run_action(&scenario, action, &client, &registry).await;

    // Allow the broker time to flush the last publishes before we return.
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}
