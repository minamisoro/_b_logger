# Codebase Map

## Repository Layout

```
/
├── .ai/                          # AI context management (this directory)
│   ├── architecture.md
│   ├── api-contracts.md
│   ├── decisions.md
│   ├── codebase-map.md
│   ├── context.md
│   └── state.md
├── .github/                      # CI/CD workflows
│   └── workflows/
├── .vscode/                      # Editor settings
├── proto/                        # Protobuf definitions (single source of truth)
│   ├── auth.proto
│   ├── node.proto
│   ├── context.proto
│   ├── query.proto
│   ├── feed.proto
│   ├── utils.proto
│   └── buf.yaml
├── core/                         # Shared Rust core library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── validation/
│   │   ├── serialization/
│   │   ├── authentication/
│   │   └── models/               # Re‑exported generated Protobuf types
│   ├── Cargo.toml
│   ├── build.rs
│   └── ffi/                      # C FFI bindings
├── server/                       # Axum HTTP backend
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── api/                  # HTTP endpoint handlers
│   │   ├── database/             # PostgreSQL repository
│   │   ├── cache/                # Redis integration
│   │   ├── auth/                 # Authentication middleware
│   │   └── lib.rs
│   ├── Cargo.toml
│   └── migrations/               # SQLx migrations
├── web/                          # Vue 3 TypeScript frontend
│   ├── src/
│   │   ├── main.ts
│   │   ├── views/                # Page components
│   │   ├── components/           # Reusable UI components
│   │   ├── stores/               # Pinia stores
│   │   ├── utils/                # Generated client SDK import
│   │   └── assets/
│   ├── public/
│   ├── index.html
│   ├── vite.config.ts
│   ├── package.json
│   └── tsconfig.json
├── desktop/                      # Tauri application
│   ├── src-tauri/
│   │   ├── Cargo.toml            # Depends on core and server?
│   │   ├── src/
│   │   │   ├── main.rs           # Tauri entry
│   │   │   └── commands.rs       # Exposed Rust functions
│   │   └── tauri.conf.json
│   └── src/                      # Shared Vue frontend (symlink to ../web/src?)
├── mobile/                       # Flutter application
│   ├── lib/
│   │   ├── main.dart
│   │   ├── src/
│   │   │   ├── screens/
│   │   │   ├── widgets/
│   │   │   ├── blocs/            # Business logic (calls generated Dart client)
│   │   │   └── ffi/              # FFI bindings to Rust core
│   ├── pubspec.yaml
│   └── android/, ios/
├── packages/                     # Generated client SDKs
│   ├── typescript/
│   │   ├── src/
│   │   ├── package.json
│   │   └── tsconfig.json
│   ├── dart/
│   │   ├── lib/
│   │   └── pubspec.yaml
│   └── rust/
│       └── src/
├── scripts/                      # Build and generation scripts
│   ├── generate.sh               # Run buf generate
│   ├── build-wasm.sh             # Compile core to WebAssembly
│   └── publish-sdks.sh
├── Cargo.toml                    # Root workspace
├── Cargo.lock
├── package.json                  # pnpm workspace root
├── pnpm-workspace.yaml
├── buf.gen.yaml                  # Code generation configuration
├── .gitignore
└── README.md
```

## Module Responsibilities

### `proto/`
- **Purpose**: Define all data structures and service interfaces.
- **Key Files**:
    - `*.proto`: Protobuf definitions for each domain.
    - `buf.yaml`: Linting and breaking change detection.
- **Generated Output**:
    - Rust types in `target/generated/` (via `prost`)
    - TypeScript types in `packages/typescript/src`
    - Dart types in `packages/dart/lib`

### `core/`
- **Purpose**: Cross‑platform business logic (validation, serialization, authentication primitives).
- **Dependencies**: `prost`, `serde`, `validator`, `argon2`, `jsonwebtoken`.
- **Outputs**:
    - Rust library crate.
    - WebAssembly module (`core.wasm`) for web.
    - C dynamic library (`libcore.so`/`libcore.dylib`) for FFI.
- **Restrictions**: Must be `no_std` compatible? Actually can use `std` but avoid network/filesystem.

### `server/`
- **Purpose**: Backend service implementing HTTP endpoints with Protobuf/JSON payloads.
- **Dependencies**: `axum`, `diesel`, `redis`, `core`, `prost`.
- **Responsibilities**:
    - Serve HTTP requests on a single port.
    - Decode Protobuf requests and encode responses (with JSON support).
    - Manage database connections and migrations.
    - Authentication, authorization, rate limiting.
    - Background jobs (email sending, feed aggregation).

### `web/`
- **Purpose**: Browser‑based user interface.
- **Framework**: Vue 3 with TypeScript, Vite, Pinia, Vue Router.
- **Integration**:
    - Uses generated TypeScript HTTP client with Protobuf serialization (or JSON).
    - May load `core.wasm` for client‑side validation.
    - Real‑time updates via WebSocket/SSE.

### `desktop/`
- **Purpose**: Native desktop application using Tauri.
- **Structure**:
    - Rust backend (`src‑tauri`) that can directly call `core` and `server` HTTP client.
    - Frontend shares the same Vue code as `web/` (via symlink or package).
- **Distribution**: Standalone binaries for Windows, macOS, Linux.

### `mobile/`
- **Purpose**: Mobile application for iOS and Android.
- **Framework**: Flutter with Dart.
- **Integration**:
    - Uses generated Dart HTTP client with Protobuf serialization.
    - May call Rust core via FFI for performance‑critical operations.
- **Build**: Standard Flutter toolchain.

### `packages/`
- **Purpose**: Generated client SDKs for each target language.
- **Publishing**: npm for TypeScript, pub.dev for Dart, crates.io for Rust.
- **Versioning**: Follows semver, tied to Protobuf version.

## Development Commands

### Code Generation
```bash
# Generate all stubs
./scripts/generate.sh

# Generate Rust types only
buf generate --template buf.gen.rust.yaml

# Generate TypeScript client
buf generate --template buf.gen.typescript.yaml
```

### Building
```bash
# Build Rust workspace
cargo build --workspace

# Build core as WebAssembly
./scripts/build-wasm.sh

# Build web frontend
cd web && pnpm build

# Build Tauri app
cd desktop && pnpm tauri build

# Build Flutter app
cd mobile && flutter build apk
```

### Testing
```bash
# Run Rust tests
cargo test --workspace

# Run web unit tests
cd web && pnpm test

# Run Flutter tests
cd mobile && flutter test
```

## Dependencies Between Modules

```mermaid
graph LR
    PROTO(proto) --> CORE(core)
    PROTO --> SERVER(server)
    PROTO --> PACKAGES(packages)
    CORE --> SERVER
    CORE --> WEB(web) via WASM
    CORE --> MOBILE(mobile) via FFI
    CORE --> DESKTOP(desktop) via FFI
    PACKAGES --> WEB
    PACKAGES --> MOBILE
    PACKAGES --> DESKTOP
    SERVER --> WEB via HTTP
    SERVER --> MOBILE via HTTP
    SERVER --> DESKTOP via HTTP
```

## Navigating the Codebase

- **Start with `proto/`** to understand the data model and API.
- **Look at `core/`** for shared validation and business rules.
- **Check `server/src/api/`** for HTTP endpoint handlers.
- **Frontend logic** is in `web/src/stores/` and `mobile/lib/src/blocs/`.
- **Generated code** is in `packages/`; avoid editing it manually.

## Notes

- The `core` crate must remain platform‑agnostic; any platform‑specific code belongs in the respective client or server.
- The `proto` directory is the ultimate source of truth; changes here trigger regeneration of all dependent code.
- The AI context files (`.ai/`) should be updated whenever a significant architectural change occurs.