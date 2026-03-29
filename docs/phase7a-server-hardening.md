# Phase 7a — Server hardening (session action cap)

## Listen address (`aetherforge_serve`)

- **Env:** **`AETHERFORGE_HTTP_ADDR`** — default **`127.0.0.1:8787`**. Parseable `host:port` (e.g. `0.0.0.0:8787` for all interfaces in containers).
- **Production:** Prefer **TLS and auth at a reverse proxy**; see **`docs/deployment.md`** and **ADR [`docs/adr/0003-deployment-tls-and-auth.md`](adr/0003-deployment-tls-and-auth.md)**.

## Per-session action quota

- **Env:** `AETHERFORGE_MAX_ACTIONS_PER_SESSION` — positive integer; default **`10000`**.
- **Semantics:** Counts every applied intent: each **`POST .../action`** adds **1**; each **`POST .../actions`** adds **`actions.len()`** (only if the whole batch is accepted).
- **When exceeded:** **`429 Too Many Requests`** with stable error JSON:
  - `error.code`: **`SESSION_ACTION_QUOTA`**
- **Batch behavior:** If `actions_applied + len(actions)` would exceed the cap, the request is **rejected** with **429** and **no** intents from that batch are applied (no partial advance).

## Test / embed API

- **`app_router_with_config(ControlConfig { max_actions_per_session })`** — use in tests with a low limit without touching process env.

## Per-IP rate limiting

**Implemented (optional):** Build with **`rate-limit`** and set **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`** for in-process per-IP limits (**429** `HTTP_RATE_LIMIT`). **Recommended production** approach: rate-limit at a **reverse proxy or edge** — see **`docs/deployment-rate-limiting.md`**.

- Tests: `cargo test -p aetherforge_control --features rate-limit --test http_rate_limit`

## Related

- Error inventory: **`docs/phase1c-ai-interface-spec-v0.md`**
- Tests: **`crates/aetherforge_control/tests/http_action_quota.rs`**

---

## Documentation & Learning Log (Employee AI)

- **429 vs 403:** Quota is a **backpressure** signal for agents; message points to new session or raising the env var.
- **Session entry:** Merged `Simulation` + `actions_applied` under one mutex to keep counting consistent with application order.
- **Follow-up (optional):** Add CI step that inspects **`cargo tree -p aetherforge_player -e normal`** (expect **no** `aetherforge_sim` on **runtime** edges). Do **not** use bare **`-i aetherforge_sim`** without `-e normal` — dev-deps pull `aetherforge_control`, which depends on sim for tests. Keep script + Rust guard as source-level defense in depth.
