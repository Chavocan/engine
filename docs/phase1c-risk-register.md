# Phase 1c — Risk register (top 5)

| # | Risk | Impact | Mitigation | Trigger signals |
|---|------|--------|------------|-----------------|
| 1 | **Non-deterministic sim** (floats, threads, unordered iteration) breaks AI replay and regression | High | Fixed tick + seeded RNG in kernel; document non-deterministic subsystems; add golden-tick tests where feasible | Same seed diverges across runs or OS |
| 2 | **Session concurrency bugs** (double `step`, torn reads) corrupt state or panic | High | Rule of record: one mutating runner per `session_id`; `Mutex`/actor per session; stress tests with concurrent HTTP | Crashes under parallel clients; impossible states in logs |
| 3 | **Schema drift** between Rust, JSON Schema, and Python SDK | Medium | Single source types in `aetherforge_schemas`; CI generates/validates schemas; version prefix `/v1` | Client validates but server rejects; field renames without version bump |
| 4 | **Headless / headed parity gaps** (GUI-only state) invalidates autonomous play | High | Kernel-only authority; CI runs headless tick tests; explicit **UNTESTED** until parity checklist green | Headless playthrough fails while headed succeeds |
| 5 | **Control plane scope creep** (auth, persistence, multiplayer) delays kernel quality | Medium | v0 localhost, no auth; defer persistence to ADR; timebox API surface | API surface grows without passing sim tests |

**Review:** Revisit after each phase; add risks when new subsystems land.
