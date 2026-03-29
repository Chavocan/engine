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
