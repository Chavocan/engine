# Roadmap — north star, kernel, and what is still open

This file is the **single alignment point** between the **repo** and the **[Lead Director program](../aetherforge-lead-director.agent.md)**.  
**Finishing the kernel checklist below is not the same as finishing the mission.** The mission is: **an AI can use the stack to design, drive, and QA a game end-to-end** — see **§ On the rails**.

---

## North star (non-negotiable product goal)

From [`aetherforge-lead-director.agent.md`](../aetherforge-lead-director.agent.md):

- **Design** games via APIs / structured tools (and eventually NL), without humans in the loop for routine iteration.
- **Run** headless simulation with **zero required human input** for scripted or autonomous play.
- **Receive** rich structured state (observations, logs) for QA and iteration.
- **Prove** completion with **end-to-end autonomous playthrough** on a **flagship** reference — not only “HTTP returns 200.”

If work only improves the control plane but **does not advance** the table in **§ On the rails**, treat it as **supporting work**, not “we shipped the product.”

---

## Kernel and control plane (v0.1 — closed)

The **AetherForge Engine** workspace hit its **v0.1** scope: sim + HTTP + SDK + CI + farm stub + docs. This section stays as the **record of what shipped**.

### Done (high level)

- Greenfield Rust workspace, simulation kernel, HTTP control plane (`/v1/sessions`, actions, batch, observation, optional SSE stream).
- Headless binary, serve binary, JSON scenario runner, HTTP-only autonomous player, play logs, session quotas.
- `farm-stub` feature: types, plant, advance day, observation `world` + time fields.
- Python SDK + contract check script + mock tests; CI (Rust + Python) in `.github/workflows/ci.yml`.
- Phase design docs under `docs/phase*.md`, ADR 0001, release checklist `docs/release-v0.1.0.md`, backlog `docs/backlog-post-v0.1.md`.

### Kernel checklist (all done)

| Priority | Item | Done when |
|----------|------|-----------|
| P0 | **v0.1.0 tag** | **Done:** `CHANGELOG.md` **\[0.1.0\]** dated **2026-03-29**; checklist in `docs/release-v0.1.0.md`; tag **`v0.1.0`**. |
| P0 | **Farm gameplay loop** | **Done (stub):** `farm_harvest` + `farm_demo_loop_plant_grow_harvest` + `examples/farm_demo_loop.json`. |
| P0 | **Demo showcase doc** | **Done:** `docs/demo-showcase.md` recording checklist verified; offline scenario commands exercised. |
| P1 | **CI golden playthrough** | **Done:** `scripts/golden_playthrough.sh` + CI step (offline farm + `http_sessions` + `player_http_loop`). |
| P1 | **Player crate split** | **Done:** `crates/aetherforge_player` (ADR 0002). `cargo tree -p aetherforge_player -e normal` has no `aetherforge_sim`. |
| P2 | **Schema CI** | **Done:** `scripts/check_schema_drift.py` + `aetherforge_export_action_schema` + observation fragments. |
| P2 | **SSE caps + play-log stderr** | **Done:** SSE caps + env (`AETHERFORGE_SSE_*`); play log **`AETHERFORGE_PLAY_LOG_STDOUT=1`**. |
| P3 | **Headed wgpu smoke** | **Done:** `cargo build -p aetherforge_platform --features headed-smoke`; `aetherforge_wgpu_smoke`. |
| P3 | **Product polish** | **Done:** `LICENSE-MIT`, `LICENSE-APACHE`, `SECURITY.md`, `docs/nl-agentic-hooks.md`; README License/Security. |

---

## On the rails — Director program (open until proven)

These rows are the **real** “percent complete” for the **game + AI mission**. Order is a guide; parallelize where it makes sense.

| Priority | Milestone | Done when |
|----------|-----------|------------|
| **R0** | **Explicit win/lose game vertical** | **Done (v1):** `Observation.mission.outcome` (`won` / `lost`) when **`farm-stub`** harvest completes; **`expect_mission_outcome`** in scenario JSON; **`examples/farm_demo_loop.json`** asserts **`won`**; tests **`scenario_mission_offline`**, sim **`farm_demo_loop_plant_grow_harvest`**. HTTP parity to **`won`**: CI job **`farm-mission-http-e2e`** (R4). Extend with **non-farm** outcomes later. |
| **R1** | **Flagship scenario depth** | **Done (v1):** **`examples/flagship_farm_two_cycles.json`** (two plant→grow→harvest cycles) + **`docs/flagship-scenario.md`**; offline test **`scenario_flagship_offline`**. |
| **R2** | **Client surface for “play”** | Move past compile-only platform: **either** [`platform-headed-roadmap.md`](platform-headed-roadmap.md) **P1** (window + clean exit) **or** a minimal **web/terminal HUD** that renders observation state — so “game” is not only JSON in a terminal. |
| **R3** | **Runtime / embedding decision** | **ADR or doc**: Godot vs Unity vs web-first vs Rust-only — **committed** tradeoffs, how `aetherforge_serve` or in-proc sim plugs in. |
| **R4** | **Autonomous playthrough proof** | **Done (v1):** CI job **`farm-mission-http-e2e`** runs **`aetherforge_scenario`** (HTTP) on **`examples/flagship_farm_http.json`** with **`farm-stub`** serve — asserts **`mission.outcome`** **`won`** end-to-end. |
| **R5** | **NL / designer path (optional but on-mission)** | Sidecar or in-process path from natural language to **validated** actions — milestone table in [`nl-agentic-hooks.md`](nl-agentic-hooks.md) advanced beyond stub **501** where product requires it. |

Detail and phase mapping: [`director-program-roadmap.md`](director-program-roadmap.md).

---

## Continuous QA

- **Kernel regression:** `cargo test`, golden playthrough, Python SDK steps — see [`CONTRIBUTING.md`](../CONTRIBUTING.md) and `ci.yml`.
- **Mission regression:** extend tests and CI so **§ On the rails** rows gain **observable proof** (not just docs).
- **Verification table:** [`phase1d-verification.md`](phase1d-verification.md) — includes **R0** mission / scenario rows.
- **Agents:** [`AGENTS.md`](../AGENTS.md), [`agent-master-plan.md`](agent-master-plan.md).

## Institutional logs

- `docs/aetherforge-director-log.md` — Director cycles.
- `docs/aetherforge-designer-log.md` — Implementation cycles.
