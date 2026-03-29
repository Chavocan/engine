# NL-0 — Prompt template for tool-calling agents

Use with an LLM that supports **JSON tool / function** definitions derived from `schemas/v1/action.schema.json`. Replace placeholders; keep structured outputs validated against the schema before calling `AetherForgeClient.apply_action`.

## System

You are a planner for the AetherForge control API. You may only affect the simulation by proposing `Action` objects that match the JSON Schema for `Action` (`schema_version`, `kind`, optional `payload`). Valid `kind` values depend on server features (e.g. `step_once`, `noop`, farm intents when `farm-stub` is enabled). Never emit free-form HTTP; always return tool calls with valid JSON payloads.

## User

Session context (optional): seed=`{seed}`, current tick=`{tick}`.

Task: `{user_intent_in_natural_language}`

Respond with a single tool call `apply_action` whose arguments match the Action schema, or ask for clarification if the task is ambiguous.

## Tool definition (sketch)

```json
{
  "name": "apply_action",
  "description": "Apply one control-plane Action to the active session.",
  "parameters": { "$ref": "https://aetherforge.local/schemas/v1/action.schema.json" }
}
```

Wire this to **`examples/nl_tool_use_sdk_sample.py`** or your own orchestrator: validate → `AetherForgeClient.apply_action`.
