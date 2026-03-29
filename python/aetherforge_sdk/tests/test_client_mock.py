"""HTTP contract tests without a live server (httpx.MockTransport)."""

from __future__ import annotations

import json

import httpx
import pytest

from aetherforge_sdk import AetherForgeClient
from aetherforge_sdk.models import Action


@pytest.fixture
def mock_client() -> AetherForgeClient:
    sid = "test-session-uuid-0001"
    seed = 99

    def handler(request: httpx.Request) -> httpx.Response:
        path = request.url.path
        method = request.method
        if method == "POST" and path == "/v1/sessions":
            raw = request.content.decode() if request.content else ""
            body = json.loads(raw if raw.strip() else "{}")
            assert "seed" not in body or isinstance(body.get("seed"), int)
            return httpx.Response(
                201,
                json={
                    "session_id": sid,
                    "schema_version": "1.0.0",
                    "seed": body.get("seed", seed),
                },
            )
        if method == "POST" and path == f"/v1/sessions/{sid}/action":
            return httpx.Response(
                200,
                json={"ok": True, "tick": 1, "schema_version": "1.0.0"},
            )
        if method == "POST" and path == f"/v1/sessions/{sid}/actions":
            body = json.loads(request.content.decode() or "{}")
            n = len(body.get("actions") or [])
            return httpx.Response(
                200,
                json={
                    "ok": True,
                    "tick": n,
                    "applied": n,
                    "schema_version": "1.0.0",
                },
            )
        if method == "GET" and path == f"/v1/sessions/{sid}/observation":
            obs = {
                "schema_version": "1.1.0",
                "tick": 1,
                "run_id": sid,
                "message": "last_intent=noop",
                "rng_draw": 12345,
                "world": {"world_version": "1.0.0", "entities": []},
            }
            return httpx.Response(200, json=obs)
        if method == "DELETE" and path == f"/v1/sessions/{sid}":
            return httpx.Response(204)
        return httpx.Response(404, json={"error": "unexpected"})

    transport = httpx.MockTransport(handler)
    inner = httpx.Client(
        base_url="http://mock.test",
        transport=transport,
        timeout=5.0,
    )
    return AetherForgeClient("http://mock.test", client=inner)


def test_round_trip_mocked(mock_client: AetherForgeClient) -> None:
    try:
        s = mock_client.create_session(seed=99)
        assert s.session_id
        assert s.seed == 99
        ack = mock_client.apply_action(s.session_id, "noop", payload={})
        assert ack.ok and ack.tick == 1
        obs = mock_client.get_observation(s.session_id)
        assert obs.schema_version == "1.1.0"
        assert obs.tick == 1
        batch = mock_client.apply_actions(
            s.session_id,
            [
                Action(kind="a", payload={}),
                Action(kind="b", payload={}),
            ],
        )
        assert batch.ok and batch.applied == 2 and batch.tick == 2
        mock_client.delete_session(s.session_id)
    finally:
        mock_client.close()


def test_observe_stream_mock_sse_body() -> None:
    sid = "stream-session-01"
    line = json.dumps(
        {
            "schema_version": "1.1.0",
            "tick": 0,
            "run_id": sid,
            "message": "m",
            "rng_draw": 0,
            "world": {"world_version": "1.0.0", "entities": []},
        }
    )
    sse_body = f"data: {line}\n\n".encode()

    def handler(request: httpx.Request) -> httpx.Response:
        if request.method == "GET" and request.url.path == f"/v1/sessions/{sid}/observe/stream":
            return httpx.Response(
                200,
                content=sse_body,
                headers={"content-type": "text/event-stream"},
            )
        return httpx.Response(404)

    inner = httpx.Client(
        base_url="http://mock.test",
        transport=httpx.MockTransport(handler),
        timeout=5.0,
    )
    c = AetherForgeClient("http://mock.test", client=inner)
    try:
        events = list(c.observe_stream(sid))
        assert len(events) == 1
        assert events[0].tick == 0
        assert events[0].schema_version == "1.1.0"
    finally:
        c.close()
