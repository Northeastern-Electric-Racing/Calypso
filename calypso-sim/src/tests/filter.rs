//! Topic filtering. The regression these guard is the filter silently letting
//! everything through (or nothing) after a build or a live update.

use crate::filter::FilterMode;

fn allows(filter: &FilterMode, topics: &[&str]) -> Vec<bool> {
    topics.iter().map(|t| filter.allows(t)).collect()
}

#[test]
fn build_picks_the_mode_and_compiles_patterns() {
    let none = FilterMode::build(&[], &[]).unwrap();
    assert_eq!(allows(&none, &["BMS/A", "VCU/B"]), [true, true]);

    let black = FilterMode::build(&[], &["^BMS/".into()]).unwrap();
    assert_eq!(allows(&black, &["BMS/A", "VCU/B"]), [false, true]);

    let white = FilterMode::build(&["^BMS/".into()], &[]).unwrap();
    assert_eq!(allows(&white, &["BMS/A", "VCU/B"]), [true, false]);

    // disable wins when both are given; the CLI marks them mutually exclusive,
    // but the live commands route through the same constructor.
    let both = FilterMode::build(&["^VCU/".into()], &["^BMS/".into()]).unwrap();
    assert_eq!(allows(&both, &["BMS/A", "VCU/B"]), [false, true]);

    // A bad regex is rejected rather than quietly matching nothing.
    assert!(FilterMode::build(&[], &["[".into()]).is_err());
}

#[test]
fn a_whitelist_with_no_match_admits_nothing() {
    // The dangerous direction: an over-narrow whitelist must silence the
    // heartbeat outright, not fall back to allowing everything.
    let white = FilterMode::build(&["^Nope/".into()], &[]).unwrap();
    assert_eq!(allows(&white, &["BMS/A", "VCU/B"]), [false, false]);
}
