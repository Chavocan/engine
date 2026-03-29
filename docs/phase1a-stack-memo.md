# Phase 1a — Greenfield stack memo (AetherForge Engine)

## 1. Primary implementation language(s)

| Layer | Language | Rationale |
|-------|----------|-----------|
| **Simulation + platform glue** | **Rust** | Strong headless story, single Cargo graph, memory safety for long-running sims, excellent cross-platform CI, native static libs for bindings. |
| **AI / designer tooling & SDK** | **Python 3.11+** | De facto for ML/agents; fast iteration on schemas, clients, and test harnesses against the control plane. |
| **Stable FFI surface (future)** | **C ABI** (exported from Rust) | Lowest-friction bindings if we add other language clients later. |

## 2. Build and test toolchain (CI-friendly)

- **Rust:** `cargo build`, `cargo test`, `cargo clippy`, `rustfmt`; lockfile committed (`Cargo.lock`).
- **Python:** `uv` or `pip` + `venv`; `pytest` for SDK and integration tests; `ruff` for lint/format.
- **CI:** GitHub Actions (or equivalent): matrix on Windows + Linux at minimum; `cargo test --all-features` and `pytest` on PRs; optional `cargo deny` for licenses.

## 3. Allowed dependency categories (candidates)

| Category | Role | Candidates (illustrative, not final pins) |
|----------|------|-------------------------------------------|
| **Window / input (headed mode)** | OS window + events | `winit` |
| **Graphics API (headed / debug viz)** | GPU abstraction | `wgpu` (Vulkan/Metal/DX12 backends) |
| **Math** | Vectors, transforms, collision helpers | `glam` |
| **Async / networking (control plane)** | REST/WebSocket server | `tokio`, `axum`, `tokio-tungstenite` (or `axum` + SSE first) |
| **Serialization / schemas** | Versioned payloads | `serde`, `serde_json`; JSON Schema generation via `schemars` where useful |
| **Logging / tracing** | AI-observable diagnostics | `tracing`, structured JSON subscriber |
| **Audio** | Optional stub in v0 | Feature-gated stub crate or `cpal` behind `audio` feature — **UNTESTED** until a slice wires sound |

## 4. Explicit exclusions

- We **do not** embed **Godot, Unity, or Unreal** as the **runtime core** that owns simulation tick, scene state, or authoritative game rules. External editors or asset pipelines may be discussed later; they are not the engine kernel.
- No “simulation only works with editor open” or hidden GUI-only state on the critical path for headless play (**UNTESTED** until headless slice proves otherwise).

## 5. AI-native alignment

- **Headless / no-window path:** Core sim and rules live in crates that compile without `winit`/`wgpu` (e.g. `--no-default-features` or dedicated `headless` feature). Headed mode is an optional consumer of the same tick API.
- **Determinism (v0 story):** Fixed timestep scheduler; seeded RNG injected per run; document that **floating-point** and multithreaded physics remain **non-deterministic unless** a later slice replaces them with fixed models. Anything not yet measured is **UNTESTED**.
- **Control plane embeddability:** Control plane implemented as async Rust service(s) in-repo, speaking versioned JSON over HTTP/WebSocket; Python SDK talks to the same contracts. Same schemas drive human-readable logs and machine consumption.

## Stack ↔ process model (one paragraph)

A single **authoritative simulation crate** advances the world on fixed ticks; **IO adapters** (window, network, filesystem) sit at the edges and never become the source of truth for game state. That separation is what lets us run identical logic headless, under REST-driven agents, or with a future farming-sim flagship without forking the rules.

---

**UNTESTED:** Claims that headless builds, deterministic seeds, and REST/WS control match headed behavior are **not proven** until Phase 1b+ slices land with runnable artifacts and checks.
