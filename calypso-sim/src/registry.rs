use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// Default state — autonomous heartbeat publishes this topic.
    Auto,
    /// Claimed by a streaming/keymap client; autonomous skips.
    Stream,
    /// Nobody publishes; both autonomous and stream skip.
    Silenced,
}

impl Owner {
    pub fn as_str(self) -> &'static str {
        match self {
            Owner::Auto => "auto",
            Owner::Stream => "stream",
            Owner::Silenced => "silenced",
        }
    }
}

pub type SharedRegistry = Arc<RwLock<TopicRegistry>>;

/// Per-topic ownership. Topics not in the map default to `Owner::Auto`.
#[derive(Debug, Default)]
pub struct TopicRegistry {
    overrides: HashMap<String, Owner>,
}

impl TopicRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn shared() -> SharedRegistry {
        Arc::new(RwLock::new(Self::new()))
    }

    pub fn owner(&self, topic: &str) -> Owner {
        self.overrides.get(topic).copied().unwrap_or(Owner::Auto)
    }

    /// Whether the autonomous heartbeat may publish `topic`. The heartbeat
    /// only drives a topic while it is still `Auto`-owned; once a stream or
    /// keymap driver claims or silences it, the heartbeat yields.
    pub fn auto_may_publish(&self, topic: &str) -> bool {
        self.owner(topic) == Owner::Auto
    }

    /// Whether a stream/keymap driver may publish `topic`. Drivers may publish
    /// anything except a topic that has been explicitly `Silenced`.
    pub fn driver_may_publish(&self, topic: &str) -> bool {
        self.owner(topic) != Owner::Silenced
    }

    /// Set ownership; returns the previous owner. Setting back to `Auto`
    /// removes the override entirely.
    pub fn set(&mut self, topic: &str, owner: Owner) -> Owner {
        let prev = self.owner(topic);
        if owner == Owner::Auto {
            self.overrides.remove(topic);
        } else {
            self.overrides.insert(topic.to_string(), owner);
        }
        prev
    }

    /// Snapshot of all non-`Auto` topic overrides, sorted by topic name.
    pub fn snapshot(&self) -> Vec<(String, Owner)> {
        let mut entries: Vec<_> = self
            .overrides
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_makes_auto_yield_but_keeps_the_driver() {
        let mut reg = TopicRegistry::new();
        // Unmapped topics default to Auto — heartbeat and driver both publish.
        assert_eq!(reg.owner("T"), Owner::Auto);
        assert!(reg.auto_may_publish("T") && reg.driver_may_publish("T"));

        assert_eq!(
            reg.set("T", Owner::Stream),
            Owner::Auto,
            "prev owner reported"
        );
        assert_eq!(reg.owner("T"), Owner::Stream);
        assert!(
            !reg.auto_may_publish("T"),
            "heartbeat yields a claimed topic"
        );
        assert!(reg.driver_may_publish("T"), "driver still owns it");
    }

    #[test]
    fn silence_blocks_both_auto_and_driver() {
        let mut reg = TopicRegistry::new();
        reg.set("T", Owner::Silenced);
        assert!(!reg.auto_may_publish("T"));
        assert!(!reg.driver_may_publish("T"));
    }

    #[test]
    fn releasing_to_auto_clears_the_override() {
        let mut reg = TopicRegistry::new();
        reg.set("T", Owner::Stream);
        assert_eq!(
            reg.set("T", Owner::Auto),
            Owner::Stream,
            "prev owner reported"
        );
        assert_eq!(reg.owner("T"), Owner::Auto);
        assert!(reg.snapshot().is_empty(), "auto topics carry no override");
    }
}
