# Changelog

## Unreleased

- **Roadmap realignment:** **`docs/roadmap-to-complete-project.md`** — **North star** + **§ On the rails** (R0–R5) for Director mission; kernel v0.1 marked **closed** in a dedicated section. **`agent-master-plan.md`**, **`director-program-roadmap.md`**, **`README`**, **`.cursor/rules`**: mission ≠ kernel complete.
- **Usability:** **`docs/getting-started.md`** end-to-end path (clone → headless → scenario → HTTP → SDK); **`scripts/dev_smoke.sh`** / **`scripts/dev_smoke.ps1`** quick smoke (headless + offline farm scenario).
- **Deployment:** **`AETHERFORGE_HTTP_ADDR`** for **`aetherforge_serve`** bind (default **`127.0.0.1:8787`**); **`docs/deployment.md`** operator index; **ADR [`docs/adr/0003-deployment-tls-and-auth.md`](docs/adr/0003-deployment-tls-and-auth.md)** (TLS/auth at proxy; in-process deferred).
- **Finish-line docs:** **`docs/release-closure-checklist.md`**, **`docs/director-program-roadmap.md`**; **`docs/agent-master-plan.md`** finish-line table.
- **NL-2 stub:** Cargo feature **`nl-interpret-stub`** — **`POST /v1/sessions/{id}/interpret`** returns **501** + `NL_INTERPRET_NOT_IMPLEMENTED`; test **`nl_interpret_stub`** CI.
- **Schema drift:** **`scripts/check_schema_drift.py`** — observation **`farm`** + **`world`** property fragments vs **`schemas/v1/observation.schema.json`**.
- **Concurrent stress:** **`http_concurrent`** adds 48-way fan-out test.
- **Rate limiting:** optional Cargo feature **`rate-limit`** on **`aetherforge_control`** / **`aetherforge_cli`**; env **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`** enables per-IP **`governor`** limiter in **`aetherforge_serve`** (**429** `HTTP_RATE_LIMIT`). Docs: **`docs/deployment-rate-limiting.md`**; tests: **`http_rate_limit`**.
- **NL milestones (planning artifacts):** **`examples/nl-prompt-template.md`**, **`examples/nl_tool_use_sdk_sample.py`**; **`docs/nl-agentic-hooks.md`** updated.
- **Platform:** **`docs/platform-headed-roadmap.md`** (headed loop phases after compile smoke).

## [0.1.0] — 2026-03-29

### Added

- **`aetherforge_scenario`:** `--emit-tick-json` (stderr `{"event":"tick","tick":…}` per step) and `--quiet` (suppress tick lines); integration tests `scenario_tick_json`.
- **Breaking (`aetherforge_cli` library):** `run_http` now takes `on_tick: &mut F`; `run_offline_with_ticks` added; `run_offline` unchanged.
- **Breaking (CLI UX):** Autonomous player binary moved to workspace crate **`aetherforge_player`**. Use **`cargo run -p aetherforge_player -- ...`** instead of **`cargo run -p aetherforge_cli --bin aetherforge_player`** (ADR **`docs/adr/0002-player-crate-split.md`**). The player package has **no** `aetherforge_sim` dependency.
- Docs: **`SECURITY.md`**, **`docs/nl-agentic-hooks.md`** (NL/agentic design stub); dual **`LICENSE-MIT`** / **`LICENSE-APACHE`**; README **License** + **Security** sections.
- Control plane: batch actions (`POST .../actions`), session action quota (`429` / `SESSION_ACTION_QUOTA`), optional `farm-stub` farming slice with day/time, growth, and **`farm_harvest`** (ripe plots → `inventory`), JSON scenario runner (`aetherforge_scenario`), example `examples/farm_demo_loop.json`, HTTP-only player (`aetherforge_player`), Phase 6 design doc, server hardening and player-boundary CI checks.
- CI: **`rust` job** runs offline **`examples/farm_demo_loop.json`** via `aetherforge_scenario` with **`farm-stub`** (see `CONTRIBUTING.md`); asserts **`aetherforge_player`** has **no `aetherforge_sim`** on **`cargo tree -e normal`** runtime edges; **golden playthrough** script (`scripts/golden_playthrough.sh`); **schema drift** check (`scripts/check_schema_drift.py`); **wgpu** compile smoke (`aetherforge_platform` / `headed-smoke`).
- Python SDK: fix **`pyproject.toml`** — `classifiers` and `dependencies` belong under **`[project]`**, not **`[project.urls]`**, restoring valid **`pip install -e`** / setuptools validation.
- Docs: **`docs/adr/0002-player-crate-split.md`** — **Accepted**; player is now workspace crate **`aetherforge_player`** (no `aetherforge_sim` dep).
- Play log: **`farm_harvested`** event when **`AETHERFORGE_PLAY_LOG=1`** and action kind is **`farm_harvest`** (`farm-stub` builds only); payload includes `plots_remaining` and `harvested_item_total`. Optional **`AETHERFORGE_PLAY_LOG_STDOUT=1`** routes JSON play lines to **stdout** (human tracing stays on stderr).
- SSE (**`sse-obs`**): concurrent **`observe/stream`** limits per session and globally (`AETHERFORGE_SSE_MAX_PER_SESSION`, `AETHERFORGE_SSE_MAX_GLOBAL`); **429** with **`SSE_SESSION_CAP`** / **`SSE_GLOBAL_CAP`** when exceeded.
- Schema: **`schema-export`** feature on **`aetherforge_schemas`** + binary **`aetherforge_export_action_schema`**; `Action` aligns with **`schemas/v1/action.schema.json`** (CI drift check).
- Platform: **`headed-smoke`** feature + binary **`aetherforge_wgpu_smoke`** (headless wgpu instance).

### Notes

- Optional **per-IP** HTTP rate limiting was added after the **v0.1.0** tag; see **`CHANGELOG` Unreleased** and **`docs/deployment-rate-limiting.md`**.
