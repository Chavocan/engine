from __future__ import annotations

from collections.abc import Iterator
from typing import Any

import httpx

from aetherforge_sdk.models import (
    Action,
    ActionAck,
    BatchActionAck,
    CreateSessionResponse,
    Observation,
)


class AetherForgeClient:
    """Sync HTTP client for `/v1` control plane (httpx)."""

    def __init__(
        self,
        base_url: str,
        *,
        timeout: float = 30.0,
        client: httpx.Client | None = None,
    ) -> None:
        self._base = base_url.rstrip("/")
        self._client = client or httpx.Client(base_url=self._base, timeout=timeout)

    def close(self) -> None:
        self._client.close()

    def __enter__(self) -> AetherForgeClient:
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()

    def create_session(self, seed: int | None = None) -> CreateSessionResponse:
        body: dict[str, Any] = {}
        if seed is not None:
            body["seed"] = seed
        r = self._client.post("/v1/sessions", json=body)
        r.raise_for_status()
        return CreateSessionResponse.model_validate_json(r.content)

    def apply_action(
        self,
        session_id: str,
        kind: str,
        *,
        payload: dict[str, Any] | None = None,
        schema_version: str = "1.0.0",
    ) -> ActionAck:
        action = Action(schema_version=schema_version, kind=kind, payload=payload or {})
        r = self._client.post(
            f"/v1/sessions/{session_id}/action",
            json=action.model_dump(),
        )
        r.raise_for_status()
        return ActionAck.model_validate_json(r.content)

    def apply_actions(self, session_id: str, actions: list[Action]) -> BatchActionAck:
        r = self._client.post(
            f"/v1/sessions/{session_id}/actions",
            json={"actions": [a.model_dump() for a in actions]},
        )
        r.raise_for_status()
        return BatchActionAck.model_validate_json(r.content)

    def get_observation(self, session_id: str) -> Observation:
        r = self._client.get(f"/v1/sessions/{session_id}/observation")
        r.raise_for_status()
        return Observation.model_validate_json(r.content)

    def observe_stream(self, session_id: str) -> Iterator[Observation]:
        """Yield observations from SSE ``GET .../observe/stream``.

        Requires the server binary built with Rust feature ``sse-obs``.
        """
        url = f"/v1/sessions/{session_id}/observe/stream"
        with self._client.stream(
            "GET",
            url,
            headers={"Accept": "text/event-stream"},
        ) as response:
            response.raise_for_status()
            buf = ""
            for chunk in response.iter_text():
                buf += chunk
                while "\n\n" in buf:
                    frame, _, buf = buf.partition("\n\n")
                    for line in frame.splitlines():
                        stripped = line.strip()
                        if stripped.startswith("data:"):
                            payload = stripped[5:].strip()
                            if payload:
                                yield Observation.model_validate_json(payload)

    def delete_session(self, session_id: str) -> None:
        r = self._client.delete(f"/v1/sessions/{session_id}")
        r.raise_for_status()
        if r.status_code != 204:
            raise RuntimeError(f"expected 204, got {r.status_code}")
