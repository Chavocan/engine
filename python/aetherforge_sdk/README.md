# aetherforge_sdk

Thin **sync** HTTP client for the AetherForge control plane (`httpx` + **Pydantic v2**).

## Parity

Targets **`Observation.schema_version` `1.1.0`** (includes `world` + `rng_draw`) and session create body **`{}`** minimum (optional `"seed": <int>`), matching `docs/phase1c-ai-interface-spec-v0.md` and the Rust workspace.

## Install (editable)

From this directory:

```bash
pip install -e ".[dev]"
```

Dev tools: **pytest**, **ruff** (`ruff check .` from this directory).

## Example script

With the server running (`cargo run -p aetherforge_cli --bin aetherforge_serve` from repo root):

```bash
# Windows PowerShell
$env:AETHERFORGE_URL = "http://127.0.0.1:8787"
python examples/ping_observation.py
```

Or: `python examples/ping_observation.py http://127.0.0.1:8787`  
(`AETHERFORGE_TEST_URL` is also accepted.)

## Example (inline)

```python
from aetherforge_sdk import AetherForgeClient

with AetherForgeClient("http://127.0.0.1:8787") as c:
    s = c.create_session(seed=1)
    c.apply_action(s.session_id, "noop", payload={})
    obs = c.get_observation(s.session_id)
    print(obs.model_dump_json(indent=2))
    # SSE tail (requires server built with Rust feature `sse-obs`):
    # for o in c.observe_stream(s.session_id):
    #     print(o.tick)
    c.delete_session(s.session_id)
```

## Schema ↔ SDK drift

From this directory:

```bash
python scripts/check_observation_contract.py
```

Ensures `schemas/v1/observation.schema.json` top-level and `world.*` `required` keys are present on the Pydantic models.

## Tests

**Always-on (no server):** `pytest` runs **mock** HTTP tests (`test_client_mock.py`).

**Optional E2E** (live server):

```bash
set AETHERFORGE_TEST_URL=http://127.0.0.1:8787
pytest
```

If `AETHERFORGE_TEST_URL` is unset, `test_client_integration.py` is **skipped**.

## Play log

Server JSON play lines: set **`AETHERFORGE_PLAY_LOG=1`** (see repo `docs/phase2c-play-log.md`).
