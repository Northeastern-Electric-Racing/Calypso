//! Keymap logic that is fragile under refactoring: the increment
//! emit-then-advance arithmetic, and the serde `untagged` shape disambiguation
//! (whose behavior depends on variant *order*, which the type alone doesn't
//! make obvious).

use crate::keymap::{KeyEntry, advance_increment, parse_key_map};

/// Press an increment state `presses` times, collecting the value emitted on
/// each press. Increment emits the current value *before* advancing.
fn press_sequence(
    start: f32,
    step: f32,
    min: Option<f32>,
    max: Option<f32>,
    presses: usize,
) -> Vec<f32> {
    let mut current = start;
    (0..presses)
        .map(|_| advance_increment(&mut current, step, min, max))
        .collect()
}

#[test]
fn advance_increment_emits_then_wraps_or_saturates() {
    // Exact, deterministic values; compared through `Vec<f32>` (element-wise
    // `PartialEq`) to stay clear of clippy's pedantic `float_cmp`.
    assert_eq!(
        press_sequence(0.0, 1.0, Some(0.0), Some(2.0), 5),
        vec![0.0, 1.0, 2.0, 0.0, 1.0],
        "with a min, climbing past max wraps back to min"
    );
    assert_eq!(
        press_sequence(0.0, 1.0, None, Some(2.0), 5),
        vec![0.0, 1.0, 2.0, 2.0, 2.0],
        "with no min, the value saturates at max"
    );
    assert_eq!(
        press_sequence(2.0, -1.0, Some(0.0), Some(2.0), 5),
        vec![2.0, 1.0, 0.0, 2.0, 1.0],
        "a negative step wraps min back up to max"
    );
}

#[test]
fn parse_disambiguates_the_four_entry_shapes() {
    let map = parse_key_map(
        r#"{
            "r": "Some/Topic",
            "p": {"topic": "T", "value": 1.0},
            "i": {"topic": "T", "value": 0.0, "step": 1.0},
            "s": {"sequence": [{"topic": "T", "value": 1.0}]}
        }"#,
    )
    .expect("valid keymap");

    // Order matters in the `untagged` enum: `step` must win over `value`
    // (Increment before Pinned), and `sequence` must be recognized ahead of the
    // other object forms. Reordering the variants would silently break this.
    assert!(
        matches!(map.get(&'r'), Some(KeyEntry::TopicOnly(_))),
        "bare string should be random-mode"
    );
    assert!(
        matches!(map.get(&'p'), Some(KeyEntry::Pinned { .. })),
        "`value` without `step` should be pinned"
    );
    assert!(
        matches!(map.get(&'i'), Some(KeyEntry::Increment { .. })),
        "`step` should select increment even with `value` present"
    );
    assert!(
        matches!(map.get(&'s'), Some(KeyEntry::Sequence { .. })),
        "`sequence` should select sequence mode"
    );
}
