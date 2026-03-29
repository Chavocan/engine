# Release closure — finish line 1 (ops)

Use this checklist when you want the repo **released and auditable** without adding product scope. Canonical commands also appear in [`release-v0.1.0.md`](release-v0.1.0.md).

## Pre-merge

- [ ] **`cargo test --workspace`** (or rely on green PR CI).
- [ ] **`cargo clippy --workspace --all-features -- -D warnings`** (optional but recommended).
- [ ] **`python scripts/check_schema_drift.py`** (if schemas or Rust `Action` changed).
- [ ] Python SDK: **`pip install -e "./python/aetherforge_sdk[dev]"`**, **`ruff check`**, **`check_observation_contract.py`**, **`pytest`**.

## After merge to `main`

- [ ] Confirm [**`.github/workflows/ci.yml`**](../.github/workflows/ci.yml) is **green** on `main`.
- [ ] **Push annotated tag** if not on the remote:  
  `git push origin v0.1.0`  
  (Create the tag first if needed: see [`release-v0.1.0.md`](release-v0.1.0.md).)

## Documentation

- [ ] [`CHANGELOG.md`](../CHANGELOG.md) matches what you are tagging.
- [ ] Optional: append a line to [`docs/aetherforge-designer-log.md`](aetherforge-designer-log.md) or director log when you cut a release.

**Agents:** run the pre-merge commands; humans only need to **merge** and **push tags** where git credentials live outside the agent.
