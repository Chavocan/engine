# Phase 9b — SSE observation stream (`sse-obs`)

## Enable

Build **`aetherforge_control`** or **`aetherforge_cli`** with feature **`sse-obs`**:

```bash
cargo build -p aetherforge_cli --features sse-obs --bin aetherforge_serve
```

Default **CI** still builds without this feature; the **SSE integration test** runs in CI via `cargo test -p aetherforge_control --features sse-obs`.

## Endpoint

- **`GET /v1/sessions/{id}/observe/stream`**
- **`Accept: text/event-stream`**
- **Payload:** each event’s `data:` line is the same JSON bytes as **`GET .../observation`** (`wire` / `observation_to_vec`).

## Cadence

- **Immediate** first event on connect (current observation).
- **Subsequent** events only when **`tick`** changes (poll interval **25 ms** inside the handler — implementation detail).
- **Keep-alive** comments every **15 s** (`text/event-stream` standard).

## Proxies

Some reverse proxies buffer SSE; may need `X-Accel-Buffering: no` (nginx) or equivalent — **UNTESTED** here.

## Python SDK (Phase 9c)

- **`AetherForgeClient.observe_stream(session_id)`** — sync iterator of **`Observation`** from SSE `data:` lines (`python/aetherforge_sdk`).
- **Tests:** mock `text/event-stream` body in **`tests/test_client_mock.py`** (no live server).

## Push wake (backlog)

**UNTESTED:** Replace the **25 ms poll** in Rust with a **`watch` channel** (or similar) updated from `step` when `sse-obs` is enabled — see Director Phase 9d note.

## Tests

- `crates/aetherforge_control/tests/sse_observe_stream.rs` — creates session, reads first SSE JSON (`tick == 0`), `POST action`, reads second event (`tick == 1`).

---

## Documentation & Learning Log (Employee AI)

- **Path:** `docs/phase9b-sse.md`; route in `server.rs` behind `#[cfg(feature = "sse-obs")]`.
- **ADR:** `docs/adr/0001-observation-stream-transport.md` (SSE decision).
- **Pitfall:** `reqwest::Response::bytes_stream` consumes the response; tests hold the byte stream across `POST` from a second client.
