# ADR 0001: Observation stream transport (server → client)

## Status

Accepted — **design only** (Phase 9a). No implementation in this slice.

## Context

- Agents and humans want to **tail** observation updates without polling `GET .../observation` on a tight loop.
- The control plane is **HTTP-first**; **mutations** remain `POST` actions (single or batch).
- Firewall and ops teams often allow **HTTPS**; raw WebSockets can be restricted or proxied differently than SSE.
- Backlog item **P1** required choosing **one** of WebSocket vs SSE before a prototype (Phase 9b).

## Decision

Use **Server-Sent Events (SSE)** for the **server → client observation stream**.

- **One-way** stream matches the dominant need: push JSON observation (or wire-identical bytes) after relevant ticks.
- **Actions and session lifecycle** stay on existing **POST/DELETE** endpoints; no requirement for a duplex socket in v0.x.
- **Reconnect** is a standard HTTP story: client opens a new `EventSource` / `GET` with `Accept: text/event-stream`, optionally with `Last-Event-ID` if we add it later.
- **Axum** can expose SSE via `axum::response::sse` (or equivalent) on a path such as `GET /v1/sessions/{id}/observation/stream` (exact path TBD in 9b).

## Consequences

- **Positive:** Simpler than WebSocket for tail-only use cases; works through many HTTP proxies; aligns with “observation pull + optional push stream.”
- **Negative:** No full-duplex on one connection; if we later need client-sent frames on the same socket, we add a **separate** WebSocket path or revisit this ADR.
- **Backpressure:** SSE consumers must read promptly; slow clients may be dropped (document limits in 9b).

## Rejected alternatives

| Alternative | Reason not chosen (for now) |
|-------------|-----------------------------|
| **Polling only** | High latency and load for agent loops; already a baseline, not a “stream.” |
| **WebSocket** | Stronger when we need **bidirectional** real-time on **one** connection; we do not yet. Revisit if gameplay or tooling requires client messages on the stream. |
| **gRPC streaming** | Heavier operational and client footprint than SSE for local/dev-first v0.x. |

## Actions path unchanged

- **POST** `/v1/sessions/{id}/action` and `/actions` remain the **only** mutation entry points for intents in this design; SSE does not carry action payloads.

---

## Documentation & Learning Log (Employee AI)

- **Path:** `docs/adr/0001-observation-stream-transport.md`
- **Process:** ADR locks transport before Axum route sketch in 9b.
- **Pitfall:** Do not imply SSE replaces `GET` snapshot; keep idempotent **GET observation** for pull-based agents.
- **Next:** Phase **9b** — feature-flagged SSE route + one integration test (Director slice).
