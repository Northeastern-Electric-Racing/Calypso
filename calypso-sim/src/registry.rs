use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// Default state — mock heartbeat publishes this topic.
    Mock,
    /// Claimed by a streaming/keymap client; mock skips.
    Stream,
    /// Nobody publishes; both mock and stream skip.
    Silenced,
}

impl Owner {
    pub fn as_str(self) -> &'static str {
        match self {
            Owner::Mock => "mock",
            Owner::Stream => "stream",
            Owner::Silenced => "silenced",
        }
    }
}

pub type SharedRegistry = Arc<RwLock<TopicRegistry>>;

/// Per-topic ownership. Topics not in the map default to `Owner::Mock`.
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
        self.overrides.get(topic).copied().unwrap_or(Owner::Mock)
    }

    /// Whether the mock heartbeat may publish `topic`. The heartbeat
    /// only drives a topic while it is still `Mock`-owned; once a stream or
    /// keymap driver claims or silences it, the heartbeat yields.
    pub fn mock_may_publish(&self, topic: &str) -> bool {
        self.owner(topic) == Owner::Mock
    }

    /// Whether a stream/keymap driver may publish `topic`. Drivers may publish
    /// anything except a topic that has been explicitly `Silenced`.
    pub fn driver_may_publish(&self, topic: &str) -> bool {
        self.owner(topic) != Owner::Silenced
    }

    /// Set ownership; returns the previous owner. Setting back to `Mock`
    /// removes the override entirely.
    pub fn set(&mut self, topic: &str, owner: Owner) -> Owner {
        let prev = self.owner(topic);
        if owner == Owner::Mock {
            self.overrides.remove(topic);
        } else {
            self.overrides.insert(topic.to_string(), owner);
        }
        prev
    }

    /// Snapshot of all non-`Mock` topic overrides, sorted by topic name.
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
