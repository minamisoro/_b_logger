# Project Context

## Development Conventions

### Naming Patterns

- **Protobuf**: Use `snake_case` for message and field names, `PascalCase` for enum values.
- **Rust**: Follow `snake_case` for variables and functions, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- **TypeScript**: `camelCase` for variables and functions, `PascalCase` for classes and interfaces, `UPPER_SNAKE_CASE` for constants.
- **Dart**: `camelCase` for variables and functions, `PascalCase` for classes, `lowercase_with_underscores` for library names.

### File Organization

- **Rust modules**: One file per logical module; group related functionality under a directory with a `mod.rs`.
- **Vue components**: Use `PascalCase` for component file names, e.g., `NodeCard.vue`. Place components in `src/components/`.
- **Flutter widgets**: Each widget in its own file under `lib/src/widgets/`.

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, missing semicolons)
- `refactor`: Code restructuring without changing behavior
- `test`: Adding or updating tests
- `chore`: Maintenance tasks, tooling, dependencies

Scope examples: `proto`, `core`, `server`, `web`, `desktop`, `mobile`, `ci`.

### Branch Strategy

- `main` – production‑ready code.
- `develop` – integration branch for features.
- Feature branches: `feat/<short-description>`
- Hotfix branches: `hotfix/<short-description>`

## Development Workflow

### 1. Start Working

```bash
# Clone repository
git clone <url>
cd _b_logger

# Install dependencies
pnpm install          # Web frontend
cargo fetch           # Rust crates
cd mobile && flutter pub get  # Flutter packages
```

### 2. Make Changes

- Update Protobuf definitions first if the change affects API.
- Regenerate client SDKs (`./scripts/generate.sh`).
- Implement server‑side changes in `server/`.
- Update shared core if business logic changes.
- Update frontend(s) accordingly.

### 3. Testing

- Run unit tests for each component:
    ```bash
    cargo test --workspace
    cd web && pnpm test:unit
    cd mobile && flutter test
    ```
- Integration tests are defined in `server/tests/` (using `wiremock` for external services).

### 4. Code Review

- All changes require a pull request.
- At least one reviewer must approve.
- CI must pass (linting, tests, builds).

### 5. Merge and Deploy

- Merge to `develop` for staging deployment.
- Merge to `main` for production (tagged release).

## Tooling

### Required Global Tools

- **Rust**: `rustc`, `cargo`, `rustup` (stable toolchain)
- **Node.js**: `node` (v18+), `pnpm` (preferred package manager)
- **Dart/Flutter**: `flutter` SDK with Dart
- **Protobuf**: `protoc` compiler, `buf` CLI
- **Database**: PostgreSQL 14+, Redis 6+

### Editor Configuration

- **VS Code** settings are in `.vscode/settings.json`.
- Recommended extensions:
    - `rust‑analyzer`
    - `Volar` (Vue)
    - `Dart` and `Flutter`
    - `buf` (for Protobuf)

### Linting and Formatting

- **Rust**: `cargo fmt` and `cargo clippy`
- **TypeScript**: `eslint` and `prettier` (config in `web/`)
- **Dart**: `dart format` and `dart analyze`

## Environment Variables

Configuration is managed via `.env` files (not committed). See `.env.example` for required variables.

### Server Environment

```bash
DATABASE_URL=postgresql://user:pass@localhost/blogger
REDIS_URL=redis://localhost:6379
JWT_SECRET=...
HTTP_PORT=8080
```

### Web Environment

```bash
VITE_API_BASE_URL=http://localhost:8080
VITE_WS_URL=ws://localhost:8080/ws
```

## Common Tasks

### Adding a New Protobuf Message

1. Edit the appropriate `.proto` file in `proto/`.
2. Run `buf generate` to regenerate stubs.
3. Update the shared core validation if needed.
4. Update server implementation.
5. Update client SDK usage in frontends.

### Adding a New Context Type

1. Define the context schema in `proto/context.proto`.
2. Add validation in `core/src/validation/context.rs`.
3. Implement creation/attachment logic in `server/src/api/node_service.rs`.
4. Update web UI to render the new context.

### Running the Full Stack Locally

```bash
# Start dependencies (PostgreSQL, Redis)
docker-compose up -d

# Start backend
cd server && cargo run

# Start web frontend (dev server)
cd web && pnpm dev

# Start Tauri dev (optional)
cd desktop && pnpm tauri dev

# Start Flutter emulator (optional)
cd mobile && flutter run
```

## Troubleshooting

### "Generated code not found"
Run `./scripts/generate.sh` to regenerate all stubs.

### "Cannot connect to database"
Ensure PostgreSQL is running and `DATABASE_URL` is correct. Run migrations with `diesel migration run`.

### "WebAssembly module fails to load"
Check that `core` was compiled with the `wasm32‑unknown‑unknown` target and that the wasm file is served with correct MIME type.

### "FFI symbols not found"
Verify that `core` was built as a `cdylib` and the library path is included in `LD_LIBRARY_PATH` (or equivalent).

## Cross‑Platform Notes

- **Web**: Avoid synchronous blocking calls; use async/await.
- **Mobile**: Network calls may be slower; implement offline caching.
- **Desktop**: Tauri allows direct filesystem access; use responsibly.

## AI Context Usage

The `.ai/` directory is intended to be read by AI assistants (like Claude) to understand the project’s architecture, decisions, and state. Keep these documents up‑to‑date when making significant changes.

- **`architecture.md`** – High‑level design and component interactions.
- **`api-contracts.md`** – API specification and generation details.
- **`decisions.md`** – Rationale behind architectural choices.
- **`codebase-map.md`** – Directory structure and module responsibilities.
- **`context.md`** – This file: conventions and workflows.
- **`state.md`** – Current implementation status and next steps.