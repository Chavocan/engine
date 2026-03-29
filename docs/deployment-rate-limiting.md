# Deployment: HTTP rate limiting

## Recommended production path

For public or multi-tenant deployments, **terminate TLS and enforce rate limits at a reverse proxy** or edge (nginx, Caddy, Traefik, Cloudflare, AWS ALB, etc.). Benefits:

- Stable client IP from `X-Forwarded-For` / proxy protocol (when configured correctly).
- No extra CPU work inside the Rust process for every request.
- Centralized WAF, TLS, and bot management.

Example (nginx) — adjust zones and burst to your SLA:

```nginx
limit_req_zone $binary_remote_addr zone=aetherforge:10m rate=10r/s;

server {
    location /v1/ {
        limit_req zone=aetherforge burst=20 nodelay;
        proxy_pass http://127.0.0.1:8787;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

## In-process limit (optional)

The workspace can be built with **`--features rate-limit`** on **`aetherforge_cli`** / **`aetherforge_control`**. When **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`** is set to a positive integer, **`aetherforge_serve`** applies a per–source-IP token bucket (**`governor`**) before your routes. On exceed, clients receive **429** with `error.code` **`HTTP_RATE_LIMIT`**.

- **Env:** `AETHERFORGE_HTTP_RATE_LIMIT_RPS` — sustained requests per second per IP (GCRA; see `governor` for burst behavior).
- **Build:** `cargo run -p aetherforge_cli --features rate-limit --bin aetherforge_serve`
- **Tests:** `cargo test -p aetherforge_control --features rate-limit --test http_rate_limit`

This path is useful for **local demos** or **single-node** installs without a proxy. It uses the **direct TCP peer address** from Axum `ConnectInfo`; behind a reverse proxy without PROXY protocol, you may see the proxy’s IP only—prefer edge limiting in that layout.

## Related

- Session action quota (per session, not per IP): `AETHERFORGE_MAX_ACTIONS_PER_SESSION` — [docs/phase7a-server-hardening.md](phase7a-server-hardening.md).
- Implementation: [crates/aetherforge_control/src/rate_limit.rs](../crates/aetherforge_control/src/rate_limit.rs).
