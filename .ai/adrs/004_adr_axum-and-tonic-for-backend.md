## ADR‑004: Axum and Tonic for Backend

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

The backend must serve gRPC requests, provide a REST gateway, and handle HTTP middleware (authentication, logging, rate limiting). The Rust ecosystem offers several web frameworks.

### Decision

Use **Axum** as the HTTP server framework and **Tonic** as the gRPC server/library. Axum’s modular design integrates well with Tonic via `tonic‑web`, and both are built on `hyper` and `tower`.

### Consequences

- **Positive**:
    - High performance and asynchronous support.
    - Rich middleware ecosystem (`tower‑http`, `tracing`).
    - Seamless REST‑gRPC bridging.
- **Negative**:
    - Relatively young ecosystem compared to Actix‑web or Rocket.
    - Must ensure compatibility between Axum and Tonic middleware stacks.