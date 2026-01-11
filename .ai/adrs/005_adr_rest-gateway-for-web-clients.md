## ADR‑005: REST Gateway for Web Clients

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

Web browsers cannot natively speak gRPC; they require HTTP/1.1 with JSON or gRPC‑Web. A REST gateway simplifies integration with existing web tooling (fetch, axios) and enables caching, CDN, and simpler debugging.

### Decision

Provide a REST gateway that translates HTTP/JSON calls into gRPC requests. Use `tonic‑web` (native integration) or `grpc‑gateway` (generated from Protobuf annotations) depending on feature requirements.

### Consequences

- **Positive**:
    - Web clients can use standard REST tooling.
    - Easier to test with `curl` or Postman.
    - CDN caching for public GET endpoints.
- **Negative**:
    - Additional layer of complexity and latency.
    - Mapping between REST and gRPC conventions may require custom configuration.