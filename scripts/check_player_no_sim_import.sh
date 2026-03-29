#!/usr/bin/env bash
# Phase 7b / ADR 0002 — `aetherforge_player` crate must not import the sim kernel in source.
# The package has no `aetherforge_sim` dependency; this script scans player sources.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILES=(
  "$ROOT/crates/aetherforge_player/src/player.rs"
  "$ROOT/crates/aetherforge_player/src/main.rs"
)
for f in "${FILES[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "missing $f" >&2
    exit 1
  fi
  if grep -Eq '^\s*use\s+::?aetherforge_sim\b' "$f"; then
    echo "forbidden direct sim kernel import in $f" >&2
    exit 1
  fi
done
echo "OK: aetherforge_player sources have no direct aetherforge_sim imports"
