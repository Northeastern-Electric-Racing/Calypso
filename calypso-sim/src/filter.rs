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

use regex::Regex;

use crate::simulatable_message::SimComponent;

/// A live handle to the heartbeat's filter. Cloneable; every sender writes to
/// the same channel the mock task is watching.
pub type FilterTx = tokio::sync::watch::Sender<FilterMode>;
/// The mock task's read side of [`FilterTx`].
pub type FilterRx = tokio::sync::watch::Receiver<FilterMode>;

/// Which topics the mock heartbeat publishes, from the `--enable-topic` /
/// `--disable-topic` flags (mutually exclusive) or a live update.
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
    /// Build from the `--enable-topic` / `--disable-topic` flags. clap makes
    /// them mutually exclusive, so at most one is non-empty.
    pub fn build(enable: &[String], disable: &[String]) -> Result<Self, String> {
        if !enable.is_empty() {
            Self::from_command("enable", enable)
        } else if !disable.is_empty() {
            Self::from_command("disable", disable)
        } else {
            Ok(Self::Disabled)
        }
    }

    /// Build from a live filter command — `mode` is `enable`, `disable` or
    /// `clear` — as sent on the `--mock` control stdin or via `set_filter`.
    /// Patterns are compiled up front, so a bad regex is rejected rather than
    /// silently disabling the heartbeat.
    pub fn from_command(mode: &str, patterns: &[String]) -> Result<Self, String> {
        match mode {
            "clear" => Ok(Self::Disabled),
            "enable" | "disable" if patterns.is_empty() => {
                Err(format!("`{mode}` needs at least one regex"))
            }
            "enable" => Ok(Self::Whitelist(compile_patterns(patterns)?)),
            "disable" => Ok(Self::Blacklist(compile_patterns(patterns)?)),
            other => Err(format!(
                "unknown mode '{other}': expected \"disable\", \"enable\", or \"clear\""
            )),
        }
    }

    #[must_use]
    pub fn allows(&self, topic: &str) -> bool {
        match self {
            FilterMode::Disabled => true,
            FilterMode::Blacklist(p) => !p.iter().any(|re| re.is_match(topic)),
            FilterMode::Whitelist(p) => p.iter().any(|re| re.is_match(topic)),
        }
    }

    /// One-line description for startup and live-update logs.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            FilterMode::Disabled => "no filter (all topics)".to_string(),
            FilterMode::Blacklist(p) => format!("blacklist [{}]", join_patterns(p)),
            FilterMode::Whitelist(p) => format!("whitelist [{}]", join_patterns(p)),
        }
    }

    /// The subset of `components` this filter admits, by index.
    #[must_use]
    pub fn admits(&self, components: &[SimComponent]) -> Vec<bool> {
        components.iter().map(|c| self.allows(&c.name)).collect()
    }
}

fn join_patterns(patterns: &[Regex]) -> String {
    patterns
        .iter()
        .map(Regex::as_str)
        .collect::<Vec<_>>()
        .join(", ")
}

fn compile_patterns(patterns: &[String]) -> Result<Vec<Regex>, String> {
    patterns
        .iter()
        .map(|p| Regex::new(p).map_err(|e| format!("Invalid regex '{p}': {e}")))
        .collect()
}
