use std::time::Duration;

use crate::simulate_data::create_simulated_components;
use regex::Regex;
use rumqttc::v5::AsyncClient;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

use crate::publish::publish_data;
use crate::registry::{Owner, SharedRegistry};

#[derive(Debug)]
enum FilterMode {
    Disabled,
    Blacklist(Vec<Regex>),
    Whitelist(Vec<Regex>),
}

impl FilterMode {
    fn build(enable: &[String], disable: &[String]) -> Result<Self, String> {
        if !disable.is_empty() {
            Ok(Self::Blacklist(compile_patterns(disable)?))
        } else if !enable.is_empty() {
            Ok(Self::Whitelist(compile_patterns(enable)?))
        } else {
            Ok(Self::Disabled)
        }
    }

    fn allows(&self, topic: &str) -> bool {
        match self {
            FilterMode::Disabled => true,
            FilterMode::Blacklist(p) => !p.iter().any(|re| re.is_match(topic)),
            FilterMode::Whitelist(p) => p.iter().any(|re| re.is_match(topic)),
        }
    }
}

fn compile_patterns(patterns: &[String]) -> Result<Vec<Regex>, String> {
    patterns
        .iter()
        .map(|p| Regex::new(p).map_err(|e| format!("Invalid regex '{p}': {e}")))
        .collect()
}

/// Background task: every 5ms, walk the simulated components and publish any
/// that are due (per `sim_freq`) AND owned by `Owner::Auto` in the registry.
///
/// Components owned by `Stream` or `Silenced` are skipped without advancing
/// internal state, so they pick up where they would have been on `release`.
pub async fn run(
    token: CancellationToken,
    client: AsyncClient,
    registry: SharedRegistry,
    enable: Vec<String>,
    disable: Vec<String>,
) {
    let filter = match FilterMode::build(&enable, &disable) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Autonomous mode: {err}");
            return;
        }
    };

    let mut components: Vec<_> = create_simulated_components()
        .into_iter()
        .filter(|c| filter.allows(&c.name))
        .collect();

    if components.is_empty() {
        info!("Autonomous: no components match the filter; nothing to simulate.");
    } else {
        info!("Autonomous: simulating {} components", components.len());
    }

    let mut interval = tokio::time::interval(Duration::from_millis(5));

    loop {
        tokio::select! {
            () = token.cancelled() => {
                debug!("Autonomous: shutting down.");
                break;
            }
            _ = interval.tick() => {
                for component in &mut components {
                    if !component.should_update() {
                        continue;
                    }
                    let owner = registry.read().await.owner(&component.name);
                    if owner != Owner::Auto {
                        continue;
                    }
                    component.update();
                    let data = component.get_decode_data();
                    if let Err(e) = publish_data(&client, &data.topic, &data.unit, &data.value).await {
                        warn!("Autonomous publish failed for {}: {e}", data.topic);
                    }
                }
            }
        }
    }
}
