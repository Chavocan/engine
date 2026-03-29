#!/usr/bin/env bash
# Phase 7b — Player path must not contain direct `use aetherforge_sim` imports.
# Note: the `aetherforge_cli` *package* still depends on `aetherforge_sim` for other
# binaries; this check enforces the HTTP-only *source* boundary for the player.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILES=(
  "$ROOT/crates/aetherforge_cli/src/player.rs"
  "$ROOT/crates/aetherforge_cli/src/bin/aetherforge_player.rs"
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
echo "OK: player sources have no direct aetherforge_sim imports"
