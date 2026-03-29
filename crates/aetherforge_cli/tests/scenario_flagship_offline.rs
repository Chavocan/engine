//! R1: flagship two-cycle farm scenario (offline, farm-stub).

use aetherforge_cli::scenario::{run_offline, ScenarioFile};

#[test]
fn flagship_two_cycles_offline_mission_won() {
    let raw = include_str!("../../../examples/flagship_farm_two_cycles.json");
    let scenario: ScenarioFile = serde_json::from_str(raw).expect("parse flagship_farm_two_cycles.json");
    assert_eq!(scenario.steps.len(), 10);
    run_offline(&scenario).expect("flagship offline scenario");
}
