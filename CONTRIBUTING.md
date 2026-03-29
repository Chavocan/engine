# Contributing

## CI layout

| Workflow | File | When | What |
|----------|------|------|------|
| **CI** | `.github/workflows/ci.yml` | `push`, `pull_request` | **Rust:** `cargo test --verbose`; SSE feature tests; player import script; **offline demo** `cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json`. **Python:** `pip install -e "./python/aetherforge_sdk[dev]"`, `ruff check python/aetherforge_sdk`, `python python/aetherforge_sdk/scripts/check_observation_contract.py`, `pytest python/aetherforge_sdk/tests -q` (includes **mock** HTTP tests; no server). |
| **SDK E2E** | `.github/workflows/sdk-e2e.yml` | **Manual** (`workflow_dispatch`) | `pytest python/aetherforge_sdk/tests/test_client_integration.py` with `AETHERFORGE_TEST_URL` = input. **Note:** GitHub-hosted runners cannot reach your laptop’s `127.0.0.1`; use a tunnel, a deployed server, or a **self-hosted** runner. |

### Local parity (recommended before push)

```bash
cargo test
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
cd python/aetherforge_sdk && pip install -e ".[dev]" && ruff check . && python scripts/check_observation_contract.py && pytest -q
```

### Publish metadata (Python)

`python/aetherforge_sdk/pyproject.toml` includes `[project.urls]` pointing at [github.com/Chavocan/engine](https://github.com/Chavocan/engine). Adjust if the canonical URL changes.
