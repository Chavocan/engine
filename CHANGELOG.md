# Changelog

## Unreleased

- Docs: **`SECURITY.md`**, **`docs/nl-agentic-hooks.md`** (NL/agentic design stub); dual **`LICENSE-MIT`** / **`LICENSE-APACHE`**; README **License** + **Security** sections.
- Control plane: batch actions (`POST .../actions`), session action quota (`429` / `SESSION_ACTION_QUOTA`), optional `farm-stub` farming slice with day/time, growth, and **`farm_harvest`** (ripe plots → `inventory`), JSON scenario runner (`aetherforge_scenario`), example `examples/farm_demo_loop.json`, HTTP-only player (`aetherforge_player`), Phase 6 design doc, server hardening and player-boundary CI checks.
- CI: **`rust` job** runs offline **`examples/farm_demo_loop.json`** via `aetherforge_scenario` with **`farm-stub`** (see `CONTRIBUTING.md`).
- Play log: **`farm_harvested`** event when **`AETHERFORGE_PLAY_LOG=1`** and action kind is **`farm_harvest`** (`farm-stub` builds only); payload includes `plots_remaining` and `harvested_item_total`.
