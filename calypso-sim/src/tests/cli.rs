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
    assert!(!cli(&["calypso-sim", "--key-map", "keys.json", "--play", "demo"]).run_mock());
    assert!(!cli(&["calypso-sim", "--list-topics"]).run_mock());
    // ...unless --mock forces it back on alongside that mode.
    assert!(cli(&["calypso-sim", "--mock"]).run_mock());
    assert!(cli(&["calypso-sim", "--stream", "--mock"]).run_mock());
}

#[test]
fn zenoh_conf_requires_the_zenoh_flag() {
    // Default transport is MQTT; --zenoh switches it.
    assert!(!cli(&["calypso-sim"]).zenoh);
    assert!(cli(&["calypso-sim", "--zenoh"]).zenoh);
    assert!(cli(&["calypso-sim", "-z"]).zenoh);

    // A conf path is optional (Zenoh defaults are used without it)...
    assert!(cli(&["calypso-sim", "--zenoh"]).zenoh_conf.is_none());
    assert!(
        cli(&["calypso-sim", "--zenoh", "--zenoh-conf", "z.json5"])
            .zenoh_conf
            .is_some()
    );

    // ...but it is meaningless without --zenoh, so clap must reject it rather
    // than silently ignoring the file the user pointed at.
    assert!(Cli::try_parse_from(["calypso-sim", "--zenoh-conf", "z.json5"]).is_err());
}
