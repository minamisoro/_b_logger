# System Architecture

## Overview

_b_logger is a full-stack social media application centered around a network of user-defined nodes with composable contexts. The architecture is designed to be modular, platform-agnostic, and maximize code reuse across web (Vue 3), desktop (Tauri), and mobile (Flutter) clients.

## Core Principles

1. **Single Source of Truth**: Protobuf definitions define all data structures and service contracts.
2. **Cross-Platform Logic Sharing**: Shared Rust core library compilable to WebAssembly and exposing C FFI for mobile/desktop.
3. **API‑First Design**: REST API with Protobuf payloads as the primary transport; clients can send/receive binary Protobuf or JSON (via content negotiation).
4. **Monorepo Workspaces**: Unified repository with Cargo, pnpm, and Flutter workspaces for simplified development and CI/CD.
5. **AI Context Retention**: All architectural decisions, API contracts, and project state are documented in the `.ai` directory for AI‑assisted development.

## High‑Level Architecture Diagram

```mermaid
graph TB
    subgraph "Client Platforms"
        WEB[Web Vue 3]
        DESK[Desktop Tauri]
        MOB[Mobile Flutter]
    end

    subgraph "API Layer"
        HTTP[HTTP Server (Axum)]
    end

    subgraph "Business Logic"
        CORE[Shared Rust Core]
        SRV[Server Application]
    end

    subgraph "Data Layer"
        DB[(PostgreSQL)]
        REDIS[(Redis Cache)]
    end

    subgraph "Code Generation"
        PROTO[Protobuf Definitions]
        GEN[Client SDK Generation]
    end

    WEB --> HTTP
    DESK --> HTTP
    MOB --> HTTP
    HTTP --> SRV
    SRV --> CORE
    SRV --> DB
    SRV --> REDIS
    PROTO --> GEN
    GEN --> WEB
    GEN --> DESK
    GEN --> MOB
    CORE --> WEB
    CORE --> DESK
    CORE --> MOB
```

## Component Descriptions

### Protobuf Definitions (`/proto`)
- All data structures (Node, Context, User, etc.) and service interfaces.
- Single source of truth for API contracts.
- Processed by `buf` and `prost` to generate Rust, TypeScript, Dart code.

### Shared Rust Core (`/core`)
- Contains cross‑platform business logic: validation, serialization, authentication primitives.
- Compiled to WebAssembly for browser execution.
- Exposes a C‑compatible FFI for integration with Flutter (via `dart:ffi`) and Tauri.
- No dependencies on server‑specific crates (database, HTTP).

### Server Application (`/server`)
- Axum‑based HTTP server that decodes Protobuf requests and encodes Protobuf responses (with optional JSON support via content negotiation).
- Implements REST endpoints corresponding to the Protobuf service definitions.
- Handles persistence, caching, background jobs, and real‑time notifications.
- Depends on the shared core for validation and business rules.

### Web Frontend (`/web`)
- Vue 3 with TypeScript, Vite build tool.
- Uses generated TypeScript client SDK for HTTP calls with Protobuf serialization (or JSON).
- Can import the WebAssembly‑compiled core for client‑side validation.

### Desktop App (`/desktop`)
- Tauri application with Rust backend and Vue frontend.
- Shares the same Vue codebase as the web frontend.
- Communicates with the server via HTTP with Protobuf payloads (or uses the shared core directly).

### Mobile App (`/mobile`)
- Flutter application written in Dart.
- Uses generated Dart HTTP client with Protobuf serialization.
- May call into the shared core via FFI (if performance‑critical logic is required).

### Client SDK Generation (`/packages`)
- Automated generation of TypeScript, Dart, and other language HTTP clients from Protobuf (including serialization helpers).
- Published as npm and Dart packages for consumption.

## Data Storage

- **Primary Database**: PostgreSQL with JSONB columns for flexible context storage.
- **Caching**: Redis for session storage, rate‑limiting, and feed caching.
- **File Storage**: Object storage (S3‑compatible) for uploaded media.

## Cross‑Platform Considerations

### Type Safety
- All generated clients guarantee type safety across platforms.
- Runtime validation is performed by the shared core, ensuring consistency.

### Authentication
- OAuth2, JWT, and session‑based authentication supported.
- Token refresh and device management handled by the server.

### Real‑Time Features
- WebSocket connections for live updates (node creation, context attachments).
- Server‑Sent Events (SSE) as a fallback.

## Deployment

- Backend services containerized with Docker.
- Kubernetes or cloud‑native orchestration for scaling.
- Frontend assets served via CDN.
- Desktop and mobile apps distributed through respective stores.

## Development Workflow

1. Update Protobuf definitions.
2. Regenerate client SDKs and server stubs.
3. Implement/update business logic in the shared core.
4. Update server and client implementations.
5. Run tests across all platforms.
6. Update AI context documentation.

## Next Steps

See `state.md` for current implementation status and upcoming tasks.