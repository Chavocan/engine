# Release checklist — v0.1.0 (AetherForge Engine)

## Tag

- Proposed annotated tag name: **`v0.1.0`**  
  ```bash
  git tag -a v0.1.0 -m "AetherForge Engine v0.1.0"
  ```

## Pre-flight commands (must be green)

Run from repository root:

```bash
cargo test --verbose
bash scripts/check_player_no_sim_import.sh
```

Python SDK:

```bash
pip install -e "./python/aetherforge_sdk[dev]"
ruff check python/aetherforge_sdk
python python/aetherforge_sdk/scripts/check_observation_contract.py
pytest python/aetherforge_sdk/tests -q
```

**SDK smoke (live server):** CI job **`sdk-live-e2e`** runs this against `aetherforge_serve` on every PR. For local parity after editing the client:

```bash
export AETHERFORGE_TEST_URL=http://127.0.0.1:8787
pytest python/aetherforge_sdk/tests/test_client_integration.py -q
```

(Unset `AETHERFORGE_TEST_URL` when done.)

## Feature matrix (local builds)

| Feature | Example command |
|---------|------------------|
| Default (no farm in observation) | `cargo build -p aetherforge_cli --bin aetherforge_serve` |
| Farm slice in observation + intents | `cargo build -p aetherforge_cli --features farm-stub --bin aetherforge_serve` |
| Offline farm scenario | `cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_5b_scenario.json` |

## Versioning policy (current)

- **Per-crate `version`** in each `crates/*/Cargo.toml` (currently **`0.1.0`**). There is **no** single `workspace.package.version` in the root `Cargo.toml`; bump versions crate-by-crate when publishing.

## `cargo publish`

- **Not assumed** for this repo (may be private / monorepo). If publishing to crates.io, publish in dependency order (`aetherforge_farm` → `aetherforge_sim` → …) after setting `description`, `repository`, and `license` as required by crates.io.

## Stakeholder sign-off

- [x] `cargo test` green on `main`
- [x] Python ruff + contract script + mock pytest green
- [x] `scripts/check_player_no_sim_import.sh` green in CI
- [x] `CHANGELOG.md` **v0.1.0** section dated **2026-03-29**
- [x] Tag **`v0.1.0`** created locally (push tags when publishing)

---

## Documentation & Learning Log (Employee AI)

- **File:** `docs/release-v0.1.0.md`; link from `README.md` if stakeholders use it often.
- **Process:** Human checkbox gate avoids tagging on red CI.
- **Pitfall:** Windows devs may lack `bash` locally; rely on **Rust tests** + **CI** for the player script.
- **Learning:** Per-crate versions match the current greenfield layout; a workspace-wide version can be added later if you standardize on unified releases.
