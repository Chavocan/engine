# Phase 2c — Play log (JSON lines)

## Enable

Set **`AETHERFORGE_PLAY_LOG=1`** before running:

- `cargo run -p aetherforge_cli --bin aetherforge_serve`
- `cargo run -p aetherforge_cli --bin aetherforge_headless` (play lines mix with headless stdout observation — tail only lines with `"target":"aetherforge.play"` or filter on `event` key if present in your JSON formatter)

When unset, the process uses normal human-readable `tracing` formatting (no JSON play lines).

## Line contract (flattened JSON)

Each play event includes at minimum:

| Field | Meaning |
|-------|---------|
| `ts` | RFC3339 millis timestamp |
| `run_id` | Simulation run id (matches `Observation.run_id`) |
| `tick` | Logical tick after the event’s context |
| `event` | `session_created`, `session_deleted`, `action_applied`, `tick_advanced`, `observation_served`, `farm_day_advanced`, `farm_harvested` (latter two when **`farm-stub`** + intent kind matches) |
| `session_id` | HTTP session id (empty string if N/A) |
| `payload` | String (JSON text), truncated past **2048** bytes with `…(truncated)` |

`observation_served` payloads carry only `{ "schema_version": "..." }` — not full observations (default).

## Tailing for an external AI

1. Run `aetherforge_serve` with `AETHERFORGE_PLAY_LOG=1`.
2. Stream stdout to a file or pipe: each **line** is one JSON object for `aetherforge.play` events (other human `tracing` lines may interleave unless you filter — in play-log mode, non-play events use the plain-text layer).
3. Parse with `json.loads` (Python) or `jq` per line; filter `event == "observation_served"` for cheap state checkpoints.

## UNTESTED

- Log rotation / file append via env (not implemented — stdout only in 2c).
- WebSocket subscribe (explicitly out of scope for 2c).
