//! Scenario logic that a refactor could silently break: the `untagged` step
//! shape disambiguation, and the load-time validation (unknown/looping invokes,
//! publish value-arity) that keeps replays terminating and well-formed.

use crate::keymap::{Step, collect_topics, parse_scenario, validate};

#[test]
fn step_shapes_are_unambiguous() {
    let scenario = parse_scenario(
        r#"{
            "a": { "steps": ["other", {"topic": "T", "value": 1.0}, {"sleep_ms": 50}] },
            "other": { "steps": [{"topic": "U", "value": 0.0}] }
        }"#,
    )
    .expect("valid scenario");

    // A bare string is an invoke, an object with `topic` is a publish, and an
    // object with `sleep_ms` is a sleep — distinguished by shape, so unlike the
    // old keymap enum this does not depend on variant order.
    let steps = &scenario["a"].steps;
    assert!(matches!(steps[0], Step::Invoke(_)), "bare string -> invoke");
    assert!(
        matches!(steps[1], Step::Publish { .. }),
        "object with topic -> publish"
    );
    assert!(
        matches!(steps[2], Step::Sleep { .. }),
        "object with sleep_ms -> sleep"
    );
}

#[test]
fn validate_rejects_unknown_invoke_and_cycles() {
    let unknown = parse_scenario(r#"{ "a": { "steps": ["ghost"] } }"#).unwrap();
    assert!(
        validate(&unknown).is_err(),
        "invoking a missing action must fail"
    );

    let self_cycle = parse_scenario(r#"{ "a": { "steps": ["a"] } }"#).unwrap();
    assert!(
        validate(&self_cycle).is_err(),
        "a self-invoking action is a cycle"
    );

    let indirect = parse_scenario(r#"{ "a": {"steps":["b"]}, "b": {"steps":["a"]} }"#).unwrap();
    assert!(validate(&indirect).is_err(), "a -> b -> a is a cycle");
}

#[test]
fn validate_requires_exactly_one_of_value_or_values() {
    let both =
        parse_scenario(r#"{ "a": {"steps":[{"topic":"T","value":1.0,"values":[1.0]}]} }"#).unwrap();
    assert!(
        validate(&both).is_err(),
        "value + values together must fail"
    );

    let neither = parse_scenario(r#"{ "a": {"steps":[{"topic":"T"}]} }"#).unwrap();
    assert!(
        validate(&neither).is_err(),
        "neither value nor values must fail"
    );

    let ok = parse_scenario(r#"{ "a": {"steps":[{"topic":"T","values":[1.0,2.0]}]} }"#).unwrap();
    assert!(validate(&ok).is_ok(), "values alone is valid");
}

#[test]
fn collect_topics_follows_invokes() {
    let scenario = parse_scenario(
        r#"{
            "big": { "steps": ["small", {"topic": "A", "value": 1.0}] },
            "small": { "steps": [{"topic": "B", "value": 1.0}, {"sleep_ms": 5}] }
        }"#,
    )
    .unwrap();
    validate(&scenario).unwrap();

    let topics = collect_topics(&scenario, "big");
    assert!(
        topics.contains("A") && topics.contains("B"),
        "topics must be gathered through invoked actions, got {topics:?}"
    );
}
