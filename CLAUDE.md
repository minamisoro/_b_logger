# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a blog project (Minamisoro Blog) organized as a monorepo with the following structure:

```
blog/
├── backend/
│   ├── api/          # Rust REST API server (minamisoro-api)
│   └── macro/        # Rust procedural macros library (minamisoro-macro)
├── frontend/
│   ├── web/          # Vue 3 public-facing blog (minamisoro-web)
│   └── admin/        # Vue 3 admin panel (minamisoro-admin)
└── scripts/          # Utility scripts
```

## Development Commands

### Backend - API (Rust)
```bash
# Build the API
cd backend/api && cargo build

# Run the API
cd backend/api && cargo run

# Run tests
cd backend/api && cargo test

# Run in watch mode (requires watchexec)
cd backend/api && watchexec -e rs,toml -r cargo run

# Check code without building
cd backend/api && cargo check

# Format code
cd backend/api && cargo fmt

# Lint code
cd backend/api && cargo clippy
```

### Backend - Macro Library (Rust)
```bash
# Build the macro library
cd backend/macro && cargo build

# Run tests
cd backend/macro && cargo test

# Check code without building
cd backend/macro && cargo check

# Format code
cd backend/macro && cargo fmt

# Lint code
cd backend/macro && cargo clippy
```

### Frontend - Public Web (Vue 3)
```bash
# Install dependencies
cd frontend/web && npm install

# Run development server
cd frontend/web && npm run dev

# Build for production
cd frontend/web && npm run build

# Preview production build
cd frontend/web && npm run preview

# Run unit tests
cd frontend/web && npm run test:unit

# Type check
cd frontend/web && npm run type-check

# Lint and fix
cd frontend/web && npm run lint

# Format code
cd frontend/web && npm run format
```

### Frontend - Admin Panel (Vue 3)
```bash
# Install dependencies
cd frontend/admin && npm install

# Run development server
cd frontend/admin && npm run dev

# Build for production
cd frontend/admin && npm run build

# Preview production build
cd frontend/admin && npm run preview

# Run unit tests
cd frontend/admin && npm run test:unit

# Type check
cd frontend/admin && npm run type-check

# Lint and fix
cd frontend/admin && npm run lint

# Format code
cd frontend/admin && npm run format
```

## Architecture

### Backend - API (`minamisoro-api`)
- **Language**: Rust (Edition 2024)
- **Type**: Binary (application)
- **Entry point**: [backend/api/src/main.rs](backend/api/src/main.rs)
- **Purpose**: REST API server for the blog platform

### Backend - Macro Library (`minamisoro-macro`)
- **Language**: Rust (Edition 2024)
- **Type**: Library (proc-macro)
- **Entry point**: [backend/macro/src/lib.rs](backend/macro/src/lib.rs)
- **Purpose**: Procedural macros used by the API and shared utilities

### Frontend - Public Web (`minamisoro-web`)
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

### Frontend - Admin Panel (`minamisoro-admin`)
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
