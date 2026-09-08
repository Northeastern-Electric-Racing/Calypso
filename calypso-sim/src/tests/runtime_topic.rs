//! Building a `SimComponent` from caller-supplied parameters. These guard the
//! two ways a bad spec hurts: a panic in the sampler, and a component that
//! looks fine but publishes nonsense forever.

use crate::runtime_topic::{TopicSpec, ValueSpec, build};
use crate::simulatable_message::SimValue;

fn range(min: f32, max: f32, inc_min: f32, inc_max: f32) -> ValueSpec {
    ValueSpec::Range {
        min,
        max,
        inc_min,
        inc_max,
        round: false,
    }
}

fn spec(topic: &str, sim_freq: f32, sim: ValueSpec) -> TopicSpec {
    TopicSpec {
        topic: topic.to_string(),
        unit: "V".to_string(),
        sim_freq,
        sim,
    }
}

#[test]
fn rejects_names_it_cannot_publish() {
    assert!(build(spec("", 100.0, range(0.0, 1.0, 0.0, 1.0))).is_err());
    assert!(build(spec("   ", 100.0, range(0.0, 1.0, 0.0, 1.0))).is_err());
    // `{}` needs a parallel points_intopic vector a runtime topic cannot supply;
    // left unset the topic publishes with a literal `{}` and no warning.
    assert!(build(spec("A/{}/B", 100.0, range(0.0, 1.0, 0.0, 1.0))).is_err());
}

#[test]
fn rejects_frequencies_that_are_not_a_rate() {
    // 0 is the dangerous one: `should_update` is `elapsed > sim_freq`, so it
    // means "due on every 5ms tick", not "off".
    for bad in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(
            build(spec("A/B", bad, range(0.0, 1.0, 0.0, 1.0))).is_err(),
            "sim_freq {bad} should be rejected"
        );
    }
    assert!(build(spec("A/B", 1.0, range(0.0, 1.0, 0.0, 1.0))).is_ok());
}

#[test]
fn rejects_range_bounds_that_would_panic_the_sampler() {
    // The sampler only tests `max - min > eps`, so a non-finite span slips
    // through to `random_range`, which panics on non-finite bounds. Building
    // these must fail rather than arm that panic for the first publish tick.
    assert!(build(spec("A/B", 100.0, range(f32::NAN, 1.0, 0.0, 1.0))).is_err());
    assert!(build(spec("A/B", 100.0, range(0.0, f32::INFINITY, 0.0, 1.0))).is_err());
    assert!(build(spec("A/B", 100.0, range(f32::NEG_INFINITY, 1.0, 0.0, 1.0))).is_err());
    assert!(build(spec("A/B", 100.0, range(0.0, 1.0, f32::NAN, 1.0))).is_err());
    assert!(build(spec("A/B", 100.0, range(0.0, 1.0, 0.0, f32::INFINITY))).is_err());
    // Both bounds are finite, but the span overflows to infinity.
    assert!(build(spec("A/B", 100.0, range(f32::MIN, f32::MAX, 0.0, 1.0))).is_err());
}

#[test]
fn rejects_inverted_range_and_increments() {
    assert!(build(spec("A/B", 100.0, range(10.0, 0.0, 0.0, 1.0))).is_err());
    assert!(build(spec("A/B", 100.0, range(0.0, 10.0, -1.0, 1.0))).is_err());
    assert!(build(spec("A/B", 100.0, range(0.0, 10.0, 5.0, 1.0))).is_err());
    // min == max is degenerate but handled downstream, so it is allowed.
    assert!(build(spec("A/B", 100.0, range(5.0, 5.0, 0.0, 1.0))).is_ok());
}

#[test]
fn range_starts_inside_its_bounds() {
    // `build` must call initialize(): without it `current` stays 0.0, which for
    // a range above zero is outside the bounds — and `update` can never recover,
    // because every candidate lands out of range and it gives up after 10 tries.
    let component = build(spec("A/B", 100.0, range(100.0, 200.0, 1.0, 5.0))).unwrap();
    let SimValue::Range { current, .. } = component.points[0].value else {
        panic!("expected a Range");
    };
    assert!(
        (100.0..=200.0).contains(&current),
        "current {current} is outside 100..200"
    );
}

#[test]
fn discrete_weights_become_running_ceilings() {
    // The stored second element is a cumulative ceiling, not the weight the
    // caller wrote. `update` picks the first ceiling above a 0..1 sample.
    let component = build(spec(
        "A/B",
        100.0,
        ValueSpec::Discrete {
            options: vec![(0.0, 0.3), (1.0, 0.3), (2.0, 0.4)],
        },
    ))
    .unwrap();
    let SimValue::Discrete { ref options, .. } = component.points[0].value else {
        panic!("expected a Discrete");
    };
    let ceilings: Vec<f32> = options.iter().map(|(_, c)| *c).collect();
    assert_eq!(
        options.iter().map(|(v, _)| *v).collect::<Vec<_>>(),
        [0.0, 1.0, 2.0]
    );
    assert!((ceilings[0] - 0.3).abs() < 1e-6, "{ceilings:?}");
    assert!((ceilings[1] - 0.6).abs() < 1e-6, "{ceilings:?}");
    // The last must be exactly 1.0: any shortfall is the probability with which
    // `update` finds no match and publishes -1.0.
    assert_eq!(ceilings[2], 1.0);
}

#[test]
fn discrete_normalises_weights_that_do_not_sum_to_one() {
    let component = build(spec(
        "A/B",
        100.0,
        ValueSpec::Discrete {
            options: vec![(7.0, 3.0), (8.0, 1.0)],
        },
    ))
    .unwrap();
    let SimValue::Discrete { ref options, .. } = component.points[0].value else {
        panic!("expected a Discrete");
    };
    assert!((options[0].1 - 0.75).abs() < 1e-6, "{options:?}");
    assert_eq!(options[1].1, 1.0);
}

#[test]
fn rejects_discrete_option_sets_that_cannot_be_sampled() {
    let empty = ValueSpec::Discrete { options: vec![] };
    assert!(build(spec("A/B", 100.0, empty)).is_err());

    let zero_sum = ValueSpec::Discrete {
        options: vec![(0.0, 0.0), (1.0, 0.0)],
    };
    assert!(build(spec("A/B", 100.0, zero_sum)).is_err());

    let bad_weight = ValueSpec::Discrete {
        options: vec![(0.0, f32::NAN)],
    };
    assert!(build(spec("A/B", 100.0, bad_weight)).is_err());

    let bad_value = ValueSpec::Discrete {
        options: vec![(f32::INFINITY, 1.0)],
    };
    assert!(build(spec("A/B", 100.0, bad_value)).is_err());
}
