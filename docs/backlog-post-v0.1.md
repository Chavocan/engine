# Backlog (post–v0.1.0)

Prioritized ideas for stakeholder triage. **Pick one of WebSocket vs SSE** where noted.

| Pri | Item | One-liner + acceptance sketch |
|-----|------|--------------------------------|
| P1 | **Observation stream (SSE)** | **Shipped (ADR 0001 + Phase 9b):** `sse-obs`, `GET .../observe/stream`; CI runs `observe_stream_emits_when_tick_changes`. |
| P1 | **`aetherforge_player` crate split** | **Done (ADR 0002):** crate `crates/aetherforge_player`; optional verify `cargo tree -p aetherforge_player -e normal` has no `aetherforge_sim`. |
| P2 | **Per-IP rate limit** | **Done:** optional **`rate-limit`** feature + **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`** (**429** `HTTP_RATE_LIMIT`); **`docs/deployment-rate-limiting.md`** documents reverse-proxy-first production layout. |
| P2 | **SSE connection caps** (see **ADR 0001**) | **Done:** per-session + global semaphores; **429** `SSE_SESSION_CAP` / `SSE_GLOBAL_CAP`; test `second_observe_stream_returns_429_when_cap_is_one`. |
| P2 | **Play-log stderr split for headless** | **Done:** `AETHERFORGE_PLAY_LOG_STDOUT=1` routes play JSON to stdout (human tracing on stderr). |
| P3 | **schemars + schema CI** | **Done:** `schema-export` + `scripts/check_schema_drift.py` (action export + observation farm fragment vs `schema_fragments/observation_farm_property.json`). |
| P3 | **Headed `winit` / `wgpu` smoke** | **Done (compile):** `headed-smoke` + `aetherforge_wgpu_smoke` headless instance; CI `cargo build -p aetherforge_platform --features headed-smoke`. |

---

## Documentation & Learning Log (Employee AI)

- **File:** `docs/backlog-post-v0.1.md` — compact P1–P3 for planning, not commitment.
- **WS vs SSE:** left as an explicit product choice in the first row to avoid designing both.
- **Relay:** Director keepalive noted; agent B continues on new slices when issued.
