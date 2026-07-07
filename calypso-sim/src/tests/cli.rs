//! CLI mode arbitration. `run_autonomous` decides when the background
//! heartbeat runs; the regression this guards is adding a new foreground mode
//! and forgetting to suppress the heartbeat under it.

use crate::cli::Cli;
use clap::Parser;

fn cli(args: &[&str]) -> Cli {
    Cli::try_parse_from(args).expect("valid args")
}

#[test]
fn run_autonomous_arbitrates_the_heartbeat_against_other_modes() {
    // On by default when nothing else is selected.
    assert!(cli(&["calypso-sim"]).run_autonomous());
    // A foreground mode or --list-topics turns the heartbeat off...
    assert!(!cli(&["calypso-sim", "--stream"]).run_autonomous());
    assert!(!cli(&["calypso-sim", "--key-map", "keys.json"]).run_autonomous());
    assert!(!cli(&["calypso-sim", "--list-topics"]).run_autonomous());
    // ...unless --auto forces it back on alongside that mode.
    assert!(cli(&["calypso-sim", "--auto"]).run_autonomous());
    assert!(cli(&["calypso-sim", "--stream", "--auto"]).run_autonomous());
}
