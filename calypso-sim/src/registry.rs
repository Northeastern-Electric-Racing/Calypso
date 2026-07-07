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
