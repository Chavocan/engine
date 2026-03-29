# Phase 6 — Autonomous player (design only)

**Status:** Phase **6b** adds **`aetherforge_player`** (`crates/aetherforge_cli`) — HTTP-only loop per below. This document remains the architectural source of truth.

## 1. Observation-only policy

- The autonomous **player** is an external process: it may use only the **HTTP control plane** (`/v1/...`) or the **Python SDK** (`AetherForgeClient`).
- It **must not** link or import `aetherforge_sim::Simulation` (or other engine crates) in the player code path. That keeps the agent identical to a **remote** AI that only sees wire JSON.
- Local tooling may still use **`aetherforge_scenario --offline`** for CI; the **player** loop described here is **network/SDK**-first.

## 2. Control loop

1. **Create session** (`POST /v1/sessions`, optional `seed`).
2. **Read observation** (`GET .../observation`).
3. **Choose** one or more intents (policy-specific).
4. **Apply** via `POST .../action` or `POST .../actions` (batch when useful).
5. Repeat until a **terminal condition** (below).

Terminal conditions (configurable):

- **`tick >= T`** for a fixed horizon.
- **Farm predicate** (when server has `farm-stub`): e.g. `farm.plots[0].growth_stage >= N` or `farm.day >= D`.
- **Max actions** (hard cap per session) — see Safety.

## 3. Policies

| Policy | Behavior |
|--------|-----------|
| **Random** | Uniform choice over a fixed allowlist of `kind` strings (seeded RNG for reproducibility). |
| **Round-robin** | Cycle through the allowlist in order. |
| **LLM adapter** | Subprocess with **stdin/stdout JSON lines**: e.g. send last observation JSON line, read back `{ "kind": "...", "payload": {} }` or `{ "actions": [...] }`. No vendor SDK inside the engine; the adapter wraps the provider. |

Policies are swappable behind one trait/interface in the future binary; this doc defines **behavior**, not crate layout.

## 4. Determinism and replay

- **Same `seed`** + **same policy** (including RNG seed for random policy) + **same server build** ⇒ same sequence of observations **if** the server is deterministic (current kernel design goal).
- **Replay:** store `(seed, policy id, policy config, action log or scenario file)`; re-run player or replay actions via **`examples/farm_5b_scenario.json`**-style scripts.
- **Logs:** enable **`AETHERFORGE_PLAY_LOG=1`** on the server to correlate `run_id`, `tick`, and events (`farm_day_advanced`, batch/single applies).

## 5. Safety

- **Max actions per session:** stop the loop after **N** mutations (single + batch steps count per inner action or per HTTP call—pick one at implementation time and document).
- **Rate limits:** optional sleep between polls; configurable for soak vs interactive.
- **Kill switch:** e.g. **`AETHERFORGE_PLAYER_ABORT=1`** or drop file path checked each iteration (exact mechanism TBD in 6b).

## 6. MVP milestone

**Golden path:** HTTP session with **`farm-stub`** server — **plant → advance day → assert stage** using the existing scenario as contract:

- Script: **`examples/farm_5b_scenario.json`** (offline analogue); MVP player should reproduce the same **tick** and **`farm.plots[0].growth_stage`** progression over **live** `aetherforge_serve` with `--features farm-stub`.

Success = observation after the loop matches the scenario’s implied end state (document exact assertions in 6b).

## Relation to existing tools

- **`aetherforge_scenario`:** declarative JSON steps; good for CI and golden files.
- **`aetherforge_player` (6b):** imperative loop + policies (`random` / `round_robin`); optional **`--llm-exec`** + repeatable **`--llm-arg`** (argv only — **no** `sh -c` by default; avoid shell injection). May **generate** scenario traces for regression later.

**Binary:** `cargo run -p aetherforge_cli --bin aetherforge_player -- --help`

---

## Documentation & Learning Log (Employee AI)

- **Paths touched this slice:** `docs/phase6-autonomous-player.md` (this file). Optional cross-link from **`README.md`** under Docs (if not already listing Phase 6).
- **Processes:** Observation-only boundary defers all world logic to the server; player code stays a thin HTTP client.
- **Pitfalls:** LLM subprocess contract must be **line-delimited JSON** with timeouts to avoid hung agents.
- **Learnings:** Reusing **`examples/farm_5b_scenario.json`** as the MVP oracle ties Phase 4b/5b work to Phase 6 without new fixtures.
- **Next:** Phase **6b** — thin `aetherforge_player` binary implementing random + round-robin + config file, pending Director approval.
