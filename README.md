# AetherForge Engine (greenfield bootstrap)

Phase 1c workspace: Rust simulation kernel + stub platform/game + control hook + CLI headless runner.

## Repository

Canonical Git remote: **[github.com/Jabraham85/Engine](https://github.com/Jabraham85/Engine)**

```bash
git clone https://github.com/Jabraham85/Engine.git
```

## Prerequisites

- [Rust](https://rustup.rs/) stable (2021 edition)

## Commands

```bash
cargo build
cargo test
cargo test -p aetherforge_control
cargo run -p aetherforge_cli --bin aetherforge_headless
cargo run -p aetherforge_cli --bin aetherforge_serve
cargo run -p aetherforge_cli --bin aetherforge_scenario -- --offline examples/scenario-min.json
cargo run -p aetherforge_cli --bin aetherforge_player -- --base-url http://127.0.0.1:8787 --policy round_robin --max-steps 5 --intents noop
```

The headless binary applies a stub intent, advances **one** tick, and prints **one** JSON line (observation) to stdout.

**Scenario runner (Phase 4b):** `aetherforge_scenario` runs a JSON script of `batch` / `single` steps; use **`--offline`** for in-process soak (no server). See **`docs/phase4b-scenario.md`**.

`aetherforge_serve` binds **`127.0.0.1:8787`** (HTTP only — no WebSocket in Phase 1d).

**Session action cap (Phase 7a):** optional env **`AETHERFORGE_MAX_ACTIONS_PER_SESSION`** (default `10000`) — returns **429** + `SESSION_ACTION_QUOTA` when exceeded. See **`docs/phase7a-server-hardening.md`**.

### Play log (Phase 2c)

Set **`AETHERFORGE_PLAY_LOG=1`** to emit JSON play-log lines for AI tailing (see `docs/phase2c-play-log.md`).

## Python SDK (Phase 3a)

See **`python/aetherforge_sdk/README.md`** — `AetherForgeClient` + Pydantic models; optional E2E with **`AETHERFORGE_TEST_URL`**.

**CI:** see **`CONTRIBUTING.md`** (`.github/workflows/ci.yml` + optional `sdk-e2e.yml`).

**Release:** **`docs/release-v0.1.0.md`** (checklist + commands for tagging v0.1.0).

## Layout

| Path | Role |
|------|------|
| `crates/aetherforge_sim` | Authoritative `Simulation` |
| `crates/aetherforge_farm` | Farming data types (Phase 5a stub) |
| `crates/aetherforge_control` | `ai_driver_enqueue_intent` (**AI_DRIVER_HOOK**) |
| `crates/aetherforge_schemas` | Rust v1 types (expand with `schemars`) |
| `crates/aetherforge_platform` | Headed stub |
| `crates/aetherforge_game` | Game content stub |
| `crates/aetherforge_cli` | `aetherforge_headless`, `aetherforge_serve`, `aetherforge_scenario`, `aetherforge_player` |
| `schemas/v1/` | JSON Schema placeholders |
| `docs/` | Phase memos + specs |

## Docs

- `docs/roadmap-to-complete-project.md` — **remaining work to v1 / demo**
- `docs/demo-showcase.md` — **5-minute demo script**
- `docs/aetherforge-director-log.md` / `docs/aetherforge-designer-log.md` — program logs
- `docs/phase1a-stack-memo.md`
- `docs/phase1b-subsystem-architecture.md`
- `docs/phase1c-ai-interface-spec-v0.md`
- `docs/phase1c-risk-register.md`
- `docs/phase1d-verification.md`
- `docs/phase2-plan.md`
- `docs/phase2c-play-log.md`
- `docs/phase4-batch.md`
- `docs/phase4b-scenario.md`
- `docs/phase5-farm-stub.md`
- `docs/phase6-autonomous-player.md`
- `docs/phase7a-server-hardening.md`
- `docs/phase7b-player-graph-guard.md`
- `docs/release-v0.1.0.md`
- `docs/backlog-post-v0.1.md`
- `docs/adr/0001-observation-stream-transport.md`
- `docs/phase9b-sse.md`
