# _B_logger

A modern blog platform built as a monorepo, featuring a Rust backend API and Vue 3 frontend applications.

Big portion of this project uses AI. I am trying to be open minded about the effectiveness of the AI, trying to see how far it can go by using its tools extensively and carefully vetting its output for correctness.

## Project Structure

This monorepo uses **Cargo workspaces** for Rust projects and **npm workspaces** for frontend projects.

```
blog/
├── backend/
│   ├── api/                    # REST API server
│   │   ├── migrations/         # Database migrations
│   │   └── src/
│   │       └── main.rs         # API entry point
│   ├── macro/                  # Procedural macros library
│   │   └── src/
│   │       └── lib.rs          # Macro definitions
│   └── scripts/                # Rust utility scripts
├── frontend/
│   ├── lib/                    # Shared API types and client
│   │   ├── api.ts              # Generated OpenAPI types
│   │   └── client.ts           # Type-safe API client wrapper
│   ├── web/                    # Public-facing blog website
│   │   └── src/
│   │       ├── assets/         # Static assets
│   │       ├── components/     # Vue components
│   │       ├── router/         # Vue Router config
│   │       ├── stores/         # Pinia stores
│   │       ├── views/          # Page views
│   │       ├── App.vue         # Root component
│   │       └── main.ts         # Entry point
│   └── admin/                  # Admin panel for content management
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

## Requirements

### Backend
- **Rust**: Latest stable version (Edition 2024)
- **Cargo**: Comes with Rust installation
- **PostgreSQL**: 18+ (for UUIDv7 support)
- **Diesel CLI**: For database migrations
  ```bash
  cargo install diesel_cli --no-default-features --features postgres
  ```

### Frontend
- **Node.js**: ^20.19.0 or >=22.12.0
- **npm**: Latest version (comes with Node.js)

### Development Tools
- **Task**: For running development workflows (recommended)
  ```bash
  # See https://taskfile.dev/installation/
  ```
- **Python**: 3.6+ (for the dev script)
- **watchexec**: For Rust hot-reloading (required for `task dev`)
  ```bash
  cargo install watchexec-cli
  ```

## Getting Started

### Initial Setup

1. **Copy the configuration template:**
   ```bash
   cp config.toml.example config.toml
   ```

2. **Set up environment variables:**
   ```bash
   # Create .env file for backend (in project root)
   echo "DATABASE_URL=postgres://user:password@localhost/blogger_db" > .env

   # Sync frontend .env files from config.toml
   python scripts/sync-config.py
   ```

3. **Set up the database:**
   ```bash
   # Create database
   createdb blogger_db

   # Run migrations
   diesel migration run --database-url postgres://user:password@localhost/blogger_db
   ```

### Quick Start

Use **Task** commands for the easiest workflow:

```bash
# Install all dependencies (uses npm workspaces)
task install

# Start all development servers (API + web + admin)
task dev

# Build all projects
task build
```

The `task dev` command will:
- Run API, web, and admin servers concurrently
- Color-code output by process (blue=API, green=frontend)
- Gracefully shut down all processes on Ctrl+C
- Automatically stop all processes if any one fails

### Alternative: Manual Commands

If you prefer not to use Task:

**Install:**
```bash
npm install
```

**Development:**
```bash
# Backend (from project root)
cargo run --bin blogger-api

# Frontend (from project root)
npm run dev

# Or run the dev script directly
python scripts/dev.py
```

**Testing:**
```bash
# Backend
cargo test --workspace

# Frontend
npm run test
```

**Building:**
```bash
# Backend
cargo build --workspace --release

# Frontend
npm run build
```

## Tech Stack

### Backend
- **Rust** (Edition 2024)
- REST API architecture

### Frontend
- **Vue 3** with Composition API
- **TypeScript** for type safety
- **Vite** as build tool
- **Pinia** for state management
- **Vue Router** for routing
- **Vitest** for unit testing
- **ESLint + Prettier** for code quality

## Documentation

For detailed development instructions, see [CLAUDE.md](CLAUDE.md).
