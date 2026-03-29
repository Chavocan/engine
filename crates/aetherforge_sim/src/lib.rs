//! Authoritative simulation kernel — Phase 2b: `world` snapshot + canonical `wire` JSON.

mod wire;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub use wire::{observation_to_string, observation_to_vec};

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

/// Versioned world slice (may start empty). **Bump policy:** patch = additive fields only; minor = new optional sections; major = breaking entity shape — see `docs/phase2-plan.md`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldSnapshot {
    pub world_version: String,
    #[serde(default)]
    pub entities: Vec<serde_json::Value>,
}

impl Default for WorldSnapshot {
    fn default() -> Self {
        Self {
            world_version: "1.0.0".to_string(),
            entities: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Observation {
    /// Observation envelope version (includes `world` since 1.1.0).
    pub schema_version: String,
    pub tick: u64,
    pub run_id: String,
    pub message: String,
    /// Determinism probe: `rng.next_u32()` after each `step`; same seed + intents ⇒ same sequence (**FP/thread caveats still UNTESTED**).
    pub rng_draw: u32,
    #[serde(default)]
    pub world: WorldSnapshot,
    /// Present only when built with **`farm-stub`** and serialized if `Some` (default builds omit key).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub farm: Option<aetherforge_farm::FarmSnapshot>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub kind: String,
}

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub run_id: String,
    pub seed: u64,
}

impl SimulationConfig {
    pub fn new(run_id: impl Into<String>, seed: u64) -> Self {
        Self {
            run_id: run_id.into(),
            seed,
        }
    }
}

pub struct Simulation {
    tick: u64,
    run_id: String,
    last_intent_kind: String,
    rng: StdRng,
    rng_draw: u32,
    #[cfg(feature = "farm-stub")]
    farm_world: aetherforge_farm::FarmWorld,
}

impl Simulation {
    /// Deterministic default seed derived from `run_id` (stable hash — not cryptographic).
    pub fn new(run_id: impl Into<String>) -> Self {
        let run_id = run_id.into();
        let seed = stable_seed_from_run_id(&run_id);
        Self::with_config(SimulationConfig { run_id, seed })
    }

    pub fn with_config(cfg: SimulationConfig) -> Self {
        let mut rng = StdRng::seed_from_u64(cfg.seed);
        let rng_draw = rng.next_u32();
        Self {
            tick: 0,
            run_id: cfg.run_id,
            last_intent_kind: String::new(),
            rng,
            rng_draw,
            #[cfg(feature = "farm-stub")]
            farm_world: aetherforge_farm::FarmWorld::default(),
        }
    }

    pub fn apply_intent(&mut self, intent: Intent) {
        #[cfg(feature = "farm-stub")]
        {
            use aetherforge_farm::{CropId, FarmPlot, TileCoord, MAX_GROWTH_STAGE};
            if intent.kind == "farm_plant" {
                let n = self.farm_world.plots.len() as i32;
                self.farm_world.plots.push(FarmPlot {
                    coord: TileCoord { x: n, y: 0 },
                    crop: CropId::new("stub_crop"),
                    growth_stage: 0,
                });
            } else if intent.kind == "farm_advance_day" {
                self.farm_world.day = self.farm_world.day.saturating_add(1);
                for p in &mut self.farm_world.plots {
                    if p.growth_stage < MAX_GROWTH_STAGE {
                        p.growth_stage += 1;
                    }
                }
            } else if intent.kind == "farm_harvest" {
                let plots = std::mem::take(&mut self.farm_world.plots);
                let mut kept = Vec::new();
                for p in plots {
                    if p.growth_stage >= MAX_GROWTH_STAGE {
                        let key = format!("harvested_{}", p.crop.0);
                        *self.farm_world.inventory.items.entry(key).or_insert(0) += 1;
                    } else {
                        kept.push(p);
                    }
                }
                self.farm_world.plots = kept;
            }
        }
        self.last_intent_kind = intent.kind;
    }

    pub fn step(&mut self) {
        self.tick = self.tick.saturating_add(1);
        self.rng_draw = self.rng.next_u32();
        tracing::event!(
            target: "aetherforge.sim.tick",
            tracing::Level::INFO,
            tick = self.tick,
            run_id = %self.run_id,
            rng_draw = self.rng_draw,
            "tick"
        );
    }

    pub fn snapshot_observation(&self) -> Observation {
        #[cfg(feature = "farm-stub")]
        let farm = Some(aetherforge_farm::FarmSnapshot::from_world(
            &self.farm_world,
            self.tick,
        ));
        #[cfg(not(feature = "farm-stub"))]
        let farm = None;
        Observation {
            schema_version: "1.1.0".to_string(),
            tick: self.tick,
            run_id: self.run_id.clone(),
            message: if self.last_intent_kind.is_empty() {
                "no_intent_applied".to_string()
            } else {
                format!("last_intent={}", self.last_intent_kind)
            },
            rng_draw: self.rng_draw,
            world: WorldSnapshot::default(),
            farm,
        }
    }
}

fn stable_seed_from_run_id(run_id: &str) -> u64 {
    let mut h = DefaultHasher::new();
    run_id.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_advances_tick() {
        let mut s = Simulation::with_config(SimulationConfig::new("test-run", 1));
        assert_eq!(s.snapshot_observation().tick, 0);
        s.step();
        assert_eq!(s.snapshot_observation().tick, 1);
    }

    #[test]
    fn reproducibility_same_seed_same_observation_json() {
        let cfg = SimulationConfig::new("run-a", 42);
        let intents = ["till", "plant", "water", "wait"];
        let mut a = Simulation::with_config(cfg.clone());
        let mut b = Simulation::with_config(cfg);
        for k in intents {
            a.apply_intent(Intent { kind: k.to_string() });
            b.apply_intent(Intent { kind: k.to_string() });
            a.step();
            b.step();
            assert_eq!(
                observation_to_string(&a.snapshot_observation()).unwrap(),
                observation_to_string(&b.snapshot_observation()).unwrap()
            );
        }
    }
}

#[cfg(all(test, feature = "farm-stub"))]
mod farm_stub_tests {
    use super::*;
    use aetherforge_farm::MAX_GROWTH_STAGE;

    #[test]
    fn farm_plant_adds_plot_to_observation() {
        let mut s = Simulation::with_config(SimulationConfig::new("farm-test", 7));
        let obs0 = s.snapshot_observation();
        assert_eq!(obs0.farm.as_ref().unwrap().plots.len(), 0);
        s.apply_intent(Intent {
            kind: "farm_plant".to_string(),
        });
        s.step();
        let obs1 = s.snapshot_observation();
        assert_eq!(obs1.farm.as_ref().unwrap().plots.len(), 1);
        assert_eq!(obs1.farm.as_ref().unwrap().plots[0].crop.0, "stub_crop");
    }

    #[test]
    fn farm_plant_then_advance_increases_stage() {
        let mut s = Simulation::with_config(SimulationConfig::new("farm-growth", 3));
        s.apply_intent(Intent {
            kind: "farm_plant".to_string(),
        });
        s.step();
        assert_eq!(s.snapshot_observation().farm.as_ref().unwrap().plots[0].growth_stage, 0);
        s.apply_intent(Intent {
            kind: "farm_advance_day".to_string(),
        });
        s.step();
        let obs = s.snapshot_observation();
        let farm = obs.farm.as_ref().unwrap();
        assert_eq!(farm.plots[0].growth_stage, 1);
        assert_eq!(farm.day, 2);
    }

    #[test]
    fn farm_demo_loop_plant_grow_harvest() {
        let mut s = Simulation::with_config(SimulationConfig::new("farm-harvest-loop", 11));
        s.apply_intent(Intent {
            kind: "farm_plant".to_string(),
        });
        s.step();
        let obs0 = s.snapshot_observation();
        let farm0 = obs0.farm.as_ref().unwrap();
        assert_eq!(farm0.plots.len(), 1);
        assert_eq!(farm0.plots[0].growth_stage, 0);

        for _ in 0..3 {
            s.apply_intent(Intent {
                kind: "farm_advance_day".to_string(),
            });
            s.step();
        }
        let obs = s.snapshot_observation();
        let farm = obs.farm.as_ref().unwrap();
        assert_eq!(farm.plots.len(), 1);
        assert_eq!(farm.plots[0].growth_stage, MAX_GROWTH_STAGE);
        assert_eq!(farm.day, 4);

        s.apply_intent(Intent {
            kind: "farm_harvest".to_string(),
        });
        s.step();
        let obs2 = s.snapshot_observation();
        let farm2 = obs2.farm.as_ref().unwrap();
        assert!(farm2.plots.is_empty());
        assert_eq!(
            farm2.inventory.items.get("harvested_stub_crop").copied(),
            Some(1)
        );
    }
}
