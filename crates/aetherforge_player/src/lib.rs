//! HTTP-only autonomous player (Phase 6b / ADR 0002). **No `aetherforge_sim` dependency** — remote-AI parity.

pub mod player;

pub use player::{run_player, PlayerConfig, PlayerError, PlayerPolicy};
