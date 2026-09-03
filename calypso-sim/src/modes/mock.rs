use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use crate::publish::{Transport, publish_data};
use crate::simulatable_message::SimComponent;

/// Background task: every 5ms, walk `components` and publish any that are due
/// (per `sim_freq`).
///
/// `components` is already the heartbeat's share of the topic space — the
/// startup [`crate::ownership::Partition`] has removed anything a driver owns —
/// so there is no per-publish ownership check here.
pub async fn run(
    token: CancellationToken,
    transport: Transport,
    mut components: Vec<SimComponent>,
) {
    if components.is_empty() {
        info!("Mock: no components to simulate.");
    } else {
        info!("Mock: simulating {} components", components.len());
    }

    let mut interval = tokio::time::interval(Duration::from_millis(5));

    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Mock: shutting down.");
                break;
            }
            _ = interval.tick() => publish_due(&mut components, &transport).await,
        }
    }
}

/// Publish every component that is due for an update (per its `sim_freq`),
/// advancing its simulated value first.
async fn publish_due(components: &mut [SimComponent], transport: &Transport) {
    for component in components {
        if !component.should_update() {
            continue;
        }
        component.update();
        let data = component.get_decode_data();
        if let Err(e) = publish_data(transport, &data.topic, &data.unit, &data.value).await {
            warn!("Mock publish failed for {}: {e}", data.topic);
        }
    }
}
