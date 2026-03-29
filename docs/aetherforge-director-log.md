# Documentation & Learning Log (Lead Director)

Append-only session summaries for the AetherForge program. Mirror bullet labels from `aetherforge-lead-director.agent.md`.

---

## 2026-03-29

- **Accomplished this cycle:** Refreshed Cursor plan to match real repo state; added `docs/roadmap-to-complete-project.md`; defined “complete product + demo” criteria (farm loop MVP, demo doc, CI golden path, v0.1.0 tag, backlog P1–P3, polish).
- **Processes used:** Plan-doc sync; gap analysis vs `docs/backlog-post-v0.1.md` and phase docs.
- **Pitfalls / observations:** Old plan claimed “no repo”; workspace already contains multi-phase implementation—roadmap must stay tied to files on disk.
- **Learnings / best practices:** Single `roadmap-to-complete-project.md` in repo keeps stakeholders aligned without opening Cursor plans.
- **Next cycle action items:** Run full test matrix locally/CI; implement farm **harvest** loop + `docs/demo-showcase.md`; wire optional CI playthrough job.

## 2026-03-29 (follow-up)

- **Accomplished this cycle:** Added `docs/demo-showcase.md` (5-minute script + embedded `farm_demo_loop` JSON); linked roadmap, logs, and demo from `README.md`; refreshed Cursor plan todos (institutional logs + demo doc marked **completed**).
- **Processes used:** Plan iteration in [aetherforge_director_program plan](C:\Users\AI\.cursor\plans\aetherforge_director_program_8c088d2a.plan.md); markdown-only edits under **plan mode** constraint.
- **Pitfalls / observations:** **Plan mode** blocked edits to `.rs` / `.json` in this session—`farm_harvest` sim logic and `examples/farm_demo_loop.json` file on disk still need an **Agent mode** (or plan-off) pass. This environment’s shell had **no `cargo` on PATH**—verify tests on your machine or GitHub Actions.
- **Learnings / best practices:** Keep scenario JSON in demo doc as copy-paste until harvest lands so the script is still self-contained.
- **Next cycle action items:** Switch to Agent mode; apply `farm_harvest` patch to `crates/aetherforge_sim/src/lib.rs` + test `farm_demo_loop_plant_grow_harvest`; write `examples/farm_demo_loop.json`; optional `play_log` for `farm_harvested`; run `cargo test -p aetherforge_sim --features farm-stub`.

## 2026-03-29 (harvest landed)

- **Accomplished this cycle:** Reconnected relay **A**; **B** (or parallel agent) shipped **`farm_harvest`**, **`farm_demo_loop_plant_grow_harvest`**, **`examples/farm_demo_loop.json`**, doc/changelog updates; spot-checked `lib.rs` on disk.
- **Processes used:** `chat_a` handoff; workspace verification via read/grep.
- **Pitfalls / observations:** Cursor **Plan mode** blocked A from editing code directly; relay + B unblocked. Shell here still lacks **`cargo`** — run tests locally or on **GitHub Actions**.
- **Learnings / best practices:** Keep Director spec in relay when A is tool-blocked.
- **Next cycle action items:** Optional **`farm_harvested`** play_log; **CI** `demo-loop` job; **`v0.1.0`** checklist.

## 2026-03-29 (CI demo-loop doc)

- **Accomplished this cycle:** Updated **`CONTRIBUTING.md`** (CI table + local parity) to document **`farm_demo_loop` offline** run; **Director chose `demo-loop` CI before optional `farm_harvested` play_log**; relay **`chat_a`** resumed with B.
- **Processes used:** Markdown-only edit (Plan mode still blocks `.yml` on A).
- **Pitfalls / observations:** **`.github/workflows/ci.yml`** must gain the same step on B or after Plan mode off: `cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json` after player script.
- **Learnings / best practices:** Doc-first CI contract when workflow files are blocked.
- **Next cycle action items:** Patch **`ci.yml`**; optional **`farm_harvested`** play_log.

## 2026-03-29 (Agent mode — `farm_harvested` play log)

- **Accomplished this cycle:** With **Agent mode** on, implemented **`play_log_farm_harvested`** in [`crates/aetherforge_control/src/server.rs`](C:\Users\AI\Game engine\crates\aetherforge_control\src\server.rs) for **`farm_harvest`** (single + batch); updated **`phase2c-play-log.md`**, **`phase5-farm-stub.md`**, **`demo-showcase.md`**, **`CHANGELOG.md`**, designer log.
- **Processes used:** Shared helper; `farm-stub` cfg gates.
- **Pitfalls / observations:** This environment still has **no `cargo`** — run **`cargo test -p aetherforge_control --features farm-stub`** locally or rely on **GitHub Actions**.
- **Learnings / best practices:** Agent mode removes Plan-mode blocks on Rust sources.
- **Next cycle action items:** Push and confirm CI green; **`v0.1.0`** checklist when ready.
