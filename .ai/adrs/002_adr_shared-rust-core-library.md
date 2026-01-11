## ADR‑002: Shared Rust Core Library

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

Business logic (validation, serialization, authentication primitives) must be identical across server, web, desktop, and mobile clients to prevent implementation drift.

### Decision

Extract cross‑platform logic into a pure‑Rust library (`/core`) that is compilable to WebAssembly for the browser and exposes a C‑compatible FFI for integration with Flutter (Dart) and Tauri.

### Consequences

- **Positive**:
    - Maximum code reuse; write logic once, run everywhere.
    - Ensures consistent behavior across platforms.
    - WebAssembly provides near‑native performance in the browser.
- **Negative**:
    - Increased complexity in build configuration (WASM, FFI).
    - Must avoid platform‑specific dependencies in the core.
    - FFI bindings require manual maintenance for each target language.