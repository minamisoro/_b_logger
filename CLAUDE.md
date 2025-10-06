# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a blog project (_B_logger) organized as a monorepo using:
- **Cargo workspaces** for Rust backend projects
- **npm workspaces** for frontend projects
- **Task** (Taskfile.dev) for workflow automation

Project structure:

```
blog/
├── backend/
│   ├── api/                    # Rust REST API server (blogger-api)
│   │   ├── migrations/         # Diesel database migrations
│   │   └── src/
│   │       └── main.rs         # API entry point
│   ├── macro/                  # Rust procedural macros library (blogger-macro)
│   │   └── src/
│   │       └── lib.rs          # Macro definitions
│   └── scripts/                # Rust utility scripts
├── frontend/
│   ├── lib/                    # Shared API types and client (blogger-lib)
│   │   ├── api.ts              # Generated OpenAPI types
│   │   └── client.ts           # Type-safe API client wrapper
│   ├── web/                    # Vue 3 public-facing blog (blogger-web)
│   │   └── src/
│   │       ├── assets/         # Static assets
│   │       ├── components/     # Vue components
│   │       ├── router/         # Vue Router config
│   │       ├── stores/         # Pinia stores
│   │       ├── views/          # Page views
│   │       ├── App.vue         # Root component
│   │       └── main.ts         # Entry point
│   └── admin/                  # Vue 3 admin panel (blogger-admin)
│       └── src/
│           ├── assets/         # Static assets
│           ├── components/     # Vue components
│           ├── router/         # Vue Router config
│           ├── stores/         # Pinia stores
│           ├── views/          # Page views
│           ├── App.vue         # Root component
│           └── main.ts         # Entry point
└── scripts/                    # Python utility scripts
    └── dev.py                  # Development server runner
```

## Development Commands

**IMPORTANT**: This project uses Task (Taskfile.dev) for workflow automation. Always prefer Task commands:

### Quick Start
```bash
# Install all dependencies
task install

# Start all development servers (API + web + admin)
task dev

# Build all projects
task build
```

### Workspace Commands

All commands run from the project root using workspace features:

**Frontend** (npm workspaces):
```bash
# Run tests across all frontend projects
npm run test

# Type check all frontend projects
npm run type-check

# Lint all frontend projects
npm run lint

# Format all frontend projects
npm run format

# Build all frontend projects
npm run build
```

**Backend** (Cargo workspaces):
```bash
# Run all backend tests
cargo test --workspace

# Format all Rust code
cargo fmt --workspace

# Lint all Rust code
cargo clippy --workspace

# Build all backend projects
cargo build --workspace

# Run specific binary
cargo run --bin blogger-api
```

## Architecture

### Backend - API (`blogger-api`)
- **Language**: Rust (Edition 2024)
- **Type**: Binary (application)
- **Entry point**: [backend/api/src/main.rs](backend/api/src/main.rs)
- **Purpose**: REST API server for the blog platform

### Backend - Macro Library (`blogger-macro`)
- **Language**: Rust (Edition 2024)
- **Type**: Library (proc-macro)
- **Entry point**: [backend/macro/src/lib.rs](backend/macro/src/lib.rs)
- **Purpose**: Procedural macros used by the API and shared utilities

### Frontend - Public Web (`blogger-web`)
- **Framework**: Vue 3 with Composition API
- **Language**: TypeScript
- **Build tool**: Vite
- **State management**: Pinia
- **Router**: Vue Router
- **Testing**: Vitest + Vue Test Utils
- **Linting**: ESLint with Prettier
- **Node version**: ^20.19.0 || >=22.12.0
- **Entry point**: [frontend/web/src/main.ts](frontend/web/src/main.ts)
- **Path alias**: `@` resolves to `src/`
- **Purpose**: Public-facing blog website for readers

### Frontend - Admin Panel (`blogger-admin`)
- **Framework**: Vue 3 with Composition API
- **Language**: TypeScript
- **Build tool**: Vite
- **State management**: Pinia
- **Router**: Vue Router
- **Testing**: Vitest + Vue Test Utils
- **Linting**: ESLint with Prettier
- **Node version**: ^20.19.0 || >=22.12.0
- **Entry point**: [frontend/admin/src/main.ts](frontend/admin/src/main.ts)
- **Path alias**: `@` resolves to `src/`
- **Purpose**: Admin interface for content management and blog editing
