# ADR 0003 — TLS, bind address, and authentication (deployment)

**Status:** Accepted  
**Date:** 2026-03-29

## Context

The control plane ships as **plain HTTP** suitable for **localhost** development. Production and shared networks require **TLS**, often **non-loopback bind**, and usually **authentication** or network isolation.

## Decision

1. **TLS termination** — Prefer a **reverse proxy** or edge (nginx, Caddy, Traefik, Cloudflare, cloud load balancers) in front of `aetherforge_serve`. The Rust binary does not embed TLS in the default product path for v0.x.
2. **Bind address** — Operators set **`AETHERFORGE_HTTP_ADDR`** (default **`127.0.0.1:8787`**) when binding must be explicit. Local-only deployments should keep **`127.0.0.1`**; LAN or container binds use the appropriate interface IP.
3. **Authentication** — No in-process API key or JWT layer in this ADR. **Defer** to proxy auth, mTLS, VPN, or private networks until requirements are specified; document chosen approach per deployment.
4. **Rate limiting** — In-process optional **`rate-limit`** feature remains a fallback; production still prefers edge limiting ([`docs/deployment-rate-limiting.md`](../deployment-rate-limiting.md)).

## Consequences

- Integration tests for TLS/auth inside the Rust crate are **out of scope** until a concrete in-process or test-harness requirement exists; [`docs/phase1d-verification.md`](../phase1d-verification.md) remains accurate.
- Operators have a **single ADR** to cite for “why HTTP on loopback by default.”

## Related

- [`docs/deployment.md`](../deployment.md) — operator index  
- [`docs/phase7a-server-hardening.md`](../phase7a-server-hardening.md) — quotas and env vars
