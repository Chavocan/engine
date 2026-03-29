#!/usr/bin/env bash
# Golden bundle: offline farm demo + in-process HTTP session tests + HTTP player loop.
# Fails on any error. Run from repo root (Linux/macOS/Git Bash); CI uses this explicitly.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
cargo test -p aetherforge_control --test http_sessions -q
cargo test -p aetherforge_player --test player_http_loop -q
