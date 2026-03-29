# Getting started — use the engine in minutes

Follow this once after **[`README.md`](../README.md)** prerequisites. No paid services; everything is local.

## 1. Toolchain

| Need | Notes |
|------|--------|
| **Rust** stable | [rustup.rs](https://rustup.rs/) — `cargo`, `rustc` |
| **Python 3.10+** (optional) | For **`python/aetherforge_sdk`** — 3.12 matches CI |

```bash
rustc --version
cargo --version
```

## 2. Clone and verify the workspace

```bash
git clone https://github.com/Chavocan/engine.git
cd engine
cargo test --workspace
```

Green tests mean the **simulation kernel**, **control plane** (in-process tests), **CLI**, and **player** wiring match CI.

**One-command smoke (faster than full test):**

- **Windows:** `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/dev_smoke.ps1`
- **Linux / macOS / Git Bash:** `bash scripts/dev_smoke.sh`

## 3. Headless observation (no server)

Prints **one** JSON observation line — same wire shape as HTTP:

```bash
cargo run -p aetherforge_cli --bin aetherforge_headless
```

## 4. Offline scripted loop (farm stub)

Runs a small **plant → grow → harvest** script in-process (no network):

```bash
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
```

Exit code **0** means ticks matched expectations.

## 5. HTTP control plane + curl

**Terminal A** — start the server (defaults to `127.0.0.1:8787`):

```bash
cargo run -p aetherforge_cli --bin aetherforge_serve
```

**Terminal B** — create a session and read observation:

```bash
curl -s -X POST http://127.0.0.1:8787/v1/sessions -H "Content-Type: application/json" -d "{\"seed\":42}"
```

Copy `session_id` from the JSON, then:

```bash
curl -s -X POST http://127.0.0.1:8787/v1/sessions/SESSION_ID/action \
  -H "Content-Type: application/json" \
  -d "{\"schema_version\":\"1.0.0\",\"kind\":\"noop\",\"payload\":{}}"

curl -s http://127.0.0.1:8787/v1/sessions/SESSION_ID/observation
```

Optional: set **`AETHERFORGE_HTTP_ADDR=127.0.0.1:9000`** before `aetherforge_serve` to change bind (see **[`deployment.md`](deployment.md)**).

## 6. Python SDK

From repo root:

```bash
pip install -e "./python/aetherforge_sdk[dev]"
```

With **Terminal A** still running `aetherforge_serve`:

```bash
# PowerShell
$env:AETHERFORGE_TEST_URL = "http://127.0.0.1:8787"
pytest python/aetherforge_sdk/tests/test_client_integration.py -q
```

Or run the small example:

```bash
python python/aetherforge_sdk/examples/ping_observation.py http://127.0.0.1:8787
```

Formatted **terminal HUD** (tick, mission, farm — use **`farm-stub`** serve for farm fields):

```bash
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_serve
# other terminal:
python python/aetherforge_sdk/examples/observation_hud.py http://127.0.0.1:8787 --farm-stub-demo --seed 42
```

## 7. Autonomous player (optional)

Requires **Terminal A** (`aetherforge_serve`). **Terminal B**:

```bash
cargo run -p aetherforge_player -- --base-url http://127.0.0.1:8787 --seed 1 --policy round_robin --max-steps 5 --intents noop
```

## Where to go next

| Goal | Doc |
|------|-----|
| **Mission after kernel** — win conditions, flagship scenario, client, autonomous QA | [`roadmap-to-complete-project.md`](roadmap-to-complete-project.md) **§ On the rails** |
| Full demo script (recording / stakeholders) | [`demo-showcase.md`](demo-showcase.md) |
| API shapes and errors | [`phase1c-ai-interface-spec-v0.md`](phase1c-ai-interface-spec-v0.md) |
| Production deploy (TLS, proxy, rate limits) | [`deployment.md`](deployment.md) |
| Agent ownership & QA | [`../AGENTS.md`](../AGENTS.md), [`CONTRIBUTING.md`](../CONTRIBUTING.md) |
