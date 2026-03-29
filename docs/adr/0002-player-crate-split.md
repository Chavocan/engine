# ADR 0002: Split `aetherforge_player` into its own crate

## Status

**Accepted** — implemented in workspace (see migration checklist below).

## Context

- **Previously:** The player binary lived under **`aetherforge_cli`** with logic in **`src/player.rs`**, while that package also shipped **`aetherforge_serve`**, **`aetherforge_scenario`**, and **`aetherforge_headless`** — all sharing a **`Cargo.toml` that listed `aetherforge_sim`**, which obscured whether the autonomous player truly avoided the sim kernel.
- **Now:** **`aetherforge_player`** is a dedicated workspace crate; **`aetherforge_cli`** keeps only the sim-adjacent binaries. Phase 7b’s **`scripts/check_player_no_sim_import.sh`** scans **`crates/aetherforge_player`** sources.
- Splitting makes the **HTTP-only boundary** enforceable by **Cargo** (the player crate does not depend on **`aetherforge_sim`**), reduces mistaken imports, and shrinks graphs for agents that only build the player.

## Decision

1. Add a workspace member **`crates/aetherforge_player`** with:
   - **Package name:** `aetherforge_player` (matches the binary name stakeholders already document).
   - **Single binary:** `src/main.rs` (or keep `src/bin/aetherforge_player.rs` if preferred — one binary either way).
   - **Library (optional):** `src/lib.rs` exporting `run_player`, `PlayerConfig`, `PlayerPolicy` if tests or other bins need it; otherwise keep logic in `main` + modules under `src/`.
2. **Move** (not copy) **`player.rs`** from **`aetherforge_cli`** into the new crate (e.g. `crates/aetherforge_player/src/player.rs`).
3. **Dependencies (runtime):** `reqwest`, `tokio`, `clap`, `rand`, `serde`, `serde_json` — **no** `aetherforge_sim`. **Dev-dependencies** may include `aetherforge_control` + `axum` for in-process integration tests only.
4. **Remove** the `[[bin]] aetherforge_player` entry and **`player` module** from **`aetherforge_cli`** once the new crate builds.
5. **CI / guard script:** Update **`scripts/check_player_no_sim_import.sh`** to scan **`crates/aetherforge_player/src/**/*.rs`** (or explicit file list) instead of (or in addition to) the old paths under **`aetherforge_cli`**.
6. **Tests:** Move or duplicate **`crates/aetherforge_cli/tests/player_http_loop.rs`** and **`player_no_sim_guard.rs`** under **`aetherforge_player/tests/`**, or keep integration tests in `aetherforge_cli` that **spawn** the new binary by package name — prefer tests **co-located** with the player crate.
7. **Docs / README / CONTRIBUTING:** Replace `cargo run -p aetherforge_cli --bin aetherforge_player` with **`cargo run -p aetherforge_player`** (or document both during a short deprecation window if needed).

## Consequences

- **Positive:** Dependency graph truth in `Cargo.toml`; harder to accidentally add `aetherforge_sim` to the player; clearer ownership for autonomous-agent UX.
- **Negative:** One more crate to publish/version (if ever published); slightly more workspace boilerplate; migration PR touches CI, docs, and possibly `Dockerfile` or scripts that reference the old `-p` / `--bin` pair.
- **Workspace:** Add `"crates/aetherforge_player"` to root **`[workspace.members]`**.

## Rejected alternatives

| Alternative | Reason not chosen (for now) |
|-------------|------------------------------|
| **Keep player in `aetherforge_cli`, rely only on grep CI** | Does not encode the boundary in Cargo; sim stays a transitive temptation in the same package. |
| **Feature-flag player inside `aetherforge_cli`** | Still one package; optional deps do not remove `aetherforge_sim` from the default resolve for other bins in the same crate without careful feature wiring. |

## Migration checklist (implementation PR)

- [x] `cargo new` / manual crate under `crates/aetherforge_player` with workspace metadata aligned with root.
- [x] Move `player.rs` + wire `main` from existing bin.
- [x] `cargo test -p aetherforge_player`; full `cargo test` workspace.
- [x] Update `check_player_no_sim_import.sh` paths.
- [x] Update `README.md`, `docs/demo-showcase.md`, `phase6` / `phase7b` docs.
- [x] `CHANGELOG.md` Unreleased: note `cargo run` path change.
