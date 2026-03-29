#!/usr/bin/env bash
# Fast local smoke: headless + offline farm scenario. Run from repo root.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== aetherforge_headless (one observation line) =="
cargo run -q -p aetherforge_cli --bin aetherforge_headless

echo ""
echo "== aetherforge_scenario farm_demo_loop (offline, farm-stub) =="
cargo run -q -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json

echo ""
echo "OK: dev_smoke passed (kernel + offline scenario). For HTTP + SDK see docs/getting-started.md"
