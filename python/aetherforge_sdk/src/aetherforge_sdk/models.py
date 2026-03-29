from __future__ import annotations

from typing import Any

from pydantic import BaseModel, Field


class WorldSnapshot(BaseModel):
    world_version: str = "1.0.0"
    entities: list[Any] = Field(default_factory=list)


class FarmPlot(BaseModel):
    coord: dict[str, int]
    crop: str
    growth_stage: int = 0


class FarmSnapshot(BaseModel):
    day: int = 1
    time_minutes: int = 0
    plots: list[FarmPlot] = Field(default_factory=list)
    inventory: dict[str, int] = Field(default_factory=dict)


class Observation(BaseModel):
    """Mirrors `Observation` schema_version 1.1.0 (Rust `aetherforge_sim::Observation`)."""

    schema_version: str
    tick: int
    run_id: str
    message: str
    rng_draw: int
    world: WorldSnapshot = Field(default_factory=WorldSnapshot)
    farm: FarmSnapshot | None = None


class CreateSessionResponse(BaseModel):
    session_id: str
    schema_version: str
    seed: int


class Action(BaseModel):
    schema_version: str = "1.0.0"
    kind: str
    payload: dict[str, Any] = Field(default_factory=dict)


class ActionAck(BaseModel):
    ok: bool
    tick: int
    schema_version: str


class BatchActionAck(BaseModel):
    ok: bool
    tick: int
    applied: int
    schema_version: str
