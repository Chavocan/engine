# Agent master plan — what still needs care

Single place for **agents** to see what is closed, what is operational hygiene, and what is optional follow-on work. The engine roadmap table in [`roadmap-to-complete-project.md`](roadmap-to-complete-project.md) is **complete** for v0.1 engine scope; this doc orders **everything else** worth tracking.

## Finish line definitions (scoped “done”)

| Finish line | Meaning | Doc |
|-------------|---------|-----|
| **1 — Ops** | Tag on remote, green `main`, CHANGELOG aligned | [`release-closure-checklist.md`](release-closure-checklist.md) |
| **2 — Shippable control plane** | Bind config, deployment ADR, schema fragments, stress coverage, honest UNTESTED | [`deployment.md`](deployment.md), ADR [`adr/0003-deployment-tls-and-auth.md`](adr/0003-deployment-tls-and-auth.md), [`phase1d-verification.md`](phase1d-verification.md) |
| **3 — Product tracks** | NL stub route, headed roadmap | [`nl-agentic-hooks.md`](nl-agentic-hooks.md), [`platform-headed-roadmap.md`](platform-headed-roadmap.md) |
| **4 — Director program** | Godot/runtime + flagship + autonomous playthrough | [`director-program-roadmap.md`](director-program-roadmap.md) |

---

## Tier A — Operational (do once / occasional)

| Item | Action |
|------|--------|
| **Tag on remote** | If `v0.1.0` exists only locally, push tags when publishing: `git push origin v0.1.0` (or equivalent). |
| **CI on `main`** | After merges, confirm GitHub Actions green on `main`; fix failures before stacking more work. |
| **Continuous QA** | On every meaningful change: extend [`phase1d-verification.md`](phase1d-verification.md) if behavior is user-visible; run commands in [`CONTRIBUTING.md`](../CONTRIBUTING.md) locally or trust CI. |

---

## Tier B — Explicit gaps (documented UNTESTED / not built)

From [`phase1d-verification.md`](phase1d-verification.md) **Still UNTESTED** — not roadmap debt for “engine complete,” but **real product gaps** if you ship beyond local HTTP:

| Track | Gap | Typical next step |
|-------|-----|-------------------|
| **Transport** | WebSocket | Product choice vs SSE ([`backlog-post-v0.1.md`](backlog-post-v0.1.md)); only if required. |
| **Hardening** | TLS, auth, non-localhost bind | Design + integration tests when deploying publicly. |
| **Schemas** | Full `observation.schema.json` vs Rust auto-gen | Extend `check_schema_drift.py` / exports without breaking SDK contract; incremental. |
| **Stress** | Soak / load / parallel beyond current tests | Add targeted benches or CI job if SLOs matter. |
| **Observability** | Rich `tracing` narrative | Optional; play-log JSON already tested. |
| **Platform** | Headed parity | Real window loop per [`platform-headed-roadmap.md`](platform-headed-roadmap.md); today: `headed-smoke` compile only. |

---

## Tier C — Product tracks (optional milestones)

| Track | Doc | Notes |
|-------|-----|--------|
| **NL / agentic** | [`nl-agentic-hooks.md`](nl-agentic-hooks.md) | NL-1 sample done; NL-2 `interpret` route optional. |
| **Director program** | [`../aetherforge-lead-director.agent.md`](../aetherforge-lead-director.agent.md) | Godot/runtime, flagship sim — **separate program** from this engine repo checklist. |

---

## Tier D — CI / automation already in place (do not duplicate manually)

Default PR [**`ci.yml`**](../.github/workflows/ci.yml): `cargo test`, golden playthrough, SSE, schema drift, rate-limit tests, wgpu smoke, player guards, farm demo, Python ruff/contract/pytest, **`sdk-live-e2e`** (localhost SDK integration).

Optional [**`sdk-e2e.yml`**](../.github/workflows/sdk-e2e.yml): remote URL / tunnel / self-hosted — for bases other than runner localhost.

---

## Suggested execution order (when picking work)

1. **Tier A** if anything is red or unpushed.
2. **Tier B** items that match the next shipping environment (e.g. TLS before public demo URL).
3. **Tier C** when product asks for NL client or external runtime.
4. Avoid re-testing **Tier D** by hand — extend automated tests instead.

---

## Documentation & Learning Log

- **File:** `docs/agent-master-plan.md` — agent-oriented backlog; update when UNTESTED rows shrink or new tracks open.
- **Owner:** agents per [`AGENTS.md`](../AGENTS.md) and [`.cursor/rules/aetherforge-agent-ownership.mdc`](../.cursor/rules/aetherforge-agent-ownership.mdc).
