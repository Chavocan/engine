# Phase 5a — Farming domain stub (`farm-stub`)

## Crate

- **`crates/aetherforge_farm`** — pure data: `CropId`, `TileCoord`, `FarmPlot`, `Inventory`, `FarmWorld`, `FarmSnapshot` (JSON-shaped slice for observations).

## Simulation hook

- Rust feature **`farm-stub`** on **`aetherforge_sim`** (see `crates/aetherforge_sim/Cargo.toml`).
- When enabled:
  - `Simulation` owns `farm_world: FarmWorld`.
  - Intent **`farm_plant`** appends one stub plot (deterministic coord `(n,0)` where `n` is prior plot count, crop id **`stub_crop`**, `growth_stage` **0**). No growth/timer yet (Phase 5b).
  - `snapshot_observation()` sets **`farm`: `Some(FarmSnapshot::from_world(&farm_world))`** so JSON always includes a `farm` object when this feature is on (including empty `plots` / `inventory` at tick 0).
- **Default builds** (feature **off**): `farm` is **`None`** and is **omitted** from JSON (`skip_serializing_if`) — same wire shape as pre–Phase 5.

## Control plane / CLI

- Optional feature **`farm-stub`** on **`aetherforge_control`** forwards to **`aetherforge_sim/farm-stub`**.
- **`aetherforge_cli`**: `farm-stub = ["aetherforge_control/farm-stub"]` — use  
  `cargo build -p aetherforge_cli --features farm-stub --bin aetherforge_serve`  
  to serve observations that include `farm`.

## Schemas & SDK

- **`schemas/v1/observation.schema.json`** — optional top-level **`farm`** (not in `required`) so default responses stay valid; when present, matches the stub shape (`plots`, `inventory`).
- Python **`Observation.farm`**: optional **`FarmSnapshot | None`** (`python/aetherforge_sdk`).

## Tests

- `cargo test -p aetherforge_sim --features farm-stub` — includes **`farm_plant_adds_plot_to_observation`**, **`farm_demo_loop_plant_grow_harvest`**, etc.
- Default `cargo test` (no feature) — unchanged HTTP / wire tests.

## Phase 5b — Time + growth (still `farm-stub`)

- **`FarmWorld.day`:** starts at **1**; intent **`farm_advance_day`** increments it by **1** and increases each plot’s **`growth_stage`** by **1** up to **`MAX_GROWTH_STAGE`** (`3`) from `aetherforge_farm`.
- **`FarmSnapshot.time_minutes`:** `(sim_tick % LOGICAL_TICKS_PER_DAY) * MINUTES_PER_SIM_TICK` with `LOGICAL_TICKS_PER_DAY = 24`, `MINUTES_PER_SIM_TICK = 60` (documented in `aetherforge_farm`).
- Intent **`farm_harvest`:** removes plots whose **`growth_stage` ≥ `MAX_GROWTH_STAGE`**; for each, increments **`inventory.items["harvested_{crop_id}"]`** by **1**; unripe plots stay in **`plots`**.
- **Play log:** event **`farm_day_advanced`** (payload `{ "day": … }`) when `AETHERFORGE_PLAY_LOG=1` and control plane is built with **`farm-stub`**, after each **`farm_advance_day`** action (single or batch). Event **`farm_harvested`** (payload `plots_remaining`, `harvested_item_total`) after **`farm_harvest`** (single or batch step).
- **Example:** `examples/farm_5b_scenario.json` — run with  
  `cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_5b_scenario.json`  
  (requires matching `aetherforge_cli` / `aetherforge_sim` **farm-stub**).
- **Full loop example:** `examples/farm_demo_loop.json` — plant → three day advances → harvest (see `docs/demo-showcase.md`).

## Rationale

Feature-flagged domain data avoids breaking the control-plane contract on CI while allowing local/agent builds to exercise farm-shaped observations early. **Omit `farm` when disabled** rather than `null`, so existing clients ignore the field entirely.
