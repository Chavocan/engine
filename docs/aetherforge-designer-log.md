# Documentation & Learning Log (Employee AI)

Append-only implementation notes. Mirror bullet labels from `aetherforge-designer-ai.agent.md`.

---

## 2026-03-29 (platform P4 — keyboard → Intent)

- **Accomplished this cycle:** **`aetherforge_window`** maps **P**/1, **D**/2, **H**/3, **Space** to **`farm_plant`** / **`farm_advance_day`** / **`farm_harvest`** / **`noop`**; default **input-driven**; **`AETHERFORGE_WINDOW_AUTO_DEMO=1`** restores auto loop; **`platform-headed-roadmap`** P4 done; roadmap R2 **v4**; **`getting-started`**, **`phase1d`**, **`CHANGELOG`**, **`director-program-roadmap`**, **`lib.rs`**.
- **Processes used:** `cargo build -p aetherforge_platform --features windowed`; `cargo clippy`.
- **Pitfalls / observations:** Intent kinds match HTTP **`action.kind`** strings for parity.
- **Learnings / best practices:** One **`refresh_title`** after each frame keeps HUD consistent in both modes.
- **Next cycle action items:** In-window text overlay or Godot shell per product.

## 2026-03-29 (platform P3 — sim in headed window)

- **Accomplished this cycle:** **`aetherforge_window`** drives **`Simulation`** each frame (farm-stub intent loop); **`set_title`** observation HUD; tick-based clear color; **`windowed`** feature enables **`aetherforge_sim/farm-stub`**; **`AETHERFORGE_WINDOW_SEED`**; docs (**`platform-headed-roadmap`** P3, **`getting-started`**, **`phase1d`**, roadmap R2 v3, **`CHANGELOG`**, **`lib.rs`**).
- **Processes used:** `cargo build -p aetherforge_platform --features windowed`; `cargo clippy -p aetherforge_platform --features windowed`.
- **Pitfalls / observations:** Continuous **`request_redraw`** loop; removed **`about_to_wait`** redraw to avoid redundant wakeups.
- **Learnings / best practices:** Title bar is a zero-dep “HUD” until in-window text (P4+).
- **Next cycle action items:** **P4** input mapping or **R5** NL.

## 2026-03-29 (headed window — `aetherforge_window`)

- **Accomplished this cycle:** Cargo feature **`windowed`**, binary **`aetherforge_window`** (`winit` + `wgpu` clear color); CI **`headed-smoke windowed`** compile; **`platform-headed-roadmap`** P1–P2 done; roadmap R2 **v2**; **`getting-started`**, **`CONTRIBUTING`**, **`phase1d-verification`**, **`director-program-roadmap`**, **`CHANGELOG`**, **`lib.rs`** placeholder text.
- **Processes used:** `cargo build -p aetherforge_platform --features windowed`; `cargo clippy` on platform.
- **Pitfalls / observations:** CI does not **run** the window binary (no GPU/display on typical runners).
- **Learnings / best practices:** Optional **`AETHERFORGE_WINDOW_MAX_SEC`** avoids stuck automation on dev machines.
- **Next cycle action items:** **Platform P3** sim hook or **R5** NL per product.

## 2026-03-29 (R2 terminal HUD + R3 ADR 0004)

- **Accomplished this cycle:** **`python/aetherforge_sdk/examples/observation_hud.py`**; **`docs/adr/0004-runtime-embedding.md`**; roadmap **R2** / **R3** rows **done (v1)**; **`platform-headed-roadmap`**, **`director-program-roadmap`**, **`phase1d-verification`**, SDK **`README`**, **`CHANGELOG`**.
- **Processes used:** `ruff check` on new example.
- **Pitfalls / observations:** Headed **winit** track unchanged — R2 v1 is intentionally **thin** (HTTP + formatted text).
- **Learnings / best practices:** Same **`Observation`** model as integration tests keeps HUD honest vs schema.
- **Next cycle action items:** **platform-headed** P1 window, or **R5** NL depth if product pulls it.

## 2026-03-29 (R1 flagship + R4 HTTP mission E2E)

- **Accomplished this cycle:** **`flagship_farm_two_cycles.json`**, **`flagship_farm_http.json`**, **`docs/flagship-scenario.md`**, **`scenario_flagship_offline`**, CI **`farm-mission-http-e2e`**; roadmap **R1** / **R4** rows **done (v1)**; **`director-program-roadmap`** phases 4–5 / 7 adjusted; **`phase1d-verification`**, **`CONTRIBUTING`**, **`CHANGELOG`**.
- **Processes used:** `cargo test -p aetherforge_cli --features farm-stub --test scenario_flagship_offline`; schema drift script unchanged.
- **Pitfalls / observations:** R4 v1 is **scenario harness** + HTTP assert, not a free-form agent loop — phase 7 stays **partial** until that exists.
- **Learnings / best practices:** Same JSON shape offline and HTTP (**`base_url`** only) keeps CI and local parity obvious.
- **Next cycle action items:** **R2** client surface or **R3** runtime ADR per product priority.

## 2026-03-29 (R0 — mission outcome + scenario assert)

- **Accomplished this cycle:** **`MissionOutcome`**, **`Observation.mission`**, schema **1.2.0**, **`expect_mission_outcome`** in **`aetherforge_cli::scenario`**, **`farm_demo_loop.json`**, **`scenario_mission_offline`** test, CI step, **`observation_mission_property.json`** drift, Python **`MissionSnapshot`**, roadmap **R0** row **done (v1)**.
- **Processes used:** `cargo test --workspace`, `python scripts/check_schema_drift.py`, pytest.
- **Pitfalls / observations:** **`lost`** path not yet used by sim — enum reserved for R0 extensions.
- **Learnings / best practices:** Game vertical **proof** is **`mission` + scenario assert**, not tick-only.
- **Next cycle action items:** **R1** flagship depth or **R4** autonomous job consuming **`won`**.

## 2026-03-29 (rails — roadmap vs kernel)

- **Accomplished this cycle:** Rewrote **`roadmap-to-complete-project.md`**: **North star**, **Kernel v0.1 (closed)**, **§ On the rails** (R0–R5: win condition, flagship depth, client surface, runtime ADR, autonomous playthrough proof, NL). Updated **`agent-master-plan`**, **`director-program-roadmap`**, **`README`**, **`.cursor/rules/aetherforge-agent-ownership.mdc`**, **`CHANGELOG`**, **`AGENTS`**.
- **Processes used:** Editorial alignment only.
- **Pitfalls / observations:** Prior “100% complete” referred only to the **kernel** table — now explicit so agents do not optimize the wrong finish line.
- **Learnings / best practices:** Single file (**`roadmap-to-complete-project.md`**) holds both **closed** and **open** so scope cannot drift silently.
- **Next cycle action items:** Implement **R0** (game outcome + asserted tests) as first engineering slice.

## 2026-03-29 (usable engine — first-run path)

- **Accomplished this cycle:** **`docs/getting-started.md`** (clone → test → headless → farm scenario → HTTP curl → Python SDK); **`scripts/dev_smoke.sh`** / **`dev_smoke.ps1`**; **`README`**, **`CONTRIBUTING`**, **`AGENTS`** links; **`CHANGELOG`** Unreleased.
- **Processes used:** `powershell -File scripts/dev_smoke.ps1` (green).
- **Pitfalls / observations:** Full **`cargo test`** remains the deep gate; smoke is for fast feedback only.
- **Learnings / best practices:** “Usable” for this repo means **documented linear path + scripts**, not new engine features.
- **Next cycle action items:** Optional CI job running **`dev_smoke.sh`** only if we need redundancy vs golden script.

## 2026-03-29 (finish lines 1–4 — scoped plan implementation)

- **Accomplished this cycle:** **`AETHERFORGE_HTTP_ADDR`**; **ADR 0003** (TLS/auth at proxy); **`docs/deployment.md`**, **`release-closure-checklist.md`**, **`director-program-roadmap.md`**; **`agent-master-plan.md`** finish-line table; schema drift **`world`** fragment; **`nl-interpret-stub`** + CI; **`http_concurrent`** 48-fan-out test; pushed **`main`** and **`v0.1.0`** to **`origin`**.
- **Processes used:** `cargo test --workspace`, `cargo clippy --all-features`, `python scripts/check_schema_drift.py`.
- **Pitfalls / observations:** NL-2 remains sidecar-first; in-process route is **501** placeholder only.
- **Learnings / best practices:** Finish lines are **documentation + incremental code**, not a single “done” bit for the Director program (see **`director-program-roadmap.md`**).
- **Next cycle action items:** Optional in-process TLS/auth only if product mandates; headed **P1** window when platform staffing returns.

## 2026-03-29 (v0.1.0 completion pass)

- **Accomplished this cycle:** Closed roadmap items: **CHANGELOG** **\[0.1.0\]**, git tag **`v0.1.0`**, **`scripts/golden_playthrough.sh`** + CI, **`scripts/check_action_schema_drift.py`** + schemars **`schema-export`**, SSE caps (**`SSE_SESSION_CAP`** / **`SSE_GLOBAL_CAP`**), **`AETHERFORGE_PLAY_LOG_STDOUT`**, **`headed-smoke`** / **`aetherforge_wgpu_smoke`**, docs (**`roadmap-to-complete-project.md`**, **`release-v0.1.0.md`**, **`demo-showcase.md`**, **`backlog-post-v0.1.md`**, **`phase1d`**, **`phase2c`**, **`CONTRIBUTING.md`**).
- **Processes used:** `cargo test`, `cargo clippy -- -D warnings`, pytest, relay **`chat_a`** handoffs.
- **Pitfalls / observations:** **`Action`** now uses **`deny_unknown_fields`** — matches **`action.schema.json`** `additionalProperties: false`.
- **Learnings / best practices:** Golden script duplicates subset of **`cargo test`** but documents the “demo bundle” explicitly for stakeholders.
- **Next cycle action items:** Per-IP **`rate-limit`** feature (still deferred); extend schema drift beyond **Action** if more types gain **`JsonSchema`**.

## 2026-03-29

- **Accomplished this cycle:** (Planned) Execution pass to follow Director roadmap — institutional logs created; repo roadmap file added; verification runs delegated to same session.
- **Processes used:** Relay + direct commits; tests as source of truth.
- **Pitfalls / observations:** Designer log was missing prior cycles; backfill only from phase docs where needed.
- **Learnings / best practices:** Keep verification table rows linked to test names whenever adding endpoints or observation fields.
- **Next cycle action items:** Farm harvest intent + observation fields; `docs/demo-showcase.md`; CI job for golden scenario.

## 2026-03-29 (Agent mode — play_log)

- **Accomplished this cycle:** Added **`play_log_farm_harvested`** in `aetherforge_control::server` (`farm-stub` only), called from **`post_action`** and **`post_actions_batch`** when intent kind is **`farm_harvest`**; updated **`docs/phase2c-play-log.md`** and **`CHANGELOG.md`**.
- **Processes used:** Shared helper to avoid duplicate emit logic; qualified `aetherforge_sim::Observation` to skip extra imports.
- **Pitfalls / observations:** This Cursor shell still has **no `cargo`** on PATH — run **`cargo test -p aetherforge_control --features farm-stub`** locally.
- **Learnings / best practices:** `harvested_item_total` sums `inventory` keys prefixed with **`harvested_`** for a compact AI-facing signal.
- **Next cycle action items:** Optional integration test capturing play-log lines (low priority).

## 2026-03-29 (P3a verification — relay B)

- **Accomplished this cycle:** (Director) A landed **P3a** files on disk; **B** to verify license headers, run **`cargo test`** / Python checks, and append results here after relay handoff.
- **Processes used:** Follow Director **`chat_a`** P3a message.
- **Pitfalls / observations:** None until toolchain run.
- **Learnings / best practices:** Dual license files match **`workspace.package.license`** in `Cargo.toml`.
- **Next cycle action items:** After merge — **`cargo test`** on maintainer machine; optional **NOTICE** file for Apache attribution if third-party crates require it.

## 2026-03-29 (B — P3a relay + toolchain hygiene)

- **Accomplished this cycle:** Received Director **P3a** (A owns `LICENSE-MIT`, `LICENSE-APACHE`, `SECURITY.md`, `docs/nl-agentic-hooks.md`). **Spot-check:** those paths were **not present** in this workspace yet — B legal review **deferred** until A lands. Ran **`cargo test`** (pass). **`python/aetherforge_sdk/pyproject.toml`:** moved **`classifiers`** and **`dependencies`** out from under **`[project.urls]`** (invalid metadata) so **`pip install -e ".[dev]"`** matches **`CONTRIBUTING.md`** / CI; then **`ruff check`**, **`check_observation_contract.py`**, **`pytest`** (pass).
- **Processes used:** Disk preflight before legal copy-edit; minimal packaging fix to unblock Python dev install.
- **Pitfalls / observations:** TOML structure under `[project.urls]` is easy to corrupt; validate with editable install locally.
- **Learnings / best practices:** B can land hygiene fixes while waiting on A’s license slice.
- **Next cycle action items:** After P3a files appear — re-read licenses + `SECURITY.md` for typos/holder line (**Chavocan and contributors** per Director); P3b README/REUSE only if stakeholder asks.

## 2026-03-29 (B — P3a spot-check + ADR 0002)

- **Accomplished this cycle:** Synced workspace now contains **`LICENSE-MIT`**, **`LICENSE-APACHE`**, **`SECURITY.md`**, **`docs/nl-agentic-hooks.md`**. **Spot-check:** MIT/Apache standard texts; copyright **Chavocan and AetherForge contributors** (MIT + Apache appendix); **`SECURITY.md`** private advisory path + scope notes OK; NL doc correctly design-only with sidecar preference. Drafted **`docs/adr/0002-player-crate-split.md`** (crate name **`aetherforge_player`**, deps graph, script + doc migration, checklist).
- **Processes used:** Read-only legal pass; ADR format matched **`0001`**.
- **Pitfalls / observations:** Early Director note “Chavocan and contributors” vs shipped “Chavocan and AetherForge contributors” — both coherent; no change requested.
- **Learnings / best practices:** ADR before move reduces debate in the implementation PR.
- **Next cycle action items:** Implement ADR 0002 in a dedicated PR; optional P3b README license section.

## 2026-03-29 (B — ADR 0002 landed, verify)

- **Accomplished this cycle:** **PASS.** **`cargo test`** (full workspace) and **`cargo test -p aetherforge_player`** — all green. **`cargo tree -p aetherforge_player -e normal`** — **no `aetherforge_sim`** on **runtime** edges. **`cargo tree -p aetherforge_player -i aetherforge_sim`** prints a path **only via `dev-dependencies`** (`aetherforge_control` for integration tests) — expected; binary crate does not list sim. **`scripts/check_player_no_sim_import.sh`:** not runnable on this Windows shell (**bash** missing); **equivalent grep** on `crates/aetherforge_player/src` — **no** `use aetherforge_sim` matches. **CI / Ubuntu** remains authority for the shell script.
- **Processes used:** Distinguish **normal** vs **full** `cargo tree` when interpreting “no sim.”
- **Pitfalls / observations:** Director checklist phrase “expect no sim” should mean **non-dev** dependency graph for the player **library/binary** artifact.
- **Learnings / best practices:** Dev-only control dep pulls sim transitively for `cargo test` builds; still no sim in **player `src/`**.
- **Next cycle action items:** Stakeholder **push + green Actions**; optional CI step `cargo tree -p aetherforge_player -e normal |` … guard; **`v0.1.0`** per release doc.

## 2026-03-29 (B — CI `cargo tree` guard for player)

- **Accomplished this cycle:** Added **`.github/workflows/ci.yml`** step: fail if **`cargo tree -p aetherforge_player -e normal`** output contains **`aetherforge_sim`**; updated **`CONTRIBUTING.md`** CI table + **`CHANGELOG.md`**. Local PowerShell check — pass.
- **Processes used:** Encodes B’s verify nuance as regression (runtime edges only).
- **Pitfalls / observations:** `grep` substring is sufficient; crate name is unique in workspace.
- **Learnings / best practices:** CI mirrors documented intent without banning dev-dep sim for tests.
- **Next cycle action items:** Watch Actions on merge.
