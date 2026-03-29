# Phase 7b — Player / sim kernel boundary (CI)

## What we enforce

- **Script:** `scripts/check_player_no_sim_import.sh` (mirrored by Rust test `player_no_sim_guard` in **`aetherforge_player`**).
- **Rule:** `crates/aetherforge_player/src/player.rs` and `src/main.rs` must not contain a **direct** import of the sim kernel crate, e.g. `use aetherforge_sim::...` or `extern crate aetherforge_sim`.
- **Package:** **`aetherforge_player`** has **no** `aetherforge_sim` dependency in `Cargo.toml` (ADR 0002).
- **Comments** (`//`, `//!`) are ignored by the Rust test; the shell script only matches `use` at line start after whitespace.

## CI

```yaml
- name: Player HTTP-only source boundary
  run: bash scripts/check_player_no_sim_import.sh
```

(See `.github/workflows/ci.yml`.)

## Optional graph check

Now that the player is a **separate crate**, verify **runtime** (non-dev) edges only:

```bash
cargo tree -p aetherforge_player -e normal
```

Expect **no `aetherforge_sim`** in that output.

**Note:** `cargo tree -p aetherforge_player -i aetherforge_sim` (without `-e normal`) may still show a path **via `dev-dependencies`** (`aetherforge_control` for in-process integration tests). That is **expected** and does **not** contradict “no sim in player **source**” or “no sim required to **ship** the player binary.”

The **`aetherforge_cli`** package still depends on **`aetherforge_sim`** for other binaries — unrelated to the player crate.
