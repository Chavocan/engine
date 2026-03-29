#!/usr/bin/env python3
"""Ping the control plane: create session → noop action → print observation.

Usage:
  AETHERFORGE_URL=http://127.0.0.1:8787 python examples/ping_observation.py
  python examples/ping_observation.py http://127.0.0.1:8787
"""

from __future__ import annotations

import os
import sys

from aetherforge_sdk import AetherForgeClient


def main() -> None:
    base = (
        (sys.argv[1] if len(sys.argv) > 1 else None)
        or os.environ.get("AETHERFORGE_URL")
        or os.environ.get("AETHERFORGE_TEST_URL")
    )
    if not base:
        print(
            "Set AETHERFORGE_URL or pass base URL as argv[1] "
            "(e.g. http://127.0.0.1:8787)",
            file=sys.stderr,
        )
        sys.exit(1)
    with AetherForgeClient(base) as c:
        s = c.create_session()
        c.apply_action(s.session_id, "noop", payload={})
        obs = c.get_observation(s.session_id)
        print(obs.model_dump_json(indent=2))
        c.delete_session(s.session_id)


if __name__ == "__main__":
    main()
