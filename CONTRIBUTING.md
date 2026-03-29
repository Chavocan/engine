# Contributing

## Canonical QA

**Green CI is the QA gate.** [`.github/workflows/ci.yml`](.github/workflows/ci.yml) is the authoritative regression bundle; you do not need a separate manual “did it feel right?” pass for merge confidence. Agents and scripts should mirror CI (or rely on it after push). Agent ownership and backlog order: [`AGENTS.md`](AGENTS.md), [`docs/agent-master-plan.md`](docs/agent-master-plan.md).

**Local parity (same story as CI’s golden step):**

- POSIX: `bash scripts/golden_playthrough.sh`
- Windows (no Git Bash): `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/golden_playthrough.ps1` (or `pwsh -File …` if PowerShell 7 is on `PATH`)

[`docs/demo-showcase.md`](docs/demo-showcase.md) is a **live demo / recording script**, not an extra test matrix.

## CI layout

| Workflow | File | When | What |
|----------|------|------|------|
| **CI** | `.github/workflows/ci.yml` | `push`, `pull_request` | **Rust:** `cargo test --verbose`; **golden** `scripts/golden_playthrough.sh`; SSE feature tests; **NL stub** `cargo test -p aetherforge_control --features nl-interpret-stub --test nl_interpret_stub`; player import script; **`cargo tree -p aetherforge_player -e normal`** must not list **`aetherforge_sim`**; **offline demo** `cargo run … farm_demo_loop.json`; **schema drift** `python scripts/check_schema_drift.py` (action + observation **farm** + **world** fragments); **wgpu** `cargo build -p aetherforge_platform --features headed-smoke`. **Python:** `pip install -e "./python/aetherforge_sdk[dev]"`, `ruff check python/aetherforge_sdk`, `python python/aetherforge_sdk/scripts/check_observation_contract.py`, `pytest python/aetherforge_sdk/tests -q` (includes **mock** HTTP tests; no server). **SDK live E2E:** job **`sdk-live-e2e`** builds `aetherforge_serve`, starts it on `127.0.0.1:8787`, runs `pytest python/aetherforge_sdk/tests/test_client_integration.py` with `AETHERFORGE_TEST_URL` set. |
| **SDK E2E (remote URL)** | `.github/workflows/sdk-e2e.yml` | **Manual** (`workflow_dispatch`) | Same pytest as **`sdk-live-e2e`**, but with a **custom base URL** (tunnel, deployed server, self-hosted runner). Default PR CI already runs localhost SDK E2E in **`sdk-live-e2e`**. |

### Local parity (recommended before push)

**Fast smoke (no full test suite):** `bash scripts/dev_smoke.sh` or `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/dev_smoke.ps1` — headless + offline farm scenario.

```bash
cargo test
# `aetherforge_cli` includes `scenario_tick_json` tests for `aetherforge_scenario --emit-tick-json` / `--quiet`.
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
cd python/aetherforge_sdk && pip install -e ".[dev]" && ruff check . && python scripts/check_observation_contract.py && pytest -q
```

### Publish metadata (Python)

`python/aetherforge_sdk/pyproject.toml` includes `[project.urls]` pointing at [github.com/Chavocan/engine](https://github.com/Chavocan/engine). Adjust if the canonical URL changes.
