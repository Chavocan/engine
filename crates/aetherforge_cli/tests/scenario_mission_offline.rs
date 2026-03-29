//! R0: scenario `expect_mission_outcome` + farm-stub harvest → **`won`**.

use aetherforge_cli::scenario::{run_offline, ScenarioFile};

#[test]
fn farm_demo_loop_json_mission_won_offline() {
    let raw = include_str!("../../../examples/farm_demo_loop.json");
    let scenario: ScenarioFile = serde_json::from_str(raw).expect("parse examples/farm_demo_loop.json");
    assert_eq!(
        scenario.expect_mission_outcome.as_deref(),
        Some("won")
    );
    run_offline(&scenario).expect("offline scenario must reach mission won");
}
