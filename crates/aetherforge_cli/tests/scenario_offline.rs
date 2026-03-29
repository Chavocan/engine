use aetherforge_cli::scenario::{run_offline, ScenarioFile};

#[test]
fn offline_scenario_from_json_happy_path() {
    let scenario: ScenarioFile = serde_json::from_str(
        r#"{
        "seed": 42,
        "steps": [
            { "use": "single", "kind": "first", "expect_tick": 1 },
            {
                "use": "batch",
                "actions": [
                    { "schema_version": "1.0.0", "kind": "b", "payload": {} },
                    { "schema_version": "1.0.0", "kind": "c", "payload": {} }
                ],
                "expect_tick": 3
            }
        ]
    }"#,
    )
    .expect("parse");
    run_offline(&scenario).expect("scenario should pass");
}

#[test]
fn offline_scenario_wrong_expect_tick_fails() {
    let scenario: ScenarioFile = serde_json::from_str(
        r#"{
        "seed": 1,
        "steps": [
            { "use": "single", "kind": "x", "expect_tick": 99 }
        ]
    }"#,
    )
    .expect("parse");

    let err = run_offline(&scenario).expect_err("should fail tick check");
    assert_eq!(err.step_index, 0);
    assert!(err.reason.contains("tick mismatch"));
    assert_eq!(err.expected_tick, Some(99));
    assert_eq!(err.actual_tick, Some(1));
}
