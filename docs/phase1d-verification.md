# Phase 1d — Verification & UNTESTED retirement

## Automated tests (proven)

| Claim | Test file | Test name |
|-------|-----------|-----------|
| `POST /v1/sessions` returns `session_id` + `schema_version` + `seed` | `crates/aetherforge_control/tests/http_sessions.rs` | `create_session_returns_ids` |
| Create → action → observation round-trip; observation has required fields (incl. `rng_draw`) | same | `round_trip_action_observation` |
| 404 error JSON shape for missing session | same | `unknown_session_404_shape` |
| `DELETE /v1/sessions/{id}` then observation 404 | same | `delete_session_lifecycle` |
| Same injected seed ⇒ matching tick / `rng_draw` / message across two sessions (HTTP) | same | `http_same_seed_parallel_observation_probes_match` |
| Two sequential actions on one session advance tick without deadlock | same | `two_sequential_action_clients_no_deadlock` |
| `GET` observation body bytes == `wire` replay of same seed + intent | same | `http_get_observation_bytes_match_wire_replay` |
| Batch of 3 actions vs 3 single posts: same final `tick` / `message` / `rng_draw` (same seed, different `session_id`) | same | `batch_three_noops_matches_three_single_posts_tick` |
| Batch with **>32** actions → **413** + `BATCH_TOO_LARGE` | same | `batch_too_many_actions_returns_413` |
| Empty `actions` array → **400** + `INVALID_BATCH` | same | `batch_empty_actions_returns_400` |
| Offline scenario runner JSON → `run_offline` happy path + tick assertion failure | `crates/aetherforge_cli/tests/scenario_offline.rs` | `offline_scenario_from_json_happy_path`, `offline_scenario_wrong_expect_tick_fails` |
| Farm stub: `farm_plant` adds one plot (feature **`farm-stub`**) | `crates/aetherforge_sim/src/lib.rs` (`farm_stub_tests`) | `farm_plant_adds_plot_to_observation` — run `cargo test -p aetherforge_sim --features farm-stub` |
| Farm stub: `farm_advance_day` bumps `growth_stage` + `day` | same | `farm_plant_then_advance_increases_stage` |
| Farm stub: `farm_harvest` moves ripe crops to `inventory`, clears ripe plots | same | `farm_demo_loop_plant_grow_harvest` — `cargo test -p aetherforge_sim --features farm-stub` |
| HTTP-only player: round-robin loop vs in-process Axum | `crates/aetherforge_player/tests/player_http_loop.rs` | `player_round_robin_stops_after_max_steps` |
| Player sources: no direct `use aetherforge_sim` lines | `crates/aetherforge_player/tests/player_no_sim_guard.rs` | `player_sources_have_no_direct_sim_import_lines`; CI: `scripts/check_player_no_sim_import.sh` |
| Session action quota: third single action → **429** | `crates/aetherforge_control/tests/http_action_quota.rs` | `single_actions_hit_quota_then_429` |
| Batch over quota: **no** partial apply, tick unchanged | same | `batch_rejected_when_would_exceed_quota_without_partial_apply` |
| SSE observation stream: event on tick change (`sse-obs`) | `crates/aetherforge_control/tests/sse_observe_stream.rs` | `observe_stream_emits_when_tick_changes` — `cargo test -p aetherforge_control --features sse-obs` |
| Python SDK SSE iterator (mock body) | `python/aetherforge_sdk/tests/test_client_mock.py` | `test_observe_stream_mock_sse_body` |
| `wire` bytes parse to same `serde_json::Value` as direct `Observation` serde | `crates/aetherforge_sim/src/wire.rs` | `wire_bytes_match_direct_serde_value` |
| Play-log JSON line has required keys (`ts`, `run_id`, `tick`, `event`, `session_id`, `payload`) | `crates/aetherforge_control/tests/play_log_json.rs` | `play_emit_produces_parseable_json_with_contract_keys` |
| `aetherforge_sim` tick advances | `crates/aetherforge_sim/src/lib.rs` | `step_advances_tick` (unit) |
| Same seed + intents ⇒ identical observation JSON (library) | same | `reproducibility_same_seed_same_observation_json` |

**Command:** `cargo test -p aetherforge_control` (runs integration + dependency unit tests as wired by Cargo).

## Still UNTESTED (explicit)

| Item | Notes |
|------|--------|
| WebSocket / SSE streaming | Deferred per Director 1d (not implemented). |
| TLS, auth, non-localhost bind | v0 spec; no tests. |
| JSON Schema CI validation vs Rust types | `schemars` generation not wired. |
| Concurrent HTTP stress (parallel mutating requests) | Single sequential mutator per session tested; parallel mutation stress not yet covered. |
| `tracing` JSON subscriber / play-log narrative | Not in 1d scope. |
| Headed vs headless parity | Platform crate still stub. |

## Serve binary

`cargo run -p aetherforge_cli --bin aetherforge_serve` — binds `127.0.0.1:8787`.
