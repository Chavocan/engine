#!/usr/bin/env python3
"""Fail if committed JSON Schemas drift from generated sources of truth.

Run from repo root. Requires `cargo` on PATH.

- **action.schema.json** — compared to **`aetherforge_export_action_schema`** (schemars).
- **observation.schema.json** — `properties.farm` / `properties.world` must match **`schema_fragments/observation_*_property.json`** (hand-heavy slices; fragments are canonical).
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

_REPO = Path(__file__).resolve().parents[1]
_SCHEMA_FRAG = _REPO / "crates" / "aetherforge_schemas" / "schema_fragments"
_FRAGMENT_FARM = _SCHEMA_FRAG / "observation_farm_property.json"
_FRAGMENT_WORLD = _SCHEMA_FRAG / "observation_world_property.json"
_OBSERVATION_SCHEMA = _REPO / "schemas" / "v1" / "observation.schema.json"


def _strip_meta(d: dict) -> dict:
    out = dict(d)
    out.pop("$id", None)
    return out


def _run_action_export() -> dict:
    raw = subprocess.check_output(
        [
            "cargo",
            "run",
            "-p",
            "aetherforge_schemas",
            "--features",
            "schema-export",
            "--bin",
            "aetherforge_export_action_schema",
            "--quiet",
        ],
        cwd=_REPO,
        text=True,
    )
    return json.loads(raw)


def _check_action() -> None:
    gen = _run_action_export()
    path = _REPO / "schemas" / "v1" / "action.schema.json"
    committed = json.loads(path.read_text(encoding="utf-8"))
    g = _strip_meta(gen)
    c = _strip_meta(committed)
    if g != c:
        sys.stderr.write(
            "action.schema.json drifts from schemars export.\n"
            f"--- generated\n{json.dumps(g, indent=2, sort_keys=True)}\n"
            f"--- committed (no $id)\n{json.dumps(c, indent=2, sort_keys=True)}\n"
        )
        raise SystemExit(1)
    print(f"OK: action.schema.json matches export ({path})")


def _check_observation_farm_fragment() -> None:
    obs = json.loads(_OBSERVATION_SCHEMA.read_text(encoding="utf-8"))
    farm = (obs.get("properties") or {}).get("farm")
    fragment = json.loads(_FRAGMENT_FARM.read_text(encoding="utf-8"))
    if farm != fragment:
        sys.stderr.write(
            "observation.schema.json `properties.farm` drifts from schema_fragments "
            f"observation_farm_property.json.\n--- in schema\n{json.dumps(farm, indent=2, sort_keys=True)}\n"
            f"--- fragment\n{json.dumps(fragment, indent=2, sort_keys=True)}\n"
        )
        raise SystemExit(1)
    print(
        "OK: observation.schema.json farm slice matches "
        f"crates/aetherforge_schemas/schema_fragments/observation_farm_property.json"
    )


def _check_observation_world_fragment() -> None:
    obs = json.loads(_OBSERVATION_SCHEMA.read_text(encoding="utf-8"))
    world = (obs.get("properties") or {}).get("world")
    fragment = json.loads(_FRAGMENT_WORLD.read_text(encoding="utf-8"))
    if world != fragment:
        sys.stderr.write(
            "observation.schema.json `properties.world` drifts from schema_fragments "
            f"observation_world_property.json.\n--- in schema\n{json.dumps(world, indent=2, sort_keys=True)}\n"
            f"--- fragment\n{json.dumps(fragment, indent=2, sort_keys=True)}\n"
        )
        raise SystemExit(1)
    print(
        "OK: observation.schema.json world slice matches "
        "crates/aetherforge_schemas/schema_fragments/observation_world_property.json"
    )


def main() -> None:
    _check_action()
    _check_observation_farm_fragment()
    _check_observation_world_fragment()
    print("OK: all wired schema drift checks passed")


if __name__ == "__main__":
    main()
