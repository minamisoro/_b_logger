## ADR‑008: REST with Protobuf Payloads

**Date**: 2026‑01‑11
**Status**: Accepted

### Context

After further research, we decided to simplify the API transport layer by using regular REST HTTP endpoints instead of gRPC, while still using Protobuf for request/response serialization. This reduces complexity (no need for gRPC server, gateway, or gRPC‑Web) while maintaining strong typing and cross‑platform compatibility.

### Decision

Use RESTful HTTP endpoints with Protobuf payloads (content‑type `application/protobuf`) as the primary transport, with optional JSON support via content negotiation. Generate Rust types, TypeScript/Dart interfaces, and HTTP client helpers from the same Protobuf definitions, but omit gRPC server stubs.

### Consequences

- **Positive**:
    - Simpler architecture: no gRPC server, no gateway translation.
    - Web clients can use standard `fetch` with Protobuf or JSON.
    - Easier debugging with `curl` and human‑readable JSON.
    - Reduced dependency on gRPC‑specific tooling.
- **Negative**:
    - Loss of bidirectional streaming (gRPC streams) — but we can use WebSocket/SSE for real‑time features.
    - Manual mapping of Protobuf service methods to REST endpoints (can be automated with code generation).
    - Less efficient than gRPC for large‑scale streaming scenarios.