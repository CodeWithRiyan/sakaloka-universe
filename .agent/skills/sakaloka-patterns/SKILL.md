---
name: sakaloka-universe-patterns
description: Coding patterns extracted from sakaloka-universe repository
version: 1.0.0
source: local-git-analysis
analyzed_commits: 17
---

# Sakaloka-Universe Patterns

## Commit Conventions

This project uses **conventional commits** consistently:
- `feat:` — New features (most common, ~60% of commits)
- `fix:` — Bug fixes
- `refactor:` — Code restructuring without behavior changes
- `chore:` — Maintenance tasks (deps, build artifacts, renames)

Commit messages are descriptive sentences, not shorthand:
```
feat: Implement database-backed authentication with refresh token rotation and session management.
refactor: Separate product routes to apply scope layers individually for each HTTP method.
```

## Code Architecture

### Planet Model (Celestial Body Naming)

Each service maps to a celestial body:

```
apps/
├── api/          # Earth — Loco.rs REST API (primary entry point)
├── ockam/        # Mars — Ockam secure transport broker
├── venus/        # Venus — Tauri desktop app (src-tauri/ tracked here)
└── firmware/     # Mercury — ARM Cortex-M4 bare-metal (excluded from workspace)

libs/
├── core/         # Domain types, traits, constants (no business logic)
├── secure/       # Single source of truth for IAM: JWT, RBAC, Argon2, newtypes
├── data/         # Database clients: SurrealDB, Qdrant, Zenoh
├── ai/           # Burn inference engine (Zenoh-only, no HTTP)
└── hal/          # Hardware abstraction layer (no_std)
```

### Crate Responsibility Rules

- **Auth/JWT/RBAC/passwords** → always `libs/secure` (never elsewhere)
- **Domain types/constants** → `libs/core`
- **Database/messaging/vector** → `libs/data`
- **HTTP routes/controllers** → `apps/api`
- **AI/embeddings** → `libs/ai`
- **Hardware/sensors** → `libs/hal`

## Workflows

### Adding a New API Endpoint (Loco Pattern)

Files always change together in this order:
1. `apps/api/migration/src/m*_create_{entity}.rs` — SeaORM migration
2. `apps/api/migration/src/lib.rs` — Register migration
3. `apps/api/src/models/_entities/{entity}.rs` — SeaORM entity
4. `apps/api/src/models/_entities/mod.rs` — Register entity
5. `apps/api/src/models/{entity}.rs` — Model wrapper (re-exports entity)
6. `apps/api/src/models/mod.rs` — Register model
7. `apps/api/src/views/{entity}.rs` — Response/request schemas (utoipa)
8. `apps/api/src/views/mod.rs` — Register view
9. `apps/api/src/controllers/{entity}.rs` — Route handlers
10. `apps/api/src/controllers/mod.rs` — Register controller
11. `apps/api/src/app.rs` — Register routes

### Adding a New Library Crate

1. Create `libs/{name}/Cargo.toml` and `libs/{name}/src/lib.rs`
2. Add to `Cargo.toml` workspace members
3. Add to `[workspace.dependencies]` as `sakaloka-{name} = { path = "libs/{name}" }`
4. Every public item must have `///` docs with `# Errors` and `# Examples` sections

### Authentication Flow

Auth changes always touch these files together:
- `libs/secure/src/jwt/` — Token issuance/validation
- `libs/secure/src/rbac/` — Role/scope definitions
- `apps/api/src/middleware/auth.rs` — Extract + validate Bearer token
- `apps/api/src/controllers/auth.rs` — Login/register handlers

### Infrastructure Changes

Docker/infra changes follow the pattern:
1. `docker-compose.yml` — Add/modify service
2. `.env.example` — Add new environment variables
3. `.github/workflows/ci.yml` — Add CI service container if needed
4. `apps/api/config/*.yaml` — Update Loco config for new service

## Iron Curtain Standards

These are enforced by CI and must never be violated:

### Forbidden Outside `#[test]`
- `.unwrap()`, `.expect()`, `panic!()`, `unsafe`
- Use `Result<T, E>` with `thiserror` in libraries, `anyhow` in binaries

### Required
- 100% doc comments on public items (`///` with `# Errors`, `# Examples`)
- Zero clippy warnings (`cargo clippy -- -D warnings`)
- Consistent formatting (`cargo fmt`)
- Newtype pattern at boundaries (no raw strings in business logic)

### RBAC Pattern
```rust
router.route("/{id}", put(handler).route_layer(RequireScope::new(Scope::EntityWrite)))
```

### Database Queries
Always parameterized — never string interpolation:
```rust
db.query("SELECT * FROM user WHERE username = $username LIMIT 1")
  .bind(("username", username))
```

## Testing Patterns

- Tests require `SAKALOKA_JWT_SECRET` env var (min 32 chars)
- Tests require `DATABASE_URL` for Postgres
- Test config at `apps/api/config/test.yaml` uses ephemeral port (0)
- Test DB has `auto_migrate: true`, `dangerously_truncate: true`, `dangerously_recreate: true`
- Doc-tests serve as both documentation and compile-time verification
- `serial_test` crate used for tests that share database state

## Port Allocation

All external ports use `5xxxx` scheme:

| Port | Service |
|------|---------|
| 53000 | API (Earth) |
| 54000 | Ockam (Mars) |
| 55432 | Postgres |
| 56333 | Qdrant (Uranus) |
| 57447 | Zenoh (Saturn) |
| 58000 | SurrealDB (Jupiter) |

## Two-Repo Workflow

- `sakaloka-universe` — Rust monorepo + Tauri shell (`apps/venus/src-tauri/`)
- `sakapos` (CodeWithRiyan/sakapos) — React frontend
- `apps/venus/` has its own `.git` pointing to sakapos
- Monorepo `.gitignore` ignores all frontend files except `src-tauri/` and `CLAUDE.md`

## Co-change Patterns (Files That Change Together)

High-correlation file groups detected from git history:

1. **API core**: `Cargo.toml` + `Cargo.lock` + `apps/api/Cargo.toml` (always together)
2. **Auth stack**: `libs/secure/src/argon2.rs` + `libs/secure/src/jwt/*` + `libs/secure/src/tokens/*` + `apps/api/src/auth.rs`
3. **Data layer**: `libs/data/src/surreal.rs` + `libs/data/Cargo.toml` + `libs/core/src/models/*`
4. **CI/Infra**: `.github/workflows/ci.yml` + `docker-compose.yml` + `.env.example`
5. **API endpoints**: `controllers/*.rs` + `models/*.rs` + `views/*.rs` (MVC triad)
