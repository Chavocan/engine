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
