# ADR 0004 — Runtime embedding and client stack (Godot vs alternatives)

**Status:** Accepted  
**Date:** 2026-03-29

## Context

The workspace ships a **Rust** simulation kernel, **`aetherforge_serve`** as the control plane, and optional **`aetherforge_platform`** (wgpu smoke plus a minimal **`aetherforge_window`** — winit + wgpu clear). Product direction in [`aetherforge-lead-director.agent.md`](../aetherforge-lead-director.agent.md) prefers **Godot 4.x** for a full game client, but the repo does not yet embed a Godot project. We need a **written decision** so agents and forks do not guess per PR.

## Decision

1. **Canonical agent and tooling interface** — **`aetherforge_serve`** over **HTTP** (`/v1/sessions`, actions, observation) remains the **stable contract** for Python SDK, scenario runner, CI, and external agents. This does not move in favor of an in-process FFI in v0.x unless a future ADR supersedes this.

2. **Primary embedding for a playable “full game” client** — **Godot 4.x** is the **default recommended** shell: open source, GDScript/C#, strong tooling, matches the Lead Director baseline. Integration path **today**: HTTP client in Godot to **`aetherforge_serve`** (same JSON as SDK). A **native bridge** (GDExtension, shared library, or in-proc sim) is **optional future work** if latency or packaging demands it — not required to claim progress on R2/R3 in the engine repo.

3. **Unity** — **Acceptable alternative** when the team or asset pipeline requires it. Same control-plane contract; tradeoffs include licensing, build farm complexity, and heavier CI vs Godot for open pipelines.

4. **Web-first / thin client** — Valid for dashboards, tools, and lightweight UIs: browser **`fetch`** (or SSE where enabled) against **`aetherforge_serve`**. Same schemas; no special status vs Godot for “official” client — choose by product.

5. **In-repo Rust headed stack** — **`aetherforge_platform`** ships **`aetherforge_window`** (feature **`windowed`**) with in-process **`aetherforge_sim`** and a **title-bar** observation HUD; further phases (input → intent) follow [`platform-headed-roadmap.md`](../platform-headed-roadmap.md). This stack is **engine workspace tooling and experimentation**, not a commitment to ship the flagship farming demo only in Rust+winit.

6. **Simulation placement** — **Headless / scripted / CI** use **`aetherforge_sim`** in-process (`aetherforge_headless`, **`aetherforge_scenario --offline`**) or remote HTTP. **Embeddable game runtimes** (Godot, Unity, web) should assume **observation JSON parity** with the HTTP API whether the sim runs in another process or, later, behind an optional embedded library.

## Consequences

- This repo may **not** add a Godot project immediately; the ADR still **commits tradeoffs** for downstream work and reviews.
- **R3** (“runtime / embedding decision”) is satisfied by this ADR plus the existing serve-first architecture.
- Contradiction with “everything in Rust” is avoided: **Rust owns sim + control plane**; **Godot (or other) owns presentation** unless the product explicitly builds a Rust-only UI in **`aetherforge_platform`**.

## Related

- [`docs/roadmap-to-complete-project.md`](../roadmap-to-complete-project.md) — **§ On the rails** (R2, R3)  
- [`docs/platform-headed-roadmap.md`](../platform-headed-roadmap.md) — headed Rust phases  
- [`docs/adr/0003-deployment-tls-and-auth.md`](0003-deployment-tls-and-auth.md) — TLS at proxy; `aetherforge_serve` on HTTP
