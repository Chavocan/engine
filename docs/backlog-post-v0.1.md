# Backlog (post–v0.1.0)

Prioritized ideas for stakeholder triage. **Pick one of WebSocket vs SSE** where noted.

| Pri | Item | One-liner + acceptance sketch |
|-----|------|--------------------------------|
| P1 | **Observation stream (SSE)** | **ADR 0001** + **Phase 9b** (`sse-obs`, `GET .../observe/stream`); **acceptance:** met by `observe_stream_emits_when_tick_changes` (feature-gated CI step). |
| P1 | **`aetherforge_player` crate split** | Own package with no `aetherforge_sim` dependency so `cargo tree -p … --bin … -i aetherforge_sim` is empty; **acceptance:** CI script exits 0 on that graph check. |
| P2 | **Per-IP rate limit** | Token bucket (or proxy-only doc) for abusive HTTP clients; **acceptance:** optional `rate-limit` feature + one test with synthetic burst → **429** or documented defer to reverse proxy. |
| P2 | **SSE connection caps** (see **ADR 0001**) | Limit concurrent `observe/stream` per **session** (e.g. single tail per `session_id`) and/or **global** fan-out; **acceptance:** configurable max + **429** or disconnect with stable error when exceeded. |
| P2 | **Play-log stderr split for headless** | Optional sink so headless tools can tail JSON lines without mixing sim `tracing`; **acceptance:** env flag + one test capturing distinct stream. |
| P3 | **schemars + schema CI** | Generate JSON Schema from Rust types and diff in CI; **acceptance:** workflow step fails on drift vs `schemas/v1/`. |
| P3 | **Headed `winit` / `wgpu` smoke** | Minimal window + clear color behind feature flag; **acceptance:** `cargo run` with feature opens and exits cleanly in CI (or marked skip with reason). |

---

## Documentation & Learning Log (Employee AI)

- **File:** `docs/backlog-post-v0.1.md` — compact P1–P3 for planning, not commitment.
- **WS vs SSE:** left as an explicit product choice in the first row to avoid designing both.
- **Relay:** Director keepalive noted; agent B continues on new slices when issued.
