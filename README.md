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

### Interactive Setup (Recommended)

The easiest way to get started is with the interactive setup script:

```bash
task init
```

This will guide you through:
- Database configuration (PostgreSQL credentials)
- Server and frontend port settings
- Automatic generation of config.toml and .env files
- Database creation and migrations
- Dependencies installation

### Manual Setup

If you prefer to configure manually:

1. **Copy the configuration template:**
   ```bash
   cp config.example.toml config.toml
   ```

2. **Edit config.toml with your settings:**
   - Update database URL with your PostgreSQL credentials
   - Adjust ports if needed (defaults: API=8080, web=8081, admin=8082)

3. **Generate .env files from config:**
   ```bash
   python scripts/sync-config.py
   ```
   This will create:
   - `.env` (backend with DATABASE_URL)
   - `frontend/web/.env` (web frontend settings)
   - `frontend/admin/.env` (admin frontend settings)

4. **Set up the database:**
   ```bash
   # Create database
   createdb blogger_db

   # Run migrations
   diesel migration run
   ```

5. **Install dependencies:**
   ```bash
   npm install
   ```

### Quick Start

Use **Task** commands for the easiest workflow:

```bash
# Interactive setup (first time only)
task init

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

## Docker Deployment

### Quick Deploy

```bash
# Set environment variables
export DATABASE_URL="postgresql://user:pass@host/db"
export WEBHOOK_SECRET="$(openssl rand -hex 32)"

# Build and start all services
docker compose up -d
```

### Services

- **API** (port 8080) - REST API server
- **Web** (port 8081) - Public blog
- **Admin** (port 8082) - Admin panel
- **Webhook** (port 9000) - GitHub webhook handler for auto-deployment

### GitHub Webhook Setup

1. Generate secret: `openssl rand -hex 32`
2. Set `WEBHOOK_SECRET` environment variable
3. Configure GitHub webhook:
   - URL: `https://your-domain.com:9000/webhook/github`
   - Secret: Your generated secret
   - Events: Releases
4. When you publish a release, it automatically pulls code and rebuilds containers

## Documentation

For detailed development instructions, see [CLAUDE.md](CLAUDE.md).
