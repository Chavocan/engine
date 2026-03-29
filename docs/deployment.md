# Deployment — operator index

Single entry point for running **AetherForge** `aetherforge_serve` beyond ad-hoc development.

## Listen address

| Env | Default | Notes |
|-----|---------|--------|
| **`AETHERFORGE_HTTP_ADDR`** | `127.0.0.1:8787` | Set to `0.0.0.0:8787` (or a specific IP) only when you intend to accept remote connections; combine with firewall / proxy. |

## TLS and authentication

**Do not** expose plain HTTP to the public internet. Terminate **TLS** at a reverse proxy or edge. **Authentication** is not built into the core server in v0.x; use the proxy, network policy, or a future ADR if in-process auth is required.

See **[ADR 0003 — deployment TLS and auth](adr/0003-deployment-tls-and-auth.md)**.

## Rate limiting and quotas

- **Per-IP (optional Rust):** [`docs/deployment-rate-limiting.md`](deployment-rate-limiting.md) — feature **`rate-limit`**, env **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`**.
- **Per-session action cap:** [`docs/phase7a-server-hardening.md`](phase7a-server-hardening.md) — **`AETHERFORGE_MAX_ACTIONS_PER_SESSION`**.

## Natural language (NL-2)

Optional stub route **`POST /v1/sessions/{id}/interpret`** returns **501** when built with **`nl-interpret-stub`** — placeholder until a real interpreter exists. Preferred path remains a **sidecar** calling the Python SDK ([`docs/nl-agentic-hooks.md`](nl-agentic-hooks.md)).

## CI parity

What GitHub Actions runs is documented in [`CONTRIBUTING.md`](../CONTRIBUTING.md) (`.github/workflows/ci.yml`).
