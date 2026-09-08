//! Which topics the mock heartbeat is allowed to publish.
//!
//! This is a *filter*, not an arbitration scheme: it says nothing about who
//! else may publish a topic. The heartbeat and a foreground driver
//! (interactive / replay / stream) are free to publish the same topic — if
//! that matters for what you are testing, mute the heartbeat's copy with
//! `--disable-topic` (or live, see [`crate::modes::control`]).
//!
//! The filter is held in a `watch` channel so it can be replaced while the sim
//! runs; the heartbeat recomputes its active set whenever the value changes.

use regex::RegexSet;

use crate::simulatable_message::SimComponent;

/// A live handle to the heartbeat's filter. Cloneable; every sender writes to
/// the same channel the mock task is watching.
pub type FilterTx = tokio::sync::watch::Sender<FilterMode>;
/// The mock task's read side of [`FilterTx`].
pub type FilterRx = tokio::sync::watch::Receiver<FilterMode>;

/// Which topics the mock heartbeat publishes, from the `--enable-topic` /
/// `--disable-topic` flags (mutually exclusive) or a live update.
///
/// The patterns are held as a [`RegexSet`] rather than a `Vec<Regex>`: matching
/// "any of these" is one fused automaton pass instead of one per pattern, and
/// the set keeps the original pattern strings for [`describe`](Self::describe).
#[derive(Debug, Clone, Default)]
pub enum FilterMode {
    /// No filter — every simulatable topic is allowed.
    #[default]
    Disabled,
    /// Publish everything *except* topics matching these patterns.
    Blacklist(RegexSet),
    /// Publish *only* topics matching these patterns.
    Whitelist(RegexSet),
}

impl FilterMode {
    /// Build from raw patterns, compiling them so a bad regex is rejected
    /// up front rather than silently disabling the heartbeat.
    ///
    /// `disable` wins if both are non-empty; the CLI marks them mutually
    /// exclusive, and the live commands only ever set one.
    pub fn build(enable: &[String], disable: &[String]) -> Result<Self, String> {
        if !disable.is_empty() {
            Ok(Self::Blacklist(compile(disable)?))
        } else if !enable.is_empty() {
            Ok(Self::Whitelist(compile(enable)?))
        } else {
            Ok(Self::Disabled)
        }
    }

    /// Build from the `mode` + `patterns` vocabulary both live control surfaces
    /// speak — `--stream`'s `set_filter` RPC and the plain-`--mock` stdin
    /// commands. Kept here rather than in each caller so the mode names, the
    /// empty-patterns rule, and the error wording are decided once.
    pub fn from_command(mode: &str, patterns: &[String]) -> Result<Self, String> {
        match mode {
            "clear" => Ok(Self::Disabled),
            "disable" | "enable" if patterns.is_empty() => {
                Err(format!("`{mode}` requires at least one pattern"))
            }
            "disable" => Self::build(&[], patterns),
            "enable" => Self::build(patterns, &[]),
            other => Err(format!(
                "unknown mode '{other}': expected \"disable\", \"enable\", or \"clear\""
            )),
        }
    }

    #[must_use]
    pub fn allows(&self, topic: &str) -> bool {
        match self {
            FilterMode::Disabled => true,
            FilterMode::Blacklist(set) => !set.is_match(topic),
            FilterMode::Whitelist(set) => set.is_match(topic),
        }
    }

    /// One-line description for startup and live-update logs.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            FilterMode::Disabled => "no filter (all topics)".to_string(),
            FilterMode::Blacklist(set) => format!("blacklist [{}]", set.patterns().join(", ")),
            FilterMode::Whitelist(set) => format!("whitelist [{}]", set.patterns().join(", ")),
        }
    }

    /// The subset of `components` this filter admits, by index.
    #[must_use]
    pub fn admits(&self, components: &[SimComponent]) -> Vec<bool> {
        components.iter().map(|c| self.allows(&c.name)).collect()
    }
}

fn compile(patterns: &[String]) -> Result<RegexSet, String> {
    RegexSet::new(patterns).map_err(|e| format!("Invalid regex: {e}"))
}
