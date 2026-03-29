# Phase 7a — Server hardening (session action cap)

## Per-session action quota

- **Env:** `AETHERFORGE_MAX_ACTIONS_PER_SESSION` — positive integer; default **`10000`**.
- **Semantics:** Counts every applied intent: each **`POST .../action`** adds **1**; each **`POST .../actions`** adds **`actions.len()`** (only if the whole batch is accepted).
- **When exceeded:** **`429 Too Many Requests`** with stable error JSON:
  - `error.code`: **`SESSION_ACTION_QUOTA`**
- **Batch behavior:** If `actions_applied + len(actions)` would exceed the cap, the request is **rejected** with **429** and **no** intents from that batch are applied (no partial advance).

## Test / embed API

- **`app_router_with_config(ControlConfig { max_actions_per_session })`** — use in tests with a low limit without touching process env.

## Per-IP rate limiting

**UNTESTED / not implemented:** Optional token-bucket per client IP behind a **`rate-limit`** feature was deferred; revisit if abusive traffic appears before a reverse proxy is added.

## Related

- Error inventory: **`docs/phase1c-ai-interface-spec-v0.md`**
- Tests: **`crates/aetherforge_control/tests/http_action_quota.rs`**

---

## Documentation & Learning Log (Employee AI)

- **429 vs 403:** Quota is a **backpressure** signal for agents; message points to new session or raising the env var.
- **Session entry:** Merged `Simulation` + `actions_applied` under one mutex to keep counting consistent with application order.
- **Follow-up:** Replace `player_no_sim_guard` substring check with a **`cargo metadata`** dependency assertion for `aetherforge_player` (Director note).
