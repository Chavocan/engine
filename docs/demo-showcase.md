# Demo showcase — AetherForge Engine (~5 minutes)

Use this script for a **live demo** or screen recording. Commands assume a POSIX shell; on Windows PowerShell, run equivalent paths from the repo root.

## Prerequisites

- Rust toolchain (`cargo`, `rustup`)
- Optional: Python 3.12+ for SDK demo

## A — Offline farming loop (no server)

Build with the farming stub feature so `farm` appears in observations:

```bash
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
```

**Expected:** exit code **0**. Final stdout line is JSON **observation** with `farm.inventory.items` containing **`harvested_stub_crop`: 1** and **empty `plots`** after harvest.

**Scenario file** `examples/farm_demo_loop.json` (checked in next to `farm_5b_scenario.json`):

```json
{
  "seed": 42,
  "steps": [
    { "use": "single", "kind": "farm_plant", "expect_tick": 1 },
    { "use": "single", "kind": "farm_advance_day", "expect_tick": 2 },
    { "use": "single", "kind": "farm_advance_day", "expect_tick": 3 },
    { "use": "single", "kind": "farm_advance_day", "expect_tick": 4 },
    { "use": "single", "kind": "farm_harvest", "expect_tick": 5 }
  ]
}
```

## B — HTTP control plane + headless observation

Terminal 1:

```bash
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_serve
```

Terminal 2 (example with `curl`; adjust `SESSION_ID` from JSON):

```bash
curl -s -X POST http://127.0.0.1:8787/v1/sessions -H "Content-Type: application/json" -d "{\"seed\":42}"
curl -s -X POST http://127.0.0.1:8787/v1/sessions/SESSION_ID/action -H "Content-Type: application/json" -d "{\"kind\":\"farm_plant\"}"
curl -s http://127.0.0.1:8787/v1/sessions/SESSION_ID/observation
```

**Narration:** “This is the AI-native surface: versioned JSON state, actions over HTTP, same simulation headless or served.”

## C — Play log (AI-tailable)

```bash
export AETHERFORGE_PLAY_LOG=1
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_serve
```

Drive a few actions; point out **JSON lines** with `event` for `action_applied`, `tick_advanced`, `farm_day_advanced`, and after a ripe **`farm_harvest`**, **`farm_harvested`** ( **`farm-stub`** serve only).

## D — Autonomous player (optional)

With server up:

```bash
cargo run -p aetherforge_player -- --base-url http://127.0.0.1:8787 --seed 1 --policy round_robin --max-steps 8 --intents farm_plant,farm_advance_day,farm_harvest
```

> Requires **`farm-stub`** so **`farm_harvest`** is applied in sim.

## E — Python SDK (optional)

```bash
pip install -e "./python/aetherforge_sdk[dev]"
export AETHERFORGE_TEST_URL=http://127.0.0.1:8787
pytest python/aetherforge_sdk/tests/test_client_integration.py -q
```

## Recording checklist

- [ ] Show **README** quickstart or this doc in the IDE.
- [ ] Run **offline scenario** (full loop once harvest lands).
- [ ] Show **one** raw observation JSON (pretty-printed).
- [ ] Mention **CI** (`.github/workflows/ci.yml`) as regression safety.

## Documentation & Learning Log (Lead Director)

- **Accomplished this cycle:** Demo script for stakeholders; ties to roadmap “demo showcase” milestone.
- **Processes used:** Copy-paste commands + expected outcomes.
- **Pitfalls / observations:** `farm-stub` must be enabled for farm intents; default build omits `farm` key.
- **Learnings / best practices:** Offline scenario first avoids firewall/port issues during recordings.
- **Next cycle action items:** Optional CI job `demo-loop` running `examples/farm_demo_loop.json`.
