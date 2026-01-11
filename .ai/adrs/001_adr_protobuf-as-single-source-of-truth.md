## ADR‑001: Protobuf as Single Source of Truth

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

We need a contract‑first API design that guarantees type safety and consistency across multiple client languages (Rust, TypeScript, Dart). The contract must be easy to version, generate code from, and support both gRPC and REST transport.

### Decision

Use Protocol Buffers (`.proto` files) as the single source of truth for all API data structures and service definitions. Generate server stubs, client SDKs, and OpenAPI specifications from the same Protobuf definitions.

### Consequences

- **Positive**:
    - Strong typing across all generated code.
    - Automatic serialization/deserialization.
    - Built‑in backward/forward compatibility.
    - Rich tooling ecosystem (`buf`, `protoc`, `tonic`, `grpc‑web`).
- **Negative**:
    - Additional build‑time step for code generation.
    - Learning curve for developers unfamiliar with Protobuf/gRPC.
    - REST mapping requires a gateway layer.