# Phase 4b — Scenario runner (`aetherforge_scenario`)

Soak-test and CI-friendly regression using a small JSON **script** of control-plane steps.

## Binary

```bash
cargo run -p aetherforge_cli --bin aetherforge_scenario -- --offline path/to/scenario.json
# HTTP (requires `aetherforge_serve` and `base_url` in JSON):
cargo run -p aetherforge_cli --bin aetherforge_scenario -- path/to/scenario.json
```

## Scenario file shape

```json
{
  "base_url": "http://127.0.0.1:8787",
  "seed": 0,
  "steps": [
    {
      "use": "batch",
      "actions": [
        { "schema_version": "1.0.0", "kind": "a", "payload": {} },
        { "schema_version": "1.0.0", "kind": "b", "payload": {} }
      ],
      "expect_tick": 2
    },
    {
      "use": "single",
      "kind": "noop",
      "schema_version": "1.0.0",
      "payload": {},
      "expect_tick": 3
    }
  ]
}
```

- **`seed`:** Passed to `POST /v1/sessions` (HTTP) or `SimulationConfig` (offline).
- **`base_url`:** Required for HTTP mode; ignored when using **`--offline`**.
- **`steps`:** Each step is a tagged object with **`use`**: **`batch`** or **`single`**.
  - **`batch`:** `actions` is an array of v1 **Action** objects (same as control plane). Applied in order with the same semantics as `POST .../actions` (per-action enqueue + `step`).
  - **`single`:** One action via `kind`, optional `schema_version` (default `"1.0.0"`), optional `payload` (default `{}`).
- **`expect_tick`** (optional per step): After the step, observation `tick` must equal this value (offline reads sim directly; HTTP uses `GET .../observation`). Omit to skip the assertion.
- **`expect_mission_outcome`** (optional, file-level): After all steps, assert `Observation.mission.outcome` is **`won`** or **`lost`** (see **R0** in [`roadmap-to-complete-project.md`](roadmap-to-complete-project.md)). Requires sim/server logic that sets **`mission`** (e.g. **`farm-stub`** harvest demo).

## Exit status

- **0:** All steps succeeded and optional `expect_tick` checks passed.
- **1:** Failure; one **JSON object** line on **stderr** with `ok: false`, `step_index`, `reason`, and optional `expected_tick` / `actual_tick` (or `null` when not applicable).

## Offline vs HTTP

| Mode | Behavior |
|------|-----------|
| `--offline` | In-process `Simulation` only; no network; suitable for default CI. |
| default | `reqwest` against `base_url`; creates a session then runs steps. |

**Invariant:** For the same `seed` and step list, **tick** progression matches between modes (HTTP session uses the same kernel semantics as offline).

## Implementation

- Types and runners: `crates/aetherforge_cli/src/scenario.rs`
- Entry: `crates/aetherforge_cli/src/bin/aetherforge_scenario.rs`
