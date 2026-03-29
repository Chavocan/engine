#!/usr/bin/env python3
"""Assert JSON Schema `required` keys for Observation are covered by the SDK model.

Run from anywhere (uses paths relative to repo root):
  python scripts/check_observation_contract.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

# Repo root: .../python/aetherforge_sdk/scripts/this_file -> parents[3]
_REPO_ROOT = Path(__file__).resolve().parents[3]
_SCHEMA = _REPO_ROOT / "schemas" / "v1" / "observation.schema.json"

# Ensure `src` is importable when run as script
_SRC = Path(__file__).resolve().parents[1] / "src"
if str(_SRC) not in sys.path:
    sys.path.insert(0, str(_SRC))

from aetherforge_sdk.models import Observation, WorldSnapshot  # noqa: E402


def _model_fields(cls: type) -> set[str]:
    return set(cls.model_fields.keys())


def _schema_required_top(schema_path: Path) -> set[str]:
    data = json.loads(schema_path.read_text(encoding="utf-8"))
    req = data.get("required") or []
    if not isinstance(req, list):
        raise SystemExit("schema: top-level `required` must be a list")
    return set(req)


def _world_required(schema_path: Path) -> set[str]:
    data = json.loads(schema_path.read_text(encoding="utf-8"))
    world = (data.get("properties") or {}).get("world") or {}
    req = world.get("required") or []
    if not isinstance(req, list):
        raise SystemExit("schema: world.required must be a list")
    return set(req)


def main() -> None:
    if not _SCHEMA.is_file():
        raise SystemExit(f"schema not found: {_SCHEMA}")

    obs_req = _schema_required_top(_SCHEMA)
    obs_fields = _model_fields(Observation)
    missing = obs_req - obs_fields
    if missing:
        raise SystemExit(
            f"Observation model missing fields for schema required: {sorted(missing)}"
        )

    world_req = _world_required(_SCHEMA)
    if "world" not in obs_fields:
        raise SystemExit("Observation must define `world`")
    world_fields = _model_fields(WorldSnapshot)
    wmissing = world_req - world_fields
    if wmissing:
        raise SystemExit(
            f"WorldSnapshot missing fields for schema world.required: {sorted(wmissing)}"
        )

    print(f"OK: observation.schema.json required keys are covered by SDK ({_SCHEMA})")


if __name__ == "__main__":
    main()
