# Flagship farm scenario (R1)

Reference **stub** script for agents and CI: **two** full plant → grow → harvest cycles in one session, ending with **`mission.outcome: won`**.

| File | Mode |
|------|------|
| [`examples/flagship_farm_two_cycles.json`](../examples/flagship_farm_two_cycles.json) | Offline / in-process (`--offline`) |
| [`examples/flagship_farm_http.json`](../examples/flagship_farm_http.json) | HTTP (`aetherforge_serve` with **`farm-stub`**, `base_url` set) |

Shorter single-cycle demo: [`examples/farm_demo_loop.json`](../examples/farm_demo_loop.json).

**Requires:** Rust feature **`farm-stub`** on **`aetherforge_cli`** / **`aetherforge_serve`**.

**Run (offline):**

```bash
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/flagship_farm_two_cycles.json
```

**Run (HTTP):** start server, then:

```bash
cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- examples/flagship_farm_http.json
```

See **R1** / **R4** in [`roadmap-to-complete-project.md`](roadmap-to-complete-project.md).
