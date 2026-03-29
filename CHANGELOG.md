# Changelog

## Unreleased

- **`aetherforge_scenario`:** `--emit-tick-json` (stderr `{"event":"tick","tick":…}` per step) and `--quiet` (suppress tick lines); integration tests `scenario_tick_json`.
- **Breaking (`aetherforge_cli` library):** `run_http` now takes `on_tick: &mut F`; `run_offline_with_ticks` added; `run_offline` unchanged.
- **Breaking (CLI UX):** Autonomous player binary moved to workspace crate **`aetherforge_player`**. Use **`cargo run -p aetherforge_player -- ...`** instead of **`cargo run -p aetherforge_cli --bin aetherforge_player`** (ADR **`docs/adr/0002-player-crate-split.md`**). The player package has **no** `aetherforge_sim` dependency.
- Docs: **`SECURITY.md`**, **`docs/nl-agentic-hooks.md`** (NL/agentic design stub); dual **`LICENSE-MIT`** / **`LICENSE-APACHE`**; README **License** + **Security** sections.
- Control plane: batch actions (`POST .../actions`), session action quota (`429` / `SESSION_ACTION_QUOTA`), optional `farm-stub` farming slice with day/time, growth, and **`farm_harvest`** (ripe plots → `inventory`), JSON scenario runner (`aetherforge_scenario`), example `examples/farm_demo_loop.json`, HTTP-only player (`aetherforge_player`), Phase 6 design doc, server hardening and player-boundary CI checks.
- CI: **`rust` job** runs offline **`examples/farm_demo_loop.json`** via `aetherforge_scenario` with **`farm-stub`** (see `CONTRIBUTING.md`); asserts **`aetherforge_player`** has **no `aetherforge_sim`** on **`cargo tree -e normal`** runtime edges.
- Python SDK: fix **`pyproject.toml`** — `classifiers` and `dependencies` belong under **`[project]`**, not **`[project.urls]`**, restoring valid **`pip install -e`** / setuptools validation.
- Docs: **`docs/adr/0002-player-crate-split.md`** — **Accepted**; player is now workspace crate **`aetherforge_player`** (no `aetherforge_sim` dep).
- Play log: **`farm_harvested`** event when **`AETHERFORGE_PLAY_LOG=1`** and action kind is **`farm_harvest`** (`farm-stub` builds only); payload includes `plots_remaining` and `harvested_item_total`.
