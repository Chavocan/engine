# Phase 7b — Player / sim kernel boundary (CI)

## What we enforce

- **Script:** `scripts/check_player_no_sim_import.sh` (also mirrored by Rust test `player_no_sim_guard`).
- **Rule:** `crates/aetherforge_cli/src/player.rs` and `src/bin/aetherforge_player.rs` must not contain a **direct** import of the sim kernel crate, e.g. `use aetherforge_sim::...` or `extern crate aetherforge_sim`.
- **Comments** (`//`, `//!`) are ignored by the Rust test; the shell script only matches `use` at line start after whitespace.

## CI

```yaml
- name: Player HTTP-only source boundary
  run: bash scripts/check_player_no_sim_import.sh
```

(See `.github/workflows/ci.yml`.)

## Why not `cargo tree --bin aetherforge_player -i aetherforge_sim`?

The **`aetherforge_cli` package** legitimately depends on **`aetherforge_sim`** for `aetherforge_headless` and `aetherforge_scenario`. Cargo resolves dependencies **per package**, not per binary, so the sim crate still appears in the dependency graph for **all** binaries in that package until the player is split into its own crate.

## Documentation & Learning Log (Employee AI)

- **Invariant:** “No sim in the player **code path**” matches remote-AI parity; package-level linkage is an implementation detail until a **`aetherforge_player` crate** split.
- **Next:** If we need a true graph-empty check, extract a thin `aetherforge_player` package with only HTTP deps.
