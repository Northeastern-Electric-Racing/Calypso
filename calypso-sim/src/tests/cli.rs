//! CLI mode arbitration. `run_mock` decides when the background
//! heartbeat runs; the regression this guards is adding a new foreground mode
//! and forgetting to suppress the heartbeat under it.

use crate::cli::Cli;
use clap::Parser;

fn cli(args: &[&str]) -> Cli {
    Cli::try_parse_from(args).expect("valid args")
}

#[test]
fn run_mock_arbitrates_the_heartbeat_against_other_modes() {
    // On by default when nothing else is selected.
    assert!(cli(&["calypso-sim"]).run_mock());
    // A foreground mode or --list-topics turns the heartbeat off...
    assert!(!cli(&["calypso-sim", "--stream"]).run_mock());
    assert!(!cli(&["calypso-sim", "--key-map", "keys.json"]).run_mock());
    assert!(!cli(&["calypso-sim", "--list-topics"]).run_mock());
    // ...unless --mock forces it back on alongside that mode.
    assert!(cli(&["calypso-sim", "--mock"]).run_mock());
    assert!(cli(&["calypso-sim", "--stream", "--mock"]).run_mock());
}
