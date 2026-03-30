# Platform — headed client roadmap (post–compile smoke)

**Current:** `aetherforge_platform` provides:

- **`headed-smoke`** / **`aetherforge_wgpu_smoke`** — headless `wgpu::Instance` compile/run check.
- **`windowed`** / **`aetherforge_window`** — **`winit`** + **`wgpu`** clear, **in-process** [`aetherforge_sim`](../../crates/aetherforge_sim) with **`farm-stub`**. **Keyboard → intent:** **P**/1 plant, **D**/2 day, **H**/3 harvest, **Space** noop (same `kind` strings as HTTP). Optional **`AETHERFORGE_WINDOW_AUTO_DEMO=1`** runs the scripted 5-step loop every frame. **Title bar** = HUD. Env: `AETHERFORGE_WINDOW_SEED`, `AETHERFORGE_WINDOW_MAX_SEC`. CI **compiles** only (`cargo build --features "headed-smoke windowed"`); it does **not** run the binary on headless runners.

**R2 (Director program):** v1 **client surface** also includes the Python terminal HUD — [`python/aetherforge_sdk/examples/observation_hud.py`](../python/aetherforge_sdk/examples/observation_hud.py). The Rust window is the **in-repo** graphical viewport; Godot/web remain valid per [**ADR 0004**](adr/0004-runtime-embedding.md).

## Phases

| Phase | Goal | Status |
|-------|------|--------|
| P1 | **Window** | **Done:** feature **`windowed`**, **`aetherforge_window`**, close or **`AETHERFORGE_WINDOW_MAX_SEC`** timeout. |
| P2 | **Swapchain / clear** | **Done:** single-frame dark-blue clear via wgpu render pass. |
| P3 | **Sim hook** | **Done:** `aetherforge_window` steps **`Simulation`** each frame; title bar = observation HUD; clear color keyed off tick. |
| P4 | **Input → intent** | **Done:** physical keys → `farm_*` / `noop` kinds; auto-demo env preserves old loop. |

## Dependencies

- Keep **optional** heavy deps (`wgpu`, `winit`) behind features so default `cargo test` stays fast.
- Reuse **`docs/phase1d-verification.md`** when adding tests; retire “platform stub” rows as phases land.

## Related

- [crates/aetherforge_platform/src/lib.rs](../crates/aetherforge_platform/src/lib.rs)
- Director program vision: [aetherforge-lead-director.agent.md](../aetherforge-lead-director.agent.md) (Godot / Unity track may supersede or fork this crate).
