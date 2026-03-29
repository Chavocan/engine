# Phase 4a — Batch actions (`POST .../actions`)

## Endpoint

- **Method / path:** `POST /v1/sessions/{id}/actions`
- **Request body:** `{ "actions": [ Action, ... ] }` where each **Action** matches `schemas/v1/action.schema.json` (same envelope as single `POST .../action`).

## Limits

- **Minimum:** one action (`actions` must be non-empty). Empty array → **400** with `error.code` **`INVALID_BATCH`**.
- **Maximum:** **32** actions per request. More → **413 Payload Too Large** with `error.code` **`BATCH_TOO_LARGE`**.

## Semantics (determinism)

For each action **in array order**, the server performs the same work as one **`POST .../action`**:

1. `ai_driver_enqueue_intent` with that action’s `kind` (payload is accepted on the wire for schema parity; the current sim hook uses `kind` only).
2. `Simulation::step()` once.

The handler holds the **session mutex** for the **entire batch** (no interleaving with other mutating requests on that session).

**Observation:** `GET .../observation` is unchanged. After a successful batch response, `tick` and the observation body reflect the **final** state after all steps (same as applying the same actions as **N** sequential single posts).

## Success response

```json
{
  "ok": true,
  "tick": <u64>,
  "applied": <N>,
  "schema_version": "1.0.0"
}
```

## Play log

When `AETHERFORGE_PLAY_LOG=1`, one event **`batch_actions_applied`** is emitted after the batch completes, with payload including `count`, `kinds`, and `final_tick`. Single-action posts still emit **`action_applied`** / **`tick_advanced`** as before.
