# Director program — cross-repo / program

Tracks the **Lead Director** mission from [`../aetherforge-lead-director.agent.md`](../aetherforge-lead-director.agent.md).

**Authoritative ordered backlog for “getting back on the rails”:** [`roadmap-to-complete-project.md`](roadmap-to-complete-project.md) **§ On the rails** (R0–R5). This file expands **phase** context and **next actions**.

## Phases (from Director doc)

| Phase | Outcome | Status (honest) |
|-------|---------|-----------------|
| 1 | Planning and architecture locked | Largely done (ADRs, HTTP, schemas) |
| 2 | Core engine framework + extension points | v0.1 kernel **done** |
| 3 | AI designer interface layer | REST + SDK **done**; runtime embedding **ADR 0004**; NL depth **partial** |
| 4 | Headless AI player and simulation | Player + scenarios **done**; **winning vertical** **done (v1)** (R0 mission + R4 HTTP assert) |
| 5 | Game creation tools and example game | **Flagship** (R1); **terminal + headed client** (R2) — Python HUD + **`aetherforge_window`** (P1–P4: sim + input→intent) |
| 6 | Polish, optimization, documentation | Continuous |
| 7 | Final validation — full autonomous playthrough | **Partial (v1):** R4 CI HTTP scenario to **`won`**; full **agent** loop still future |

## Next actions (prioritize in order)

1. **Platform** — In-window text/overlay, render farm state, or Godot shell ([`platform-headed-roadmap.md`](platform-headed-roadmap.md) beyond P4).
2. **R4+** — Deeper **agent** autonomous loop (beyond scenario harness) or additional outcomes / non-stub content as product requires.
3. **R5** — NL depth where [`nl-agentic-hooks.md`](nl-agentic-hooks.md) milestones demand it.
4. **Godot or web shell** — out-of-repo client using ADR 0004; keep **`aetherforge_serve`** contract stable.

## Relationship to [`agent-master-plan.md`](agent-master-plan.md)

- **Tiers A–D** = ops, shippable HTTP, optional NL/headed — **supporting** work.
- **§ On the rails** = **mission** work — do not treat the kernel checklist as “ship.”
