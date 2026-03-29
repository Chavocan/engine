# Platform — headed client roadmap (post–compile smoke)

**Current:** `aetherforge_platform` provides **`headed-smoke`** / **`aetherforge_wgpu_smoke`** — a headless `wgpu::Instance` compile/run check ([crates/aetherforge_platform](../crates/aetherforge_platform)). `platform_placeholder()` remains the public stub for a full loop.

**R2 (Director program):** v1 **client surface** for observation is the Python terminal HUD — [`python/aetherforge_sdk/examples/observation_hud.py`](../python/aetherforge_sdk/examples/observation_hud.py) — not this crate. Phases below remain the **Rust headed** track.

## Planned phases (planning only)

| Phase | Goal | Acceptance sketch |
|-------|------|---------------------|
| P1 | **Window** | `winit` window behind `windowed` feature; event loop exits cleanly on close or timeout; CI skip with documented reason on headless runners (no GPU). |
| P2 | **Swapchain / clear** | First frame clear color; survives one frame without validation errors on dev machines. |
| P3 | **Sim hook** | Optional pull of `aetherforge_sim` observation snapshot into a debug HUD or log line (parity with headless tick). |
| P4 | **Input → intent** | Map window/input to the same `Intent` / HTTP path as the control plane (or in-process driver) per ADR. |

## Dependencies

- Keep **optional** heavy deps (`wgpu`, `winit`) behind features so default `cargo test` stays fast.
- Reuse **`docs/phase1d-verification.md`** when adding tests; retire “platform stub” rows as phases land.

## Related

- [crates/aetherforge_platform/src/lib.rs](../crates/aetherforge_platform/src/lib.rs)
- Director program vision: [aetherforge-lead-director.agent.md](../aetherforge-lead-director.agent.md) (Godot / Unity track may supersede or fork this crate).
