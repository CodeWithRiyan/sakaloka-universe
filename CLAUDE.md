# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Build
cargo build --workspace                # Debug build (excludes firmware)
cargo build --release --workspace      # Release build

# Lint & Format
cargo fmt --all -- --check             # Check formatting
cargo clippy --workspace --all-targets -- -D warnings  # Lint (deny all warnings)

# Tests
cargo test --workspace                 # All tests (needs SAKALOKA_JWT_SECRET env var)
cargo test -p sakaloka-secure          # Single crate
cargo test -p sakaloka-api -- health   # Single test by name filter
cargo test --lib                       # Unit tests only
cargo test --doc                       # Doc-tests only

# Documentation (CI enforces 100% coverage)
RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps

# Firmware (cross-compilation, excluded from workspace)
cargo build -p sakaloka-firmware --target thumbv7em-none-eabihf

# Docker
docker compose up -d                   # Start all services
docker compose up -d surrealdb qdrant zenoh  # Start infra only
```

## Environment Setup

Copy `.env.example` to `.env`. Required for tests: `SAKALOKA_JWT_SECRET` (min 32 chars). SurrealDB migrations live in `libs/data/surql/` and must be applied in order.

## Architecture

This is a Rust workspace ("planet model") where each service maps to a celestial body:

**Apps:**
- `apps/api` (Earth) — Axum 0.8 REST API. Entry point for all client requests. Routes: `/health`, `/auth/*`, `/entity/product/*`
- `apps/ockam` (Mars) — Ockam secure transport broker (scaffold, full impl Sprint 12)
- `apps/firmware` (Mercury) — RTIC bare-metal for ARM Cortex-M4, `no_std`. Excluded from workspace build

**Libraries:**
- `libs/core` — Domain types (`User`, `Product`), constants, traits. No business logic, no async
- `libs/secure` — **Single source of truth for all IAM**: Argon2id hashing, JWT (HS256 user + service tokens), RBAC (`Role`/`Scope` enums), `RequireScope` Axum middleware layer, newtype wrappers (`EmailAddress`, `UserId`, etc.)
- `libs/data` — Database clients: `SurrealClient` (HTTP driver), `ZenohClient` (pub/sub), `QdrantClient` (vector search)
- `libs/ai` — Burn 0.16 local inference engine. No HTTP, Zenoh-only
- `libs/hal` — Hardware abstraction layer, `no_std` compatible

**External services** (Docker Compose, all on `127.0.0.1:5xxxx`):
- SurrealDB (:58000) — graph/document DB, SCHEMAFULL schema, JWT auth
- Zenoh (:57447) — pub/sub with per-planet ACLs, topic scheme: `sakaloka/{planet}/{entity}/{event}`
- Qdrant (:56333) — vector search with scoped API keys

## Iron Curtain Standards

These are enforced by CI and clippy config:

- **No `.unwrap()`, `.expect()`, `panic!()`, or `unsafe`** outside `#[test]` blocks. Use `Result<T, E>` with `thiserror` in libraries, `anyhow` only in binary crates
- **100% doc comments** — every public item needs `///` docs with `# Errors` and `# Examples` sections
- **Deny all clippy warnings** — `cargo clippy -- -D warnings`
- **Newtype pattern** at boundaries — raw strings must not reach business logic (use `EmailAddress`, `Username`, `Password`, etc. from `libs/secure`)

## Key Patterns

**Route + RBAC middleware:**
```rust
router.route("/{id}", put(handler).route_layer(RequireScope::new(Scope::EntityWrite)))
```

**Auth flow:** Bearer token → `auth_middleware` extracts/validates via `libs/secure` → injects `UserClaims` into request extensions → `RequireScope` layer checks scopes.

**Database queries** always use parameterized SurrealQL:
```rust
db.query("SELECT * FROM user WHERE username = $username LIMIT 1")
  .bind(("username", username))
```

## Port Allocation

All external ports use the `5xxxx` scheme to avoid conflicts. Internal ports are standard (3000, 4000, 8000, etc.). See `docker-compose.yml` for the full mapping.

## Decision Tree: Which Crate?

- Auth/JWT/RBAC/password/tokens → `libs/secure` (never elsewhere)
- Domain types/constants → `libs/core`
- Database/messaging/vector queries → `libs/data`
- AI/embeddings → `libs/ai`
- HTTP routes/controllers → `apps/api`
- Hardware/sensor interfaces → `libs/hal`
