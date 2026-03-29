# Director program — finish line 4 (cross-repo / program)

This document tracks the **Lead Director** mission from [`../aetherforge-lead-director.agent.md`](../aetherforge-lead-director.agent.md). It is **not** the same as the **engine workspace** checklist in [`roadmap-to-complete-project.md`](roadmap-to-complete-project.md); it spans runtime choice, flagship content, and final autonomous validation.

## Phases (ordered)

| Phase | Outcome | Engine repo role |
|-------|---------|------------------|
| 1 | Planning and architecture locked | ADRs, HTTP contracts, schemas — largely present |
| 2 | Core engine framework + extension points | Crates, sim, control plane — present for v0.1 scope |
| 3 | AI designer interface layer | REST (+ SSE); Python SDK; NL sidecar path — **in progress** (NL-2 optional in-process) |
| 4 | Headless AI player and simulation | `aetherforge_player`, scenarios, play logs — present |
| 5 | Game creation tools and example game | **Flagship farming sim** as standalone product — **not** complete in this repo alone |
| 6 | Polish, optimization, documentation | Continuous |
| 7 | Final validation — **full autonomous playthrough** on flagship | **Proof milestone** — requires Phase 5 deliverable |

## Relationship to [`agent-master-plan.md`](agent-master-plan.md)

- **Tiers A–D** in the agent master plan cover **this repository**.
- **This file** is the **program-level** backlog for Godot/runtime integration, shipped flagship title, and end-to-end AI playthrough proof.

## Next actions (when prioritizing the program)

1. Choose or confirm **runtime** (e.g. Godot 4) and integration boundary with the Rust kernel or HTTP control plane.
2. Stand up **reference game project** (Harvest-Moon-style) using engine APIs.
3. Automate **autonomous playthrough** validation (metrics, pass/fail) on CI or a scheduled runner.
