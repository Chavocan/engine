#!/usr/bin/env python3
"""Minimal terminal \"HUD\": formatted observation (tick, mission, farm) — R2 client surface v1.

Requires a running server. For farm fields and mission outcome, build with farm-stub:

  cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_serve

Usage:
  AETHERFORGE_URL=http://127.0.0.1:8787 python examples/observation_hud.py
  python examples/observation_hud.py http://127.0.0.1:8787
  python examples/observation_hud.py http://127.0.0.1:8787 --farm-stub-demo --seed 42
"""

from __future__ import annotations

import argparse
import os
import sys

from aetherforge_sdk import AetherForgeClient
from aetherforge_sdk.models import Observation


def _lines_for_observation(obs: Observation) -> list[str]:
    lines: list[str] = [
        f"tick={obs.tick}  run_id={obs.run_id}",
        f"message: {obs.message}",
        f"rng_draw: {obs.rng_draw}",
    ]
    if obs.mission is not None:
        lines.append(f"mission.outcome: {obs.mission.outcome}")
    else:
        lines.append("mission: (none)")
    farm = obs.farm
    if farm is not None:
        lines.append(f"farm: day={farm.day}  time={farm.time_minutes} min")
        if not farm.plots:
            lines.append("  plots: (empty)")
        else:
            lines.append("  plots:")
            for i, p in enumerate(farm.plots):
                cx = p.coord.get("x", "?")
                cy = p.coord.get("y", "?")
                lines.append(f"    [{i}] ({cx},{cy}) {p.crop!r}  growth_stage={p.growth_stage}")
        inv = farm.inventory.items
        if inv:
            lines.append(f"  inventory: {dict(inv)}")
        else:
            lines.append("  inventory: (empty)")
    else:
        lines.append(
            "farm: (none - server likely built without farm-stub, or no farm state yet)"
        )
    return lines


def _run_farm_stub_demo(c: AetherForgeClient, session_id: str) -> None:
    """One plant → grow → harvest loop (matches short farm demo)."""
    steps = [
        ("farm_plant", {}),
        ("farm_advance_day", {}),
        ("farm_advance_day", {}),
        ("farm_advance_day", {}),
        ("farm_harvest", {}),
    ]
    for kind, payload in steps:
        c.apply_action(session_id, kind, payload=payload)


def main() -> None:
    parser = argparse.ArgumentParser(description="Print a readable observation HUD.")
    parser.add_argument(
        "base_url",
        nargs="?",
        default=None,
        help="Control plane base URL (or set AETHERFORGE_URL / AETHERFORGE_TEST_URL)",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=None,
        help="Optional session seed",
    )
    parser.add_argument(
        "--farm-stub-demo",
        action="store_true",
        help="Run a 5-step farm stub loop before printing (needs farm-stub server)",
    )
    args = parser.parse_args()
    base = (
        args.base_url
        or os.environ.get("AETHERFORGE_URL")
        or os.environ.get("AETHERFORGE_TEST_URL")
    )
    if not base:
        print(
            "Set AETHERFORGE_URL or pass base URL as positional arg "
            "(e.g. http://127.0.0.1:8787)",
            file=sys.stderr,
        )
        sys.exit(1)

    with AetherForgeClient(base) as c:
        s = c.create_session(seed=args.seed)
        if args.farm_stub_demo:
            _run_farm_stub_demo(c, s.session_id)
        else:
            c.apply_action(s.session_id, "noop", payload={})
        obs = c.get_observation(s.session_id)
        print("\n".join(_lines_for_observation(obs)))
        c.delete_session(s.session_id)


if __name__ == "__main__":
    main()
