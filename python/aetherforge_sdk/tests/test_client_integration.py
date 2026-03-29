from __future__ import annotations

import os

import pytest

from aetherforge_sdk import AetherForgeClient


@pytest.mark.skipif(
    not os.environ.get("AETHERFORGE_TEST_URL"),
    reason="set AETHERFORGE_TEST_URL to running aetherforge_serve (e.g. http://127.0.0.1:8787)",
)
def test_round_trip_e2e() -> None:
    base_url = os.environ["AETHERFORGE_TEST_URL"]
    with AetherForgeClient(base_url) as c:
        created = c.create_session(seed=424242)
        assert created.session_id
        assert created.schema_version == "1.0.0"
        assert created.seed == 424242

        ack = c.apply_action(created.session_id, "sdk_probe", payload={})
        assert ack.ok is True
        assert ack.tick == 1

        obs = c.get_observation(created.session_id)
        assert obs.schema_version == "1.2.0"
        assert obs.tick == 1
        assert "sdk_probe" in obs.message
        assert obs.world.world_version == "1.0.0"
        assert obs.world.entities == []

        c.delete_session(created.session_id)
