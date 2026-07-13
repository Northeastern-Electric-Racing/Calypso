//! Who owns what — resolved once, at startup, and never renegotiated.
//!
//! The mock heartbeat and the foreground driver (interactive / replay / stream)
//! can both publish, but they must never fight over the same topic. Rather than
//! arbitrate at runtime, we partition topics up front: any topic the driver owns
//! is simply removed from the heartbeat's set here, so the two publish disjoint
//! sets by construction.
//!
//! A driver's owned topics are:
//! * `--key-map` / `--play`: the topics its scenario publishes (known at load);
//! * `--stream`: nothing is auto-reserved (its topics aren't known up front) —
//!   carve topics out of the heartbeat explicitly with `--disable-topic`.

use std::collections::BTreeSet;

use regex::Regex;

use crate::simulatable_message::SimComponent;
use crate::simulate_data::create_simulated_components;

/// Which topics the mock heartbeat is allowed to publish, from the
/// `--enable-topic` / `--disable-topic` flags (mutually exclusive).
#[derive(Debug)]
pub enum FilterMode {
    /// No filter — every simulatable topic is allowed.
    Disabled,
    /// Publish everything *except* topics matching these patterns.
    Blacklist(Vec<Regex>),
    /// Publish *only* topics matching these patterns.
    Whitelist(Vec<Regex>),
}

impl FilterMode {
    /// Build from the raw CLI patterns, compiling them so a bad regex fails
    /// fast at startup instead of silently disabling the heartbeat.
    pub fn build(enable: &[String], disable: &[String]) -> Result<Self, String> {
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

/// The startup topic partition: the components the mock heartbeat will drive,
/// and the topics reserved for the driver (which the heartbeat skips).
pub struct Partition {
    /// Components the mock heartbeat will publish.
    pub heartbeat: Vec<SimComponent>,
    /// Topics reserved for the foreground driver; the heartbeat is off on these.
    pub driver_owned: BTreeSet<String>,
}

impl Partition {
    /// Resolve the partition: the heartbeat drives every simulatable topic the
    /// `filter` allows and the driver does not own. Driver ownership wins, so
    /// the two sides are always disjoint.
    #[must_use]
    pub fn resolve(filter: &FilterMode, driver_owned: BTreeSet<String>) -> Self {
        let heartbeat = create_simulated_components()
            .into_iter()
            .filter(|c| filter.allows(&c.name) && !driver_owned.contains(&c.name))
            .collect();
        Self {
            heartbeat,
            driver_owned,
        }
    }

    /// Print the split to stderr so it is clear, before anything publishes,
    /// exactly what the heartbeat drives and what it has ceded to the driver.
    pub fn print_summary(&self) {
        if self.driver_owned.is_empty() {
            eprintln!(
                "Ownership: mock heartbeat drives {} topic(s); none reserved for a driver.",
                self.heartbeat.len()
            );
        } else {
            let reserved: Vec<&str> = self.driver_owned.iter().map(String::as_str).collect();
            eprintln!(
                "Ownership: mock heartbeat drives {} topic(s); {} reserved for the driver \
                 (heartbeat off): {}",
                self.heartbeat.len(),
                reserved.len(),
                reserved.join(", ")
            );
        }
    }
}
