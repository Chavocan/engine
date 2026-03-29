//! HTTP control plane — Axum (Phase 1d).
//!
//! **AI_DRIVER_HOOK** path: `crates/aetherforge_control/src/lib.rs` → `ai_driver_enqueue_intent`.

mod server;

pub mod play_log;
#[cfg(feature = "rate-limit")]
pub mod rate_limit;

pub use server::{app_router, app_router_with_config, AppState, ControlConfig};

use aetherforge_sim::{Intent, Simulation};

// AI_DRIVER_HOOK: external agents enqueue Intent here (Axum handlers or CLI harness call this).
pub fn ai_driver_enqueue_intent(sim: &mut Simulation, intent: Intent) {
    sim.apply_intent(intent);
}
