# Minamisoro Blog

A modern blog platform built as a monorepo, featuring a Rust backend API and Vue 3 frontend applications.

Big portion of this project uses AI. I am trying to be open minded about the effectiveness of the AI, trying to see how far it can go by using its tools extensively and carefully vetting its output for correctness.

## Project Structure

```
blog/
├── backend/
│   ├── api/          # REST API server
│   └── macro/        # Procedural macros library
├── frontend/
│   ├── web/          # Public-facing blog website
│   └── admin/        # Admin panel for content management
└── scripts/          # Utility scripts
```

## Requirements

### Backend
- **Rust**: Latest stable version (Edition 2024)
- **Cargo**: Comes with Rust installation

### Frontend
- **Node.js**: ^20.19.0 or >=22.12.0
- **npm**: Latest version (comes with Node.js)

### Development
- **Python**: 3.6+ (for running the dev script)

### Optional Tools
- **watchexec**: For Rust hot-reloading during development
  ```bash
  cargo install watchexec-cli
  ```
- **Task**: For running development tasks
  ```bash
  # See https://taskfile.dev/installation/
  ```

## Getting Started

### Quick Start (All Servers)

```bash
# Install frontend dependencies
task install

# Run all development servers concurrently with color-coded output
task dev
# Or directly: python scripts/dev.py
```

The dev script will:
- Run API, web, and admin servers concurrently
- Color-code output by process (blue=API, green=web, yellow=admin)
- Gracefully shut down all processes on Ctrl+C
- Automatically stop all processes if any one fails

### Backend Development

```bash
# Run the API server
cd backend/api
cargo run

# Run in watch mode with watchexec
watchexec -e rs,toml -r cargo run
```

### Frontend Development

```bash
# Public website
cd frontend/web
npm install
npm run dev

# Admin panel
cd frontend/admin
npm install
npm run dev
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
