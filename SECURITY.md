# Security policy

## Supported versions

| Version / branch | Supported |
|------------------|-----------|
| `main`           | Yes       |
| Older tags       | Best effort only |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for undisclosed security problems.

1. Open a **private security advisory** on [github.com/Chavocan/engine](https://github.com/Chavocan/engine) (**Security** tab → **Report a vulnerability**), if enabled for the repository.
2. If advisories are not available, contact the maintainers through a **private channel** they publish on the repo (e.g. email in maintainer profile). Update this section when a dedicated security contact email exists.

Include:

- A short description of the issue and impact
- Steps to reproduce (or a proof-of-concept)
- Affected component (e.g. `aetherforge_serve`, Python SDK)
- Optional: suggested fix or patch

We aim to acknowledge reports within a few business days. Critical issues (RCE, auth bypass on exposed surfaces) get priority.

## Scope notes

- The control plane defaults to **localhost** (`127.0.0.1`) in development; binding to `0.0.0.0` or deploying without TLS/auth is **out of scope** for “secure by default” guarantees in early versions—document deployment hardening separately.
- **AI-facing APIs** (`/v1/...`) should be treated as **powerful**: rate limits and session quotas exist but are not a substitute for network-level controls in production.
