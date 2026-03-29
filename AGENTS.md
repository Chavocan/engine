# Agent instructions (AetherForge Engine)

This repository expects **AI coding agents** to drive implementation and verification. The human sets product direction and constraints; **agents own execution, tests, and CI parity**.

## Authority

| Area | Owner |
|------|--------|
| Code changes, tests, scripts, CI alignment | Agent (within user instructions) |
| Merge/release confidence | **Green GitHub Actions** — see [`CONTRIBUTING.md`](CONTRIBUTING.md) § Canonical QA |
| What to build next, scope, “ship it” | Human |

## Before pushing

- Run or rely on the same checks as [`.github/workflows/ci.yml`](.github/workflows/ci.yml): `cargo test`, golden playthrough (`scripts/golden_playthrough.sh` or `scripts/golden_playthrough.ps1`), feature tests as listed in [`CONTRIBUTING.md`](CONTRIBUTING.md), Python SDK steps, `scripts/check_schema_drift.py` when schemas change.
- Do not ask the human to manually substitute for automated QA.

## Pointers

- **Roadmap (engine scope):** [`docs/roadmap-to-complete-project.md`](docs/roadmap-to-complete-project.md)
- **Master backlog / finish lines:** [`docs/agent-master-plan.md`](docs/agent-master-plan.md)
- **Ops closure (tags, CI):** [`docs/release-closure-checklist.md`](docs/release-closure-checklist.md)
- **Deployment (bind, TLS ADR):** [`docs/deployment.md`](docs/deployment.md)
- **Director program (phases 1–7):** [`docs/director-program-roadmap.md`](docs/director-program-roadmap.md)
- **Verification table:** [`docs/phase1d-verification.md`](docs/phase1d-verification.md)
- **Broader program (Godot, flagship sim, etc.):** [`aetherforge-lead-director.agent.md`](aetherforge-lead-director.agent.md) — not the same as “engine crate checklist”

## Cursor

Persistent rule: [`.cursor/rules/aetherforge-agent-ownership.mdc`](.cursor/rules/aetherforge-agent-ownership.mdc)
