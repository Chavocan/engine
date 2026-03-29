//! Headless entry: ≥1 fixed tick, one JSON observation line on stdout (via `wire` — same as HTTP).
use aetherforge_control::{ai_driver_enqueue_intent, play_log};
use aetherforge_sim::{observation_to_string, Intent, Simulation, SimulationConfig};

fn main() {
    play_log::try_init_from_env();
    let mut sim = Simulation::with_config(SimulationConfig::new("headless-cli", 42));
    ai_driver_enqueue_intent(
        &mut sim,
        Intent {
            kind: "noop".to_string(),
        },
    );
    sim.step();
    let obs = sim.snapshot_observation();
    println!(
        "{}",
        observation_to_string(&obs).expect("observation json (wire)")
    );
}
