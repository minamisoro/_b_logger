## ADR‑003: Monorepo with Workspaces

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

The project consists of multiple interconnected components (backend, core, web, desktop, mobile) that need to be developed, tested, and released together.

### Decision

Adopt a monorepo structure with language‑specific workspaces:
- Rust workspace for `core`, `server`, `proto‑build`
- pnpm workspace for `web` and shared TypeScript packages
- Flutter project placed under `mobile/`
- Tauri project under `desktop/`

### Consequences

- **Positive**:
    - Single CI/CD pipeline for the entire system.
    - Easy cross‑component refactoring and code sharing.
    - Unified versioning and dependency management.
- **Negative**:
    - Larger repository size.
    - Build times may increase without careful caching.
    - Requires tooling that understands multiple language ecosystems.