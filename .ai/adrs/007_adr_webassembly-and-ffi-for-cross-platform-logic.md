## ADR‑007: WebAssembly and FFI for Cross‑Platform Logic

**Date**: 2026‑01‑09  
**Status**: Accepted

### Context

The shared Rust core must run in the browser (WebAssembly) and be callable from Dart (Flutter) and native desktop (Tauri).

### Decision

Compile the core to **WebAssembly** for the web frontend, and expose a **C‑compatible FFI** for Dart (via `dart:ffi`) and Tauri (via `#[no_mangle]` extern functions). Use `wasm‑bindgen` for JavaScript/TypeScript interoperability.

### Consequences

- **Positive**:
    - Write once, run everywhere with near‑native performance.
    - Strong type safety across language boundaries.
    - Leverages Rust’s zero‑cost abstractions.
- **Negative**:
    - Increased build complexity (WASM target, FFI bindings).
    - Debugging across language boundaries is challenging.
    - WASM binary size must be kept small for web performance.