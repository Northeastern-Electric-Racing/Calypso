use std::time::Duration;

use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use crate::filter::{FilterMode, FilterRx};
use crate::publish::{Transport, publish_data};
use crate::runtime_topic::{SimCommand, SimCommandRx};
use crate::simulatable_message::SimComponent;

/// Background task: every 5ms, walk `components` and publish any that are both
/// admitted by the current filter and due for an update (per `sim_freq`).
///
/// `components` is the full simulatable set; the filter decides which of them
/// are live. It can change while running (see [`crate::modes::control`]), so
/// the admitted set is recomputed on change rather than per tick — a regex
/// match per component per 5ms tick would be ~72k matches/sec for nothing.
///
/// This is a filter, not an ownership claim: a foreground driver may publish
/// the same topics. Mute the heartbeat's copy if that matters.
///
/// The set itself can also grow and shrink at runtime via `cmd_rx` (see
/// [`crate::runtime_topic`]). This task owns the list, so it is also the only
/// place that can answer "is this name already taken".
pub async fn run(
    token: CancellationToken,
    transport: Transport,
    mut components: Vec<SimComponent>,
    mut filter_rx: FilterRx,
    mut cmd_rx: SimCommandRx,
) {
    let mut admitted = filter_rx.borrow_and_update().admits(&components);
    log_active(&components, &admitted, &filter_rx.borrow());

    let mut interval = tokio::time::interval(Duration::from_millis(5));

    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Mock: shutting down.");
                break;
            }
            // The sender lives as long as the process, so a receive error here
            // is not reachable in practice; treat it as "stop watching".
            changed = filter_rx.changed() => {
                if changed.is_err() {
                    break;
                }
                let filter = filter_rx.borrow_and_update().clone();
                admitted = filter.admits(&components);
                log_active(&components, &admitted, &filter);
            }
            Some(cmd) = cmd_rx.recv() => {
                // A list is a read; only a mutation needs the set recomputed.
                if let SimCommand::List { reply } = cmd {
                    let snapshot = components
                        .iter()
                        .map(|c| (c.name.clone(), c.unit.clone()))
                        .collect();
                    let _ = reply.send(snapshot);
                } else {
                    apply(&mut components, cmd);
                    let filter = filter_rx.borrow().clone();
                    admitted = filter.admits(&components);
                    log_active(&components, &admitted, &filter);
                }
            }
            _ = interval.tick() => publish_due(&mut components, &admitted, &transport).await,
        }
    }
}

/// Apply one add/remove to the component list and answer the caller.
///
/// A dropped reply channel means the caller gave up waiting; the mutation still
/// stands, so there is nothing to do about it.
fn apply(components: &mut Vec<SimComponent>, cmd: SimCommand) {
    match cmd {
        SimCommand::Add { component, reply } => {
            let result = if components.iter().any(|c| c.name == component.name) {
                Err(format!("topic '{}' is already simulated", component.name))
            } else {
                info!("Mock: adding topic {}", component.name);
                components.push(*component);
                Ok(())
            };
            let _ = reply.send(result);
        }
        SimCommand::Remove { name, reply } => {
            let before = components.len();
            // Removes every component under this name. Names are not unique in
            // the generated set — the in-topic placeholder topics repeat — so
            // the count goes back to the caller rather than being assumed to be 1.
            components.retain(|c| c.name != name);
            let removed = before - components.len();
            if removed > 0 {
                info!("Mock: removed {removed} component(s) for topic {name}");
            }
            let _ = reply.send(removed);
        }
        // Handled by the caller, which does not need the recompute below.
        SimCommand::List { .. } => unreachable!("List is answered before apply"),
    }
}

/// How many active topics to name before logging just the count. Naming a
/// filtered-down set is the useful case; naming all ~360 buries the log.
const MAX_NAMED: usize = 25;

/// Log which topics the heartbeat is driving, so the active set is visible at
/// startup and after every live filter change.
fn log_active(components: &[SimComponent], admitted: &[bool], filter: &FilterMode) {
    let live: Vec<&str> = components
        .iter()
        .zip(admitted)
        .filter_map(|(c, ok)| ok.then_some(c.name.as_str()))
        .collect();

    let total = components.len();
    let filter = filter.describe();

    if live.is_empty() {
        info!("Mock: 0 of {total} topics active — {filter}");
    } else if live.len() <= MAX_NAMED {
        info!(
            "Mock: {} of {total} topics active — {filter}: {}",
            live.len(),
            live.join(", ")
        );
    } else {
        info!(
            "Mock: {} of {total} topics active — {filter} (too many to list; \
             narrow the filter or see --list-topics)",
            live.len()
        );
    }
}

/// Publish every admitted component that is due for an update (per its
/// `sim_freq`), advancing its simulated value first.
async fn publish_due(components: &mut [SimComponent], admitted: &[bool], transport: &Transport) {
    for (component, admitted) in components.iter_mut().zip(admitted) {
        if !admitted || !component.should_update() {
            continue;
        }
        component.update();
        let data = component.get_decode_data();
        if let Err(e) = publish_data(transport, &data.topic, &data.unit, &data.value).await {
            warn!("Mock publish failed for {}: {e}", data.topic);
        }
    }
}
