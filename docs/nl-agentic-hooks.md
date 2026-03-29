# Natural language & agentic hooks (design stub)

**Status:** design only — no production NL endpoint in-tree yet. The engine’s **contract** today is **structured** HTTP + JSON (`docs/phase1c-ai-interface-spec-v0.md`).

## Goal

Allow **external** agents (LLMs, orchestrators) to drive the same simulation **without** hand-authoring raw JSON for every intent, while keeping the **kernel** free of vendor-specific NLP.

## Recommended layering

1. **Kernel (Rust):** unchanged — `Intent { kind, ... }`, versioned schemas, deterministic stepping.
2. **Control plane:** optional future route, e.g. `POST /v1/sessions/{id}/interpret`, body `{ "text": "plant a crop on the next free tile" }`, returns **proposed** `Action` or `Action[]` + confidence; **separate** confirmation step `POST .../action` if safety demands (two-phase commit for mutating sim).
3. **Sidecar service (preferred for v1):** Python or Node **microservice** that calls an LLM with a **tool schema** matching `schemas/v1/*.json`, then calls existing **`AetherForgeClient`** — no Rust dependency on model APIs.
4. **CLI:** `aetherforge_nl` (future) — stdin lines → HTTP — same as sidecar but for demos.

## Guardrails

- **Never** pass raw model output straight to `apply_intent` without validation against schema.
- **Log** interpreted intents to play-log (`AETHERFORGE_PLAY_LOG=1`) with redaction for PII in user text.
- **Quota:** reuse `AETHERFORGE_MAX_ACTIONS_PER_SESSION` and per-IP limits when NL maps to many actions.

## Milestones

| Phase | Deliverable |
|-------|-------------|
| NL-0 | This document + **[examples/nl-prompt-template.md](../examples/nl-prompt-template.md)** |
| NL-1 | **[examples/nl_tool_use_sdk_sample.py](../examples/nl_tool_use_sdk_sample.py)** — validated `Action` → `AetherForgeClient.apply_action` (swap in real tool-use output from your model provider). |
| NL-2 | Optional **`nl-interpret-stub`** feature: **`POST /v1/sessions/{id}/interpret`** returns **501** + stable JSON (`NL_INTERPRET_NOT_IMPLEMENTED`) until a real interpreter ships; sidecar path remains preferred. |

See also **[docs/platform-headed-roadmap.md](platform-headed-roadmap.md)** for the headed runtime track (separate from NL).
