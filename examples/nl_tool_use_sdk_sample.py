#!/usr/bin/env python3
"""NL-1 sketch: map a structured "LLM output" dict to `AetherForgeClient.apply_action`.

This script does **not** call OpenAI/Anthropic by default — it demonstrates validation +
HTTP using the SDK. Point `AETHERFORGE_TEST_URL` at a running `aetherforge_serve`, then:

  python examples/nl_tool_use_sdk_sample.py

To integrate a real model, replace `fake_llm_propose_action()` with tool-use output from
your provider, keeping the same validation path.
"""

from __future__ import annotations

import json
import os
import sys
from typing import Any

from aetherforge_sdk import AetherForgeClient
from aetherforge_sdk.models import Action


def fake_llm_propose_action(user_text: str) -> dict[str, Any]:
    """Stub: pretend the model chose `step_once`. Replace with real tool-use JSON."""
    _ = user_text
    return {"schema_version": "1.0.0", "kind": "step_once", "payload": {}}


def validate_action(data: dict[str, Any]) -> Action:
    """Ensure pydantic agrees with the wire schema before sending."""
    return Action.model_validate(data)


def main() -> None:
    base = os.environ.get("AETHERFORGE_TEST_URL", "http://127.0.0.1:8787").rstrip("/")
    text = " ".join(sys.argv[1:]) or "advance the simulation one tick"

    raw = fake_llm_propose_action(text)
    action = validate_action(raw)

    with AetherForgeClient(base_url=base) as client:
        sid = client.create_session(seed=42).session_id
        ack = client.apply_action(
            sid,
            action.kind,
            payload=action.payload,
            schema_version=action.schema_version,
        )
        obs = client.get_observation(sid)
        print(json.dumps({"ack": ack.model_dump(), "observation_tick": obs.tick}, indent=2))


if __name__ == "__main__":
    main()
