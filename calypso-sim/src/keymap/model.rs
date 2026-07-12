//! The scenario data model — plain deserialized types, no logic. The functions
//! that load, validate, and run these live in the parent `keymap` module.

use std::collections::HashMap;

use serde::Deserialize;

/// A scenario file: a set of named [`Action`]s keyed by name.
///
/// A scenario drives the interactive (`--key-map`) and replay (`--play`) modes.
/// There is no separate "script" concept: a replay program is itself just an
/// [`Action`] whose steps run in order.
pub type Scenario = HashMap<String, Action>;

/// One named command: an ordered list of [`Step`]s, optionally bound to a
/// single keyboard `key` for interactive mode.
#[derive(Debug, Clone, Deserialize)]
pub struct Action {
    /// Keyboard key that fires this action in interactive mode. Actions with
    /// no `key` are still reachable via `--play` or another action's invoke
    /// step, so pure replay programs need no key.
    #[serde(default)]
    pub key: Option<char>,

    /// Human-readable label shown in the interactive listing and in the log
    /// line printed when the action fires.
    #[serde(default)]
    pub desc: Option<String>,

    /// The steps run, in order, each time the action fires.
    pub steps: Vec<Step>,
}

/// One step of an [`Action`]. The three forms are disambiguated purely by
/// shape — a bare string, an object with `topic`, or an object with
/// `sleep_ms` — so there is no order-dependent matching to get wrong.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Step {
    /// Run another action by name — the composition/reuse primitive.
    Invoke(String),

    /// Publish to `topic`. Exactly one of `value` (scalar) or `values` (array)
    /// must be set; `unit` defaults to empty. This is validated at load.
    Publish {
        topic: String,
        #[serde(default)]
        value: Option<f32>,
        #[serde(default)]
        values: Option<Vec<f32>>,
        #[serde(default)]
        unit: Option<String>,
    },

    /// Wait `sleep_ms` milliseconds before the next step.
    Sleep { sleep_ms: u64 },
}
