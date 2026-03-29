//! Pure farming domain data (Phase 5a stub) — no rendering, no gameplay loop.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Simulation ticks in a day cycle for `time_minutes` (see [`FarmSnapshot::from_world`]).
pub const LOGICAL_TICKS_PER_DAY: u64 = 24;
/// In-game minutes advanced per simulation tick within one day (`24 * 60 / 24`).
pub const MINUTES_PER_SIM_TICK: u64 = 60;

/// `growth_stage` stops increasing at this value (stub cap).
pub const MAX_GROWTH_STAGE: u32 = 3;

fn default_farm_day() -> u64 {
    1
}

/// Opaque crop identifier (string id for v0 stub).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CropId(pub String);

impl CropId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Integer tile coordinates in an abstract farm grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

/// One planted tile (growth timer deferred to Phase 5b).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmPlot {
    pub coord: TileCoord,
    pub crop: CropId,
    pub growth_stage: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(default)]
    pub items: HashMap<String, u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmWorld {
    #[serde(default = "default_farm_day")]
    pub day: u64,
    #[serde(default)]
    pub plots: Vec<FarmPlot>,
    #[serde(default)]
    pub inventory: Inventory,
}

impl Default for FarmWorld {
    fn default() -> Self {
        Self {
            day: 1,
            plots: Vec::new(),
            inventory: Inventory::default(),
        }
    }
}

/// Sub-object embedded in `Observation` when `farm-stub` is enabled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmSnapshot {
    /// Calendar day counter; incremented by intent **`farm_advance_day`**.
    #[serde(default = "default_farm_day")]
    pub day: u64,
    /// `(sim_tick % LOGICAL_TICKS_PER_DAY) * MINUTES_PER_SIM_TICK` (see constants above).
    #[serde(default)]
    pub time_minutes: u32,
    #[serde(default)]
    pub plots: Vec<FarmPlot>,
    #[serde(default)]
    pub inventory: Inventory,
}

impl FarmSnapshot {
    pub fn from_world(world: &FarmWorld, sim_tick: u64) -> Self {
        let time_minutes = (sim_tick % LOGICAL_TICKS_PER_DAY) * MINUTES_PER_SIM_TICK;
        let time_minutes = u32::try_from(time_minutes).unwrap_or(u32::MAX);
        Self {
            day: world.day,
            time_minutes,
            plots: world.plots.clone(),
            inventory: world.inventory.clone(),
        }
    }
}
