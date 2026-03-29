# Phase 2 plan — core loop hardening

**Status:** Phases **2a–2c** are **Director-signed** on review; local proof remains **`cargo test` / `pytest` green** on stakeholder/CI machines (relay agents may lack toolchains).

Slices are ordered; Phase **3a** (Python SDK) is in progress in-repo unless the Director reopens scope.

## 2a — Determinism hooks (**landed**)

- **Seeded RNG:** `Simulation::with_config(SimulationConfig { run_id, seed })`; `StdRng::seed_from_u64`. Optional `seed` on `POST /v1/sessions`; response echoes `seed`.
- **Probe field:** `Observation.rng_draw` updated every `step` (and initial draw at construction).
- **Tick vs wall clock:** Logical tick advances only inside `Simulation::step`; callers (CLI, Axum) batch wall time into discrete steps (**no wall clock inside kernel**).
- **Tests:** `crates/aetherforge_sim/src/lib.rs` — `reproducibility_same_seed_same_observation_json`; `crates/aetherforge_control/tests/http_sessions.rs` — `http_same_seed_parallel_observation_probes_match`, `two_sequential_action_clients_no_deadlock`.
- **Concurrency debt (Phase 1d):** Session `Simulation` now uses **`tokio::sync::Mutex`** (async-safe); session map remains `std::sync::Mutex` (held briefly).

**UNTESTED (explicit):** Floating-point non-determinism (no FP in kernel yet); multi-threaded physics; cross-platform RNG byte-identical guarantees beyond `rand` `StdRng` docs.

## Version / bump policy (observation + world)

- **`Observation.schema_version`:** **1.1.0** introduces required **`world`**. Patch bumps = additive optional top-level fields only; minor = new sub-objects; major = rename/remove fields or change `world` contract.
- **`WorldSnapshot.world_version`:** patch = additive; minor = new optional arrays/maps under `world`; major = breaking entity shape. Keep `entities` present (empty array allowed) so AI consumers always have a stable attachment point.

## 2b — Richer observation (**landed**)

- **`world`:** `WorldSnapshot { world_version, entities }` on `aetherforge_sim::Observation` (default empty `entities`, `world_version = "1.0.0"`).
- **Parity:** `aetherforge_sim::wire::observation_to_vec` / `observation_to_string` is the **only** encoder for headless stdout and `GET .../observation` body (`crates/aetherforge_control/src/server.rs`). Unit test `wire_bytes_match_direct_serde_value`; integration test `http_get_observation_bytes_match_wire_replay`.
- **Schemas:** `schemas/v1/observation.schema.json` updated. **`aetherforge_schemas::v1::Observation`** mirrors the shape for future SDK — drift vs kernel is **UNTESTED** until schema-generation CI exists.

## 2c — Play-log subscriber (**landed**)

- **`AETHERFORGE_PLAY_LOG=1`:** JSON **flattened** `fmt` layer for `target = "aetherforge.play"` + plain-text layer for other targets — `crates/aetherforge_control/src/play_log.rs`.
- **Events:** `session_created`, `session_deleted`, `action_applied`, `tick_advanced`, `observation_served` (payload ≤2048 bytes; observation line carries `schema_version` only).
- **Test:** `crates/aetherforge_control/tests/play_log_json.rs` — `play_emit_produces_parseable_json_with_contract_keys`.
- **Docs:** `docs/phase2c-play-log.md`. **No WebSocket** in 2c.

---

### Documentation & Learning Log (Employee AI)

- **Accomplished this cycle:** Phase **2a–2c** + **3a** Python SDK (httpx + Pydantic, optional E2E pytest).
- **Processes used:** Single `observation_to_vec` path; integration replay vs `GET` bytes.
- **Pitfalls / observations:** `aetherforge_schemas` mirror can drift without codegen CI.
- **Learnings / best practices:** Build HTTP observation `Response` from `wire` bytes, not `axum::Json`, to lock parity.
- **Next cycle action items:** Director 3a review; async httpx or WebSocket per product priority.

**UNTESTED (2b):** JSON Schema ↔ Rust CI validation; schema mirror drift; pretty vs compact JSON for headless (both compact); non-Rust SDK decode.

**UNTESTED (2c):** File sink / rotation env vars; interleaved stdout when headless prints observation + play JSON; production log volume controls.

## 3a — Python SDK (thin client) (**landed**)

- **Path:** `python/aetherforge_sdk/` — package **`aetherforge_sdk`** (Pydantic models + sync **`httpx`** `AetherForgeClient`).
- **API:** `create_session(seed=...)`, `apply_action`, `get_observation`, `delete_session` — mirrors v1 HTTP + **`Observation` 1.1.0**.
- **Tests:** `pytest` in `tests/test_client_integration.py` — skipped unless **`AETHERFORGE_TEST_URL`** points at a live `aetherforge_serve`.
- **Docs:** `python/aetherforge_sdk/README.md`.
