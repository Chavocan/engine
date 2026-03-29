# Roadmap — complete project + demo

Living summary aligned with the Cursor **AetherForge Director** plan. Update when milestones close.

## Done (high level)

- Greenfield Rust workspace, simulation kernel, HTTP control plane (`/v1/sessions`, actions, batch, observation, optional SSE stream).
- Headless binary, serve binary, JSON scenario runner, HTTP-only autonomous player, play logs, session quotas.
- `farm-stub` feature: types, plant, advance day, observation `world` + time fields.
- Python SDK + contract check script + mock tests; CI (Rust + Python) in `.github/workflows/ci.yml`.
- Phase design docs under `docs/phase*.md`, ADR 0001, release checklist `docs/release-v0.1.0.md`, backlog `docs/backlog-post-v0.1.md`.

## Remaining for “complete” engine product

| Priority | Item | Done when |
|----------|------|-----------|
| P0 | **v0.1.0 tag** | Checklist in `docs/release-v0.1.0.md` green; `CHANGELOG` `[0.1.0]` dated. |
| P0 | **Farm gameplay loop** | **Done (stub):** `farm_harvest` + `farm_demo_loop_plant_grow_harvest` + `examples/farm_demo_loop.json`. |
| P0 | **Demo showcase doc** | `docs/demo-showcase.md` — run offline scenario per doc; verify on your machine (`cargo`). |
| P1 | **CI golden playthrough** | Workflow (or script) runs scenario or `aetherforge_player` against `serve` (or offline-only first) and fails on error. |
| P1 | **Player crate split** | Separate crate; `cargo tree` shows no `aetherforge_sim` dep for player package. |
| P2 | **Schema CI** | Generated or checked schemas match `schemas/v1/` in CI. |
| P2 | **SSE caps + play-log stderr** | Per `docs/backlog-post-v0.1.md`. |
| P3 | **Headed wgpu smoke** | Feature-gated window or documented CI skip. |
| P3 | **Product polish** | **Partial:** `LICENSE-MIT`, `LICENSE-APACHE`, `SECURITY.md`, `docs/nl-agentic-hooks.md`; README License/Security sections. |

## Continuous QA

- Local: `cargo test`, `bash scripts/check_player_no_sim_import.sh`, Python steps in `docs/release-v0.1.0.md`.
- Remote: push to GitHub triggers `ci.yml`; optional `sdk-e2e.yml` for live HTTP tests.
- Each feature: extend `docs/phase1d-verification.md` (or successor) and retire **UNTESTED** where possible.

## Institutional logs

- `docs/aetherforge-director-log.md` — Director cycles.
- `docs/aetherforge-designer-log.md` — Implementation cycles.

Same bullet headings as in `.cursor/agents/aetherforge-*.agent.md`.
