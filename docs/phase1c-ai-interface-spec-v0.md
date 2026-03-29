# Phase 1c — AI interface spec v0 (local control plane)

**Status:** v0 — local development only. **Base URL:** `http://127.0.0.1:8787` (configurable via `AETHERFORGE_CONTROL_BIND`). **API prefix:** `/v1`.

## Versioning

- Paths are prefixed with **`/v1`**. Breaking wire changes require **`/v2`** (new schema folder `schemas/v2/`).
- Every JSON body that carries game or control payload includes **`schema_version`** (string, e.g. `"1.0.0"`) per envelope rules in `schemas/v1/control_envelope.schema.json` (**placeholder** until schema files land).

## Auth (local dev)

- **Recommendation:** No auth for v0; bind **`127.0.0.1` only** by default. Document that any LAN exposure requires a future auth slice.
- **UNTESTED:** TLS, tokens, API keys.

## Session ownership & concurrency (rule of record)

- **One mutating runner per `session_id`:** At most one task may call `Simulation::step`, `apply_intent`, or other mutating APIs for a given session at a time.
- **Axum model (v0 target):** One long-lived **`SessionHandle`** (owning or `Arc<Mutex<Simulation>>`) per session, stored in a **`SessionRegistry`**. HTTP handlers **do not** spawn a new exclusive runner per request for mutations; they **await** the same session mutex / actor mailbox. Read-only **`GET observation`** may use shared lock if the sim exposes consistent snapshots (**UNTESTED** until implemented).
- **Session id:** Opaque string, `^[a-zA-Z0-9_-]{8,64}$` (v0); **created** by `POST /v1/sessions`, **destroyed** by `DELETE /v1/sessions/{id}`. Ownership of the `Simulation` instance transfers only through this registry (**UNTESTED** until HTTP impl exists).

## Minimal endpoint set (v0)

| Method | Path | Purpose |
|--------|------|---------|
| `POST` | `/v1/sessions` | **Create** session; **JSON body** `{}` or `{ "seed": <u64> }` (omit `seed` for random). Returns `{ "session_id", "schema_version", "seed" }` |
| `DELETE` | `/v1/sessions/{id}` | **Destroy** session and release runner / locks (`204` body empty) |
| `POST` | `/v1/sessions/{id}/action` | Submit player/agent **Action** (see `schemas/v1/action.schema.json`); advances sim per server policy (e.g. apply + one tick) |
| `POST` | `/v1/sessions/{id}/actions` | Submit **batch** of Actions: body `{ "actions": [ Action, ... ] }`, **1–32** items; same per-item semantics as repeated `.../action` in order (see **`docs/phase4-batch.md`**) |
| `GET` | `/v1/sessions/{id}/observation` | Latest **Observation** snapshot (no mutation) |
| `GET` | `/v1/sessions/{id}/observe/stream` | **SSE** stream of observation JSON when built with Rust feature **`sse-obs`** (see **`docs/phase9b-sse.md`**) |

**Note:** v0 allows merging action+tick into a single POST; **GET observation** remains for pull-based agents. Alternative: `POST .../step` — **deferred** unless Director extends spec.

## Error JSON shape

```json
{
  "error": {
    "code": "SESSION_NOT_FOUND",
    "message": "human readable",
    "request_id": "uuid-or-trace-id"
  },
  "schema_version": "1.0.0"
}
```

**Codes (v0 inventory):** `SESSION_NOT_FOUND`, `INVALID_ACTION`, `INVALID_BATCH` (empty `actions` array), `BATCH_TOO_LARGE` (>32 actions), `SESSION_ACTION_QUOTA` (per-session intent cap — **429**), `INVALID_JSON`, `CONFLICT` (mutation in progress), `INTERNAL`.

## Schema file links (placeholders)

| Concept | Schema file | Rust module (planned) |
|---------|-------------|------------------------|
| Action | `schemas/v1/action.schema.json` | `aetherforge_schemas::v1::Action` |
| Observation | `schemas/v1/observation.schema.json` | `aetherforge_schemas::v1::Observation` |
| World / state snapshot (optional v0) | `schemas/v1/state.schema.json` | `aetherforge_schemas::v1::State` |
| Control envelope | `schemas/v1/control_envelope.schema.json` | `aetherforge_schemas::v1::ControlEnvelope` |

**Field inventory (placeholder):**

- **Action:** `schema_version`, `kind` (enum string), `payload` (object, game-specific).
- **Observation:** `schema_version`, `tick` (integer), `run_id`, `fields` (object).
- **State:** `schema_version`, `tick`, `entities` (array, TBD).

## Logging

- Control plane emits **`tracing`** events compatible with Phase 1b; request ids correlate with errors.

---

## Verification status (Phase 1d)

In-memory **`POST/DELETE /v1/sessions`**, **`POST .../action`**, **`GET .../observation`** are covered by integration tests — see **`docs/phase1d-verification.md`** for test names and paths.

**Still UNTESTED:** WebSocket streaming, TLS/auth, schema CI generation, concurrent mutation stress, production session TTL, `tracing` JSON subscriber wiring.

---

**UNTESTED (rolled forward):** WebSocket streaming, backpressure, session TTL, TLS, auth — see Phase 1d doc for detail. **Batch actions** are implemented (Phase 4a); see **`docs/phase4-batch.md`** and Phase 1d table.
