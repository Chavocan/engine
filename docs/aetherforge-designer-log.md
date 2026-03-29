# Documentation & Learning Log (Employee AI)

Append-only implementation notes. Mirror bullet labels from `aetherforge-designer-ai.agent.md`.

---

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
