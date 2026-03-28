# 🌌 Sakaloka-Universe — Product Requirements Document

**Version:** 1.4 | **Owner:** PT Riyan Solusi Teknologi | **Status:** Active — Full 8-Planet Universe
**Methodology:** Agile Scrum (2-Week Sprints) | **Confidentiality:** Internal Only

> This root README is the product and architecture PRD.
> For actual day-to-day development, use [SETUP.md](./SETUP.md), the root [Makefile](./Makefile), and the Venus app guide at [apps/venus/README.md](./apps/venus/README.md).

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Infrastructure Configuration](#2-infrastructure-configuration-server-sync)
3. [Technical Standards — Iron Curtain](#3-technical-standards--sakaloka-iron-curtain)
4. [Product Backlog](#4-product-backlog-epic-overview)
5. [Agile Sprint Plans](#5-agile-sprint-plans-10-sprints--2-weeks)
6. [Global Acceptance Criteria](#6-global-acceptance-criteria)
7. [Documentation Protocol](#7-documentation-protocol-luna)
8. [Agile Ceremonies & Cadence](#8-agile-ceremonies--cadence)
9. [Risk Register](#9-risk-register)
10. [Technology Stack Summary](#10-technology-stack-summary)
11. [Appendix](#11-appendix)

---

## 1. Executive Summary

Sakaloka-Universe is an enterprise-grade, full-stack platform developed by PT Riyan Solusi Teknologi. It is built on a strict Rust-first technology stack with a zero-panic, zero-unsafe-code policy. The system integrates a graph/document database, vector search engine, real-time messaging bus, and a local AI inference engine into a single cohesive universe of interconnected microservices.

This PRD outlines all product phases, user stories, sprint plans, acceptance criteria, and technical standards under the Agile Scrum framework to enable rapid, high-quality delivery.

### 1.1 Vision Statement

To build an intrinsically safe, self-documented, and performance-optimized platform that operates entirely within controlled infrastructure boundaries — delivering AI-augmented data experiences with zero runtime crashes and sub-10ms internal latency.

### 1.2 Stakeholders & Roles

| Role | Name / Team | Responsibility |
|---|---|---|
| Product Owner | PT Riyan Solusi Teknologi | Vision, Backlog Prioritization, Acceptance |
| Scrum Master | Tech Lead | Sprint Facilitation, Impediment Removal |
| Backend Dev Team | Rust Engineers | API, Data Layer, Security, AI Engine |
| Frontend Dev Team | React / Tauri Engineers | Web UI, Desktop Bridge, Component Design |
| DevOps / Infra | Infrastructure Team | Docker, Port Mgmt, Uptime Kuma, CI/CD |
| QA / Testing | Quality Assurance | Integration Tests, Cargo Test, Doc-Tests |

---

## 2. Infrastructure Configuration (Server-Sync)

To prevent port conflicts with existing services — Residence, POS Damai, Odoo, and other infrastructure — Sakaloka-Universe (Development Phase) uses a dedicated port allocation scheme with the prefix `5xxxx` on the host server, mapping to standard container ports internally.

### 2.1 Port Allocation Table

| Service / Planet | Container Port | External Port (Server) | Technology | Description |
|---|---|---|---|---|
| Mercury (RTIC) | — | — | RTIC (Rust) | Bare-metal embedded runtime; no container — flashed to hardware |
| Venus (Tauri) | 1420 | `127.0.0.1:51420` | Tauri 2.0 + React | Desktop/web UI — only external user surface |
| Earth (Loco.rs) | 3000 | `127.0.0.1:53000` | Loco.rs (Rust) | Primary REST/GraphQL API — business logic hub |
| Mars (Ockam) | 4000 | `127.0.0.1:54000` | Ockam (Rust) | Secure channel broker — mTLS + encrypted transport between planets |
| Jupiter (SurrealDB) | 8000 | `127.0.0.1:58000` | SurrealDB | Graph & Document Database |
| Saturn (Zenoh) | 7447 | `127.0.0.1:57447` | Eclipse Zenoh | Real-time Messaging Bus |
| Uranus (Qdrant) | 6333 | `127.0.0.1:56333` | Qdrant | Vector Search Engine |
| Neptune (Burn) | — | — | Burn (Rust) | Local AI inference — no HTTP, Zenoh only |
| Uptime Kuma | 3001 | `127.0.0.1:3001` | Node.js | Health Monitoring (Existing) |

### 2.2 Docker Compose Configuration

```yaml
version: '3.9'
services:
  # 🌍 Earth — business logic API
  sakaloka-api:
    build: ./apps/api
    ports:
      - '127.0.0.1:53000:3000'
    depends_on: [mars-ockam, surrealdb, qdrant, zenoh]

  # 🔴 Mars — Ockam secure channel broker (all inter-planet traffic routes through here)
  mars-ockam:
    build: ./apps/ockam
    ports:
      - '127.0.0.1:54000:4000'
    environment:
      - OCKAM_VAULT_SECRET=${SAKALOKA_OCKAM_SECRET}

  # 🪐 Jupiter — graph/document database
  surrealdb:
    image: surrealdb/surrealdb:latest
    ports:
      - '127.0.0.1:58000:8000'

  # 🪐 Saturn — real-time messaging bus
  zenoh:
    image: eclipse/zenoh:latest
    ports:
      - '127.0.0.1:57447:7447'

  # 🔵 Uranus — vector search engine
  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - '127.0.0.1:56333:6333'

  # ⚡ Mercury (RTIC) — bare-metal embedded firmware
  # Not a container. Firmware is cross-compiled and flashed to hardware.
  # Mercury publishes sensor data to Saturn (Zenoh) via serial/USB bridge.
```

---

## 3. Technical Standards — Sakaloka Iron Curtain

The "Iron Curtain" represents the non-negotiable quality gates enforced at compile time. Every Rust crate in the workspace must comply with these directives. Violations cause build failure — not warnings.

### 3.1 Compiler Enforcement Directives

Every `lib.rs` and `main.rs` file must contain the following at the top:

```rust
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]     // No .unwrap() allowed
#![deny(clippy::expect_used)]     // No .expect() allowed
#![deny(missing_docs)]            // 100% public API documentation
#![forbid(unsafe_code)]           // Zero unsafe code — absolute prohibition
```

### 3.2 No-Panic Policy

The application must never panic in production. This is enforced through the following code hygiene rules:

- Use the `?` operator to propagate `Result<T, E>` up the call chain
- Use `match` or `if let` to explicitly handle `Option<T>` branches
- All error types must implement the `std::error::Error` trait
- Custom error enums are preferred over `anyhow` in library crates
- Use `thiserror` for ergonomic custom error derivation

### 3.3 Cargo Workspace Structure

```
sakaloka-universe/
├── Cargo.toml              # Workspace root
├── apps/
│   ├── api/               # 🌍 Earth  — Loco.rs API server
│   ├── firmware/          # ⚡ Mercury — RTIC bare-metal firmware (no_std)
│   ├── ockam/             # 🔴 Mars   — Ockam secure channel node
│   └── web/               # 🌸 Venus  — React + Tauri 2.0 frontend
├── libs/
│   ├── core/              # Shared domain types & traits (std + no_std)
│   ├── data/              # SurrealDB, Qdrant, Zenoh clients
│   ├── secure/            # ALL IAM: Argon2, JWT, RBAC, Ockam identity
│   ├── hal/               # ⚡ Mercury — Hardware abstraction for RTIC tasks
│   └── ai/                # 🔮 Neptune — Burn model + embed()
└── docs/                  # MdBook + ADRs in /docs/adr/
```

> **`libs/hal` and `apps/firmware` are `no_std` crates.** They must never
> import std, tokio, or any async runtime. RTIC provides its own task
> scheduler on bare metal.

> **`libs/core` ships both `std` and `no_std` builds** via feature flags
> so Mercury can share domain types with the rest of the universe.

---

## 4. Product Backlog (Epic Overview)

The product backlog is organized into 5 Epics aligned with the 5 development phases. Each epic is broken down into user stories with story points using the Fibonacci scale (1, 2, 3, 5, 8, 13).

| Epic ID | Epic Name | Planet | Phase | Total Points | Priority |
|---|---|---|---|---|---|
| EP-01 | Environment & Infrastructure Setup | All | Phase 1 | 21 pts | 🔴 Critical |
| EP-02 | Data Layer & Security / IAM | Earth · Mars · Jupiter | Phase 2 | 34 pts | 🔴 Critical |
| EP-03 | Backend API with Loco.rs | Earth | Phase 3 | 21 pts | 🟠 High |
| EP-04 | Intelligence & Real-time Sync | Saturn · Uranus · Neptune | Phase 4 | 34 pts | 🟠 High |
| EP-05 | Frontend Interface (UI/Desktop) | Venus | Phase 5 | 21 pts | 🟡 Medium |
| EP-06 | Embedded Hardware Layer | Mercury | Phase 6 | 21 pts | 🟠 High |
| EP-07 | Secure Transport (Ockam Channels) | Mars | Phase 7 | 13 pts | 🔴 Critical |

---

## 5. Agile Sprint Plans (10 Sprints × 2 Weeks)

The project is structured into 10 two-week sprints covering all 5 phases. Each sprint includes user stories, acceptance criteria, and story point estimates. **Velocity target: 20–25 story points per sprint.**

---

### 5.1 Phase 1 — Environment & Infrastructure (EP-01)

#### Sprint 1 — Goal: Setup Docker environment and Cargo workspace with port isolation

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-01 | As a DevOps engineer, I want a `docker-compose.yml` with 5xxxx port bindings so services don't conflict with existing infrastructure. | All 4 services start without port conflicts; `docker ps` shows all containers healthy | 5 | 🔴 Critical |
| US-02 | As a developer, I want a Cargo workspace with `apps/` and `libs/` crates so all Rust code is organized in one monorepo. | `cargo build --workspace` succeeds; all Iron Curtain directives compile without warnings | 8 | 🔴 Critical |
| US-03 | As an operator, I want a `/health` endpoint on the API so Uptime Kuma can monitor service availability. | `GET /health` returns `200 OK` with JSON status; Uptime Kuma heartbeat is green | 3 | 🟠 High |
| US-04 | As a developer, I want Clippy and rustfmt CI checks so code quality is enforced on every commit. | CI pipeline runs `cargo clippy -- -D warnings` and `cargo fmt --check`; blocks merge on failure | 5 | 🟠 High |

**Sprint 1 Total: 21 pts**

---

#### Sprint 2 — Goal: Validate infrastructure ports and establish baseline documentation

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-05 | As a DevOps engineer, I want automated port conflict detection so new services never break existing ones. | Pre-deploy script scans for port collisions; deployment fails with clear error if conflict found | 5 | 🔴 Critical |
| US-06 | As a developer, I want an MdBook site in `/docs` so architecture decisions are always accessible. | `cargo doc` generates docs for all lib crates; `mdbook build` succeeds; site is browsable | 5 | 🟡 Medium |
| US-07 | As a developer, I want Docker health checks on all services so orchestration knows when services are ready. | All containers report healthy within 30s of start; `depends_on` respects health status | 3 | 🟠 High |

**Sprint 2 Total: 13 pts**

---

### 5.2 Phase 2 — Data Layer & Security / IAM (EP-02)

> **Decision log:** Inter-planet auth = JWT shared secret (`SAKALOKA_JWT_SECRET`). External user-facing auth surface = Venus only. All IAM logic lives in `libs/secure`.

---

#### IAM Architecture

`libs/secure` is the **single source of truth** for all authentication and authorization across the universe. No planet implements its own auth logic.

```
╔══════════════════════════════════════════════════════════════╗
║                   SAKALOKA IAM FLOW                          ║
║                                                              ║
║  [ Human User ]                                              ║
║       │  credentials                                         ║
║       ▼                                                      ║
║  ┌──────────┐   POST /auth/login    ┌─────────────────────┐ ║
║  │  Venus   │ ─────────────────────▶│  Earth  :53000      │ ║
║  │ React /  │ ◀─────────────────────│  Loco.rs API        │ ║
║  │  Tauri   │  { access, refresh }  └──────────┬──────────┘ ║
║  └──────────┘                                  │            ║
║  (Only external                    JWT (shared secret)      ║
║   user auth surface)               service tokens           ║
║                            ┌───────┴──────────────────┐    ║
║                            │                           │    ║
║                    ┌───────▼──────┐         ┌──────────▼──┐ ║
║                    │ Jupiter      │         │ Saturn       │ ║
║                    │ SurrealDB    │         │ Zenoh :57447 │ ║
║                    │ :58000       │         └──────────────┘ ║
║                    └──────────────┘                  │      ║
║                                            ┌─────────▼────┐ ║
║                    ┌─────────────────────┐ │ Uranus       │ ║
║                    │ Neptune  Burn AI    │ │ Qdrant :56333│ ║
║                    │ (no HTTP, Zenoh only│ └──────────────┘ ║
║                    └─────────────────────┘                  ║
╚══════════════════════════════════════════════════════════════╝
```

**Two token types — both signed with `SAKALOKA_JWT_SECRET` (HS256):**

| Token | Holder | TTL | Purpose |
|---|---|---|---|
| **User JWT** | Venus → stored in memory | 15 min | Human session; carries `role` + `scopes` |
| **Refresh Token** | Venus → OS keychain (Tauri) | 7 days | Rotates access tokens without re-login |
| **Service JWT** | Every planet (auto-managed) | 5 min | Planet-to-planet calls; carries `aud` + `service_scopes` |

---

#### Token Claim Schemas

**User JWT payload:**

```json
{
  "sub":        "user:01J...",
  "iss":        "sakaloka:earth",
  "aud":        ["sakaloka:earth"],
  "exp":        1720000000,
  "iat":        1719996400,
  "jti":        "uuid-v4",
  "role":       "editor",
  "scopes":     ["organization:read", "product:read", "product:update"],
  "session_id": "session:01J..."
}
```

**Service JWT payload (planet-to-planet):**

```json
{
  "sub":            "service:earth",
  "iss":            "sakaloka:iam",
  "aud":            ["sakaloka:jupiter"],
  "exp":            1719996700,
  "iat":            1719996400,
  "jti":            "uuid-v4",
  "service_scopes": ["db:read", "db:write"],
  "planet":         "earth"
}
```

> `aud` is always the **target planet name**. Jupiter rejects a token intended for Saturn — even if the signature is valid.

---

#### Per-Planet Integration

| Planet | Auth Mechanism | External User Auth? | Service Token Required? | Scopes |
|---|---|---|---|---|
| 🌍 **Earth** (Axum) | Validates User JWTs from Venus; issues service tokens to call other planets | ❌ Receives calls from Venus, not humans directly | Issues tokens, doesn't receive them | All |
| 🌸 **Venus** (React/Tauri) | **Only external user-facing surface.** Handles login UI, stores tokens securely | ✅ Yes — login, register, refresh, logout | ❌ Holds User JWT only | User scopes |
| 🪐 **Jupiter** (SurrealDB) | `DEFINE TOKEN` validates incoming service JWTs natively — no Earth proxy needed | ❌ Internal only | ✅ Earth's service token (`aud: sakaloka:jupiter`) | `db:read`, `db:write`, `db:delete`, `db:admin` |
| 🪐 **Saturn** (Zenoh) | Connection-time token validation + per-planet topic ACL | ❌ Internal only | ✅ Per-planet service token | `zenoh:publish`, `zenoh:subscribe` |
| 🔵 **Uranus** (Qdrant) | Scoped API keys derived from service JWTs via `ApiKeyVault` in `libs/secure` | ❌ Internal only | ✅ Neptune write key, Earth rw key | Per-collection read / write |
| 🔮 **Neptune** (Burn AI) | No HTTP surface — Zenoh only. Validates Saturn service token on connect | ❌ Completely isolated | ✅ Saturn service token | `zenoh:subscribe`, `zenoh:publish` |

---

#### RBAC — Roles, DB Permissions, and Scope Matrix

**Roles:** `admin` · `editor` · `viewer` · `service` (internal planets only)

User JWT scopes are now derived from the `role.permissions` JSON stored in the
database. Business roles should use feature-based permissions such as
`product:create` or `organization:select`, while technical scopes such as
`db:write` and `zenoh:publish` are reserved for system/bootstrap or
service-to-service use.

| Scope | `admin` | `editor` | `viewer` | `service` |
|---|---|---|---|---|
| `organization:read` | ✅ | ✅ | ✅ | ❌ |
| `organization:select` | ✅ | ✅ | ❌ | ❌ |
| `product:read` | ✅ | ✅ | ✅ | ❌ |
| `product:create` | ✅ | ✅ | ❌ | ❌ |
| `product:update` | ✅ | ✅ | ❌ | ❌ |
| `product:delete` | ✅ | ❌ | ❌ | ❌ |
| `role:read` | ✅ | ❌ | ❌ | ❌ |
| `role:update` | ✅ | ❌ | ❌ | ❌ |
| `user:read` | ✅ | ❌ | ❌ | ❌ |
| `user:update` | ✅ | ❌ | ❌ | ❌ |
| `db:read` | ✅ | ❌ | ❌ | ✅ |
| `db:write` | ✅ | ❌ | ❌ | ✅ |
| `db:admin` | ✅ | ❌ | ❌ | ❌ |
| `zenoh:publish` | ✅ | ❌ | ❌ | ✅ |
| `zenoh:subscribe` | ✅ | ❌ | ❌ | ✅ |

---

#### Venus Auth Flows (Only External Surface)

**Login flow:**
```
User types credentials
       │
       ▼
Venus  →  POST /auth/login { username, password }  →  Earth
                                                         │
                                              Argon2 verify hash
                                              Issue User JWT (15min)
                                              Issue Refresh Token (7d)
                                                         │
Venus  ←  { access_token, refresh_token }  ←  Earth
   │
   ├── access_token  →  React state (memory only, never localStorage)
   └── refresh_token →  Tauri OS keychain (encrypted at rest)
```

**Authenticated request flow:**
```
Venus  →  GET /products/123
          Authorization: Bearer <access_token>  →  Earth
                                                      │
                                           validate_user_jwt()
                                           check scope: product:read
                                                      │
                                           get_service_token(Jupiter, 5min)
                                                      │
                                           SELECT * WHERE id = 123
                                           Authorization: Bearer <service_token>  →  Jupiter
                                                                                        │
                                                                           DEFINE TOKEN validates
                                                                           aud = sakaloka:jupiter ✅
                                                                                        │
Venus  ←  { data }  ←  Earth  ←─────────────────────────────────────────────  { rows }
```

**Refresh token rotation:**
```
(access_token nearing expiry)
Venus  →  POST /auth/refresh { refresh_token }  →  Earth
                                                      │
                                           Validate refresh token
                                           Invalidate old token (JTI blocklist)
                                           Issue new access_token (15min)
                                           Issue new refresh_token (7d)  ← rotation
                                                      │
Venus  ←  { new_access_token, new_refresh_token }  ←  Earth
```

> ⚠️ Reusing an already-rotated refresh token terminates the entire session immediately (`TokenReused` error).

---

#### `libs/secure` Crate Structure

```
libs/secure/
├── src/
│   ├── lib.rs              # Iron Curtain directives + pub re-exports
│   ├── argon2.rs           # hash_password(), verify_password()
│   ├── jwt/
│   │   ├── mod.rs          # JwtKeys struct, SAKALOKA_JWT_SECRET loading
│   │   ├── user_claims.rs  # UserClaims, issue_user_token(), validate_user_token()
│   │   └── service_claims.rs  # ServiceClaims, issue_service_token(), validate_service_token()
│   ├── rbac/
│   │   ├── mod.rs          # Role, Scope enums
│   │   ├── guard.rs        # RequireScope — Axum middleware layer
│   │   └── matrix.rs       # Role::allowed_scopes() lookup table
│   ├── tokens/
│   │   ├── store.rs        # ServiceTokenStore — cache + proactive refresh
│   │   └── rotation.rs     # rotate_refresh_token(), JTI blocklist check
│   ├── keys/
│   │   └── vault.rs        # ApiKeyVault — scoped Qdrant key management
│   └── newtypes.rs         # UserId, TokenId, SessionId, Planet, ServiceName, EmailAddress, ...
```

**Key public functions:**

| Function | Module | Description |
|---|---|---|
| `hash_password(pw)` | `argon2` | Argon2id hash → `Result<String, SecureError>` |
| `verify_password(pw, hash)` | `argon2` | Constant-time verify → `Result<bool, SecureError>` |
| `JwtKeys::issue_user_token(sub, role)` | `jwt` | Sign User JWT → `Result<String, SecureError>` |
| `JwtKeys::issue_service_token(planet, aud)` | `jwt` | Sign Service JWT with audience → `Result<String, SecureError>` |
| `JwtKeys::validate_user_token(token)` | `jwt` | Decode + verify → `Result<UserClaims, SecureError>` |
| `JwtKeys::validate_service_token(token, expected_aud)` | `jwt` | Decode + verify audience → `Result<ServiceClaims, SecureError>` |
| `ServiceTokenStore::get_token(planet)` | `tokens` | Return cached or fresh service token |
| `rotate_refresh_token(old, db, keys)` | `tokens` | Rotate + JTI blocklist → `Result<TokenPair, SecureError>` |
| `RequireScope::new(scope)` | `rbac` | Axum middleware enforcing a single scope |
| `Role::allowed_scopes()` | `rbac` | Returns `&[Scope]` for a given role |
| `ApiKeyVault::get_qdrant_key(scope)` | `keys` | Return scoped Qdrant API key |

---

#### Sprint 3 — Goal: IAM Core — Argon2, JWT (User + Service), SurrealDB schema

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-08 | As a developer, I want Schema-Full SurrealDB tables for `user`, `session`, `refresh_token`, and `jti_blocklist` so identity state is persisted with strict integrity. | All `.surql` files load without errors; SCHEMAFULL rejects invalid fields; migrations are idempotent | 8 | 🔴 Critical |
| US-09 | As a security engineer, I want Argon2id password hashing in `libs/secure` so credentials are never stored in plaintext. | Argon2 PHC strings in DB; `verify_password()` returns `Result<bool, SecureError>`; zero `.unwrap()` | 5 | 🔴 Critical |
| US-10 | As a developer, I want User JWT and Service JWT both signed with `SAKALOKA_JWT_SECRET` and carrying an `aud` claim so tokens cannot be replayed across planets. | User token `aud = sakaloka:earth`; service token `aud = sakaloka:<target>`; mismatched `aud` returns `AudienceMismatch` error | 8 | 🔴 Critical |
| US-10b | As a developer, I want `ServiceTokenStore` to proactively cache and refresh service tokens so no planet ever calls another with an expired credential. | Cache hit > 99% under load; tokens refreshed 60s before expiry; exponential backoff on refresh failure | 5 | 🔴 Critical |

**Sprint 3 Total: 26 pts**

---

#### Sprint 4 — Goal: RBAC, Newtype validation, Jupiter + Saturn IAM wiring

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-11 | As a developer, I want Newtype wrappers (`EmailAddress`, `Username`, `Password`, `Planet`, `TokenId`) so raw strings never enter business logic. | Construction returns `Result`; invalid input produces a descriptive error; no `.unwrap()` anywhere in newtypes | 8 | 🔴 Critical |
| US-12 | As a developer, I want `RequireScope` middleware applied to every Earth route so access is enforced by fine-grained permission scopes at the framework level. | 403 returned for insufficient scope with structured error body; `viewer` role blocked from `product:update` routes | 8 | 🔴 Critical |
| US-30 | As a developer, I want SurrealDB configured with `DEFINE TOKEN` using `SAKALOKA_JWT_SECRET` so Jupiter validates incoming service JWTs natively without an Earth proxy. | SurrealDB rejects tokens with wrong `aud` or invalid signature; `db:read` tokens cannot execute mutations | 8 | 🔴 Critical |
| US-31 | As a developer, I want Zenoh topic ACLs configured per planet so no service can publish or subscribe outside its designated namespace. | Earth blocked from publishing to `sakaloka/jupiter/**`; ACL violations logged with planet identity; Venus has zero direct Zenoh access | 5 | 🔴 Critical |
| US-13 | As a QA engineer, I want doc-tests for every public function in `libs/secure` so the IAM documentation is always executable and correct. | `cargo test` runs all doc examples; zero failures; `#![deny(missing_docs)]` passes for entire crate | 3 | 🟠 High |

**Sprint 4 Total: 32 pts**

---

#### Sprint 4b — Goal: Uranus + Neptune isolation, Venus secure storage, refresh rotation

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-33 | As a developer, I want `ApiKeyVault` in `libs/secure` to provision scoped Qdrant API keys so Neptune can only write vectors and cannot execute search queries. | Neptune write-only key returns 403 on search; Earth rw key can upsert and search; key rotation tested end-to-end | 5 | 🟠 High |
| US-34 | As a security engineer, I want Neptune to expose no HTTP port and only communicate via Zenoh with a valid Saturn service token so it is fully isolated from the internet. | `docker inspect` shows no port bindings for Neptune; only Zenoh session authenticated; `nmap` scan returns no open ports | 5 | 🔴 Critical |
| US-35 | As a Venus user, I want my refresh token stored in the OS keychain via `tauri-plugin-store` with encryption so it is never readable by JavaScript or written to disk unencrypted. | Access token stored in React state only; refresh token in encrypted Tauri store; `localStorage` audit returns zero JWT entries | 8 | 🔴 Critical |
| US-36 | As a security engineer, I want refresh token rotation on every `/auth/refresh` so a stolen refresh token cannot be reused after the legitimate user has already refreshed. | Old refresh token invalidated on use; reuse attempt returns `TokenReused` and terminates full session; JTI blocklist enforced on every request | 8 | 🔴 Critical |

**Sprint 4b Total: 26 pts**

---

#### IAM Acceptance Criteria

| ID | Criterion | Test Method | Pass Condition |
|---|---|---|---|
| IAM-01 | Service tokens carry correct `aud` | Unit: issue Jupiter token, validate against Uranus | `AudienceMismatch` error returned |
| IAM-02 | Expired tokens rejected across all planets | Integration: use token after TTL + 1s | 401 / auth error; no data leaked |
| IAM-03 | Refresh token rotation — reuse detected | Integration: use refresh token twice | Second use returns `TokenReused`; session destroyed |
| IAM-04 | RBAC scope enforcement on Earth routes | Integration: `viewer` calls write endpoint | 403 with scope error detail |
| IAM-05 | Jupiter validates service JWT via `DEFINE TOKEN` | Integration: send self-signed JWT to SurrealDB | Query rejected; no rows returned |
| IAM-06 | Zenoh ACL blocks cross-planet publishing | Integration: Earth publishes to `sakaloka/jupiter/**` | Publish rejected; ACL violation logged |
| IAM-07 | Neptune has zero external ports | `nmap` + `docker inspect` on Neptune container | No open ports found |
| IAM-08 | Venus access token never touches disk | Code audit + Tauri integration test | Zero `localStorage` writes; no JWT in any file |
| IAM-09 | Qdrant Neptune write key cannot search | Integration: use Neptune key for a search query | 403 from Qdrant |
| IAM-10 | Zero `.unwrap()` in `libs/secure` | `cargo clippy` with `#![deny(clippy::unwrap_used)]` | Build succeeds, zero warnings |

---

### 5.3 Phase 3 — Backend API with Loco.rs (EP-03)

#### Sprint 5 — Goal: Scaffold Loco.rs API with CRUD endpoints and Swagger UI

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-14 | As a frontend developer, I want CRUD endpoints for core entities so the UI can manage data. | POST, GET, PUT, DELETE for all core resources; responses are typed Rust structs serialized to JSON; errors follow RFC 9457 Problem Details format | 8 | 🟠 High |
| US-15 | As a developer, I want Swagger UI auto-generated from code so API contracts are always up-to-date. | Swagger UI available at `/swagger-ui`; all endpoints documented; schemas derived from Rust types | 5 | 🟠 High |
| US-16 | As a developer, I want request/response middleware for logging so all API traffic is observable. | Structured JSON logs with request ID, latency, status code; latency percentiles exported to metrics endpoint | 5 | 🟡 Medium |

**Sprint 5 Total: 18 pts**

---

#### Sprint 6 — Goal: API performance validation and full Rustdoc coverage

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-17 | As a performance engineer, I want internal API-to-DB latency below 10ms so response SLAs are met. | Benchmark tests confirm p99 latency < 10ms for SurrealDB queries on local Docker network | 8 | 🔴 Critical |
| US-18 | As a developer, I want 100% Rustdoc coverage on all public APIs in `libs/` so onboarding is self-service. | `#![deny(missing_docs)]` compiles with zero errors; `cargo doc --open` shows all functions documented | 5 | 🟠 High |
| US-19 | As a QA engineer, I want integration tests for all API routes so regressions are caught automatically. | Integration test suite runs against in-memory/test DB; all routes have at least one happy path and one error path test | 5 | 🟠 High |

**Sprint 6 Total: 18 pts**

---

### 5.4 Phase 4 — Intelligence & Real-time (EP-04)

#### Sprint 7 — Goal: Implement Zenoh pub/sub messaging and Burn AI embedding engine

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-20 | As a developer, I want a Zenoh pub/sub bus so services can emit and consume events without tight coupling. | Zenoh router reachable at port 57447; publisher and subscriber sessions established; messages delivered with < 5ms broker latency | 8 | 🟠 High |
| US-21 | As a data scientist, I want a local Burn AI embedding model in `libs/ai` so sensitive data is never sent to external APIs. | Model loads from local file path; `embed()` function returns `Vec<f32>` of correct dimension; runs on CPU with acceptable throughput | 13 | 🟠 High |

**Sprint 7 Total: 21 pts**

---

#### Sprint 8 — Goal: Vector search pipeline — SurrealDB to Qdrant sync and semantic search

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-22 | As a developer, I want automatic embedding sync from SurrealDB to Qdrant so vector search is always current. | On record insert/update, embedding is computed and upserted into Qdrant; sync latency < 500ms; sync failures are retried with backoff | 13 | 🟠 High |
| US-23 | As a user, I want semantic search across all entities so I can find records by meaning, not just keywords. | `POST /search?q=` returns top-N results ranked by cosine similarity; results include score and entity reference; response time < 200ms | 8 | 🟠 High |

**Sprint 8 Total: 21 pts**

---

### 5.5 Phase 5 — Frontend Interface (EP-05)

#### Sprint 9 — Goal: Build React UI with component library and API integration

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-24 | As a user, I want a React dashboard so I can view and manage data from any browser. | React app serves from Vite dev server; all core entities have List, Detail, and Form views; API errors are displayed in user-friendly messages | 8 | 🟡 Medium |
| US-25 | As a user, I want a semantic search bar in the UI so I can find any record instantly by natural language. | Search input calls `POST /search`; results appear with relevance scores; debounce prevents excessive API calls | 5 | 🟡 Medium |
| US-26 | As a developer, I want Tauri 2.0 commands connecting UI to Rust backend so the desktop app can use native OS capabilities. | Tauri Commands return `Result<T, String>`; all commands are typed with serde; frontend handles `Err` variant gracefully with toast notification | 8 | 🟡 Medium |

**Sprint 9 Total: 21 pts**

---

#### Sprint 10 — Goal: System integration testing, documentation freeze, and release readiness

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-27 | As a product owner, I want end-to-end integration tests across all services so release quality is confirmed. | E2E test suite covers critical user journeys; all 5 services running simultaneously; zero panics in test run | 8 | 🔴 Critical |
| US-28 | As a developer, I want a frozen MdBook documentation release so the architecture knowledge is captured for the v1.3 milestone. | `mdbook build` succeeds; all modules documented; doc-tests pass; README accurately reflects current state | 5 | 🟠 High |
| US-29 | As a DevOps engineer, I want a final port audit confirming zero conflicts with Residence, POS Damai, and Odoo services. | Automated port scan report generated; all Sakaloka ports in 5xxxx range; no overlaps detected; report archived | 3 | 🔴 Critical |

**Sprint 10 Total: 16 pts**

---

## 6. Global Acceptance Criteria

The following criteria must ALL be satisfied before any Sprint Review is considered complete and before a release is approved. These are non-negotiable product-level quality gates.

| # | Criterion | Measurement Method | Target |
|---|---|---|---|
| AC-01 | Zero Runtime Crash (No Panic) | `cargo test` + production log analysis; zero `panic!` occurrences | 0 panics in 30-day observation window |
| AC-02 | Port Compatibility | Automated port scan script pre/post-deployment | Zero conflicts with Residence, POS Damai, Odoo |
| AC-03 | 100% API Documentation | `cargo doc` runs with `#![deny(missing_docs)]`; zero compile errors | All public items documented |
| AC-04 | Internal Latency SLA | Benchmark tool (`criterion.rs`) measuring API → DB round-trip | p99 < 10ms on local Docker network |
| AC-05 | Zero Unsafe Code | `#![forbid(unsafe_code)]` in all crates; CI enforced | Zero unsafe blocks anywhere in codebase |
| AC-06 | Doc-Test Pass Rate | `cargo test` includes all `///` code examples | 100% doc-test pass rate |
| AC-07 | Health Check Coverage | Uptime Kuma monitors all 4 Sakaloka services | All services green within 30s of container start |

---

## 7. Documentation Protocol (Luna)

Documentation is not optional — it is compiled and tested. The Luna protocol ensures that all knowledge is captured, versioned, and verified on every CI run.

### 7.1 Rustdoc Standards

- Every public function, struct, enum, and trait in `libs/` must have a doc comment (`///`)
- Doc comments must include: a one-line summary, extended description if complex, and at least one `/// # Examples` block
- `cargo doc --no-deps` must complete with zero warnings when run with `#![deny(missing_docs)]`

### 7.2 Doc-Test Requirements

- Every code example in documentation must be a valid, compilable Rust snippet
- `cargo test` runs all doc-tests automatically as part of the CI pipeline
- Doc-tests that require external services should use `#[cfg(test)]` feature flags

### 7.3 MdBook Architecture Guide

- All architectural decisions are recorded as Architecture Decision Records (ADR) in `/docs/adr/`
- MdBook is built and deployed alongside Rustdoc on every merge to `main`
- Each phase has a dedicated chapter with diagrams, rationale, and component descriptions

---

## 8. Agile Ceremonies & Cadence

| Ceremony | Frequency | Duration | Participants | Output |
|---|---|---|---|---|
| Sprint Planning | Every 2 weeks (start) | 3 hours | PO, Scrum Master, Dev Team | Sprint Backlog, Sprint Goal |
| Daily Standup | Daily | 15 minutes | Dev Team, Scrum Master | Blockers identified, progress updated |
| Sprint Review | Every 2 weeks (end) | 2 hours | All stakeholders | Demo, feedback, backlog update |
| Sprint Retrospective | Every 2 weeks (end) | 1.5 hours | Dev Team, Scrum Master | Action items for process improvement |
| Backlog Refinement | Weekly (mid-sprint) | 1 hour | PO, Dev Team lead | Groomed stories, acceptance criteria |
| Tech Spike Review | As needed | 1 hour | Dev Team | Spike findings, decision record |

### 8.1 Definition of Ready (DoR)

A User Story is ready to be pulled into a Sprint only if ALL of the following are true:

- User Story is written in standard format: *As a [role], I want [feature] so that [benefit]*
- Acceptance Criteria are specific, measurable, and agreed upon by the PO
- Story points have been estimated by the team using Planning Poker
- All external dependencies are identified and unblocked
- Technical approach is agreed upon (no open architecture questions)

### 8.2 Definition of Done (DoD)

A User Story is complete only if ALL of the following are satisfied:

- Code is merged to `main` with no merge conflicts
- All CI checks pass: `cargo clippy`, `cargo fmt`, `cargo test`, `cargo doc`
- Acceptance Criteria verified by Product Owner
- No new clippy warnings introduced (zero-warning policy)
- Rustdoc updated for any new or modified public APIs
- Integration test added or updated to cover the new behavior
- No `.unwrap()` or `.expect()` introduced (Iron Curtain compliance)

---

## 9. Risk Register

| Risk ID | Risk Description | Likelihood | Impact | Mitigation Strategy |
|---|---|---|---|---|
| R-01 | Port conflict with existing services (Residence, POS Damai, Odoo) | Low | 🔴 Critical | Automated port scan pre-deploy; dedicated 5xxxx prefix; docker network isolation |
| R-02 | Burn AI model inference too slow on CPU for production load | Medium | 🟠 High | Benchmark early in Sprint 7; consider ONNX runtime fallback; cache embeddings aggressively |
| R-03 | SurrealDB schema migration breaks existing data | Low | 🟠 High | Schema versioning in `.surql` files; migration dry-run in staging before production |
| R-04 | Developer adds `.unwrap()` bypassing Iron Curtain | Medium | 🟡 Medium | CI enforces `#![deny(clippy::unwrap_used)]`; PR review checklist includes Iron Curtain check |
| R-05 | Zenoh pub/sub introduces message ordering issues | Low | 🟡 Medium | Design for idempotent event handlers; use sequence numbers in event payloads |
| R-06 | Sprint velocity lower than 20 points due to Rust learning curve | Medium | 🟡 Medium | Pair programming; Rust study sessions in retrospective; allocate spike time in early sprints |

---

## 10. Technology Stack Summary

| Layer | Technology | Version Target | Role in System |
|---|---|---|---|
| Planet | Technology | Version | Role |
|---|---|---|---|
| ⚡ Mercury | RTIC | 2.x | Bare-metal embedded runtime — hardware tasks, ISRs, sensor I/O (`no_std`) |
| 🌸 Venus | Tauri 2.0 + React + Vite | Tauri 2.x, React 18+ | Desktop/web UI — only external user-facing surface |
| 🌍 Earth | Loco.rs (Rust) | Latest stable | REST/GraphQL API — all business logic lives here |
| 🔴 Mars | Ockam (Rust) | Latest stable | End-to-end encrypted secure channels between all planets |
| 🪐 Jupiter | SurrealDB | Latest stable | Graph + Document + Relational hybrid database |
| 🪐 Saturn | Eclipse Zenoh | Latest stable | Pub/sub real-time messaging bus |
| 🔵 Uranus | Qdrant | Latest stable | Vector embedding storage and semantic search |
| 🔮 Neptune | Burn (Rust) | Latest stable | Local AI inference engine — no HTTP, Zenoh only |
| — | Docker + Compose | v2.x | Service containerisation (all planets except Mercury) |
| — | Uptime Kuma | Existing | Health monitoring on port 3001 |
| — | Rustdoc + MdBook | Latest | API docs + architecture knowledge base |
| — | Argon2 (`argon2` crate) | Latest | Password hashing in `libs/secure` |
| — | `jsonwebtoken` crate | Latest | JWT signing and validation in `libs/secure` |

---

### 5.6 Phase 6 — Embedded Hardware Layer (EP-06) — ⚡ Mercury

#### Sprint 11 — Goal: RTIC firmware scaffold, hardware abstraction, and Zenoh bridge

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-37 | As a hardware engineer, I want an RTIC 2.x firmware project in `apps/firmware` with a `no_std` HAL in `libs/hal` so embedded tasks are structured and safe. | `cargo build --target thumbv7em-none-eabihf` succeeds; Iron Curtain directives applied; zero `unsafe` in HAL (RTIC internal `unsafe` is isolated) | 8 | 🟠 High |
| US-38 | As a hardware engineer, I want RTIC tasks for sensor polling and serial output so hardware events are captured in real time with deterministic scheduling. | Sensor task runs at fixed priority; readings serialised to a shared resource; no data races detected by RTIC's compile-time analysis | 8 | 🟠 High |
| US-39 | As a developer, I want a serial-to-Zenoh bridge process that reads Mercury's output and publishes to `sakaloka/mercury/sensor/{type}` so the rest of the universe can consume hardware events. | Bridge publishes within 50ms of serial receipt; message format is typed and deserializable; bridge reconnects automatically on serial disconnect | 5 | 🟠 High |

**Sprint 11 Total: 21 pts**

---

### 5.7 Phase 7 — Secure Transport (EP-07) — 🔴 Mars

#### Sprint 12 — Goal: Ockam secure channels between all planets

| ID | User Story | Acceptance Criteria | Points | Priority |
|---|---|---|---|---|
| US-40 | As a security engineer, I want Ockam secure channels established between Earth ↔ Jupiter, Earth ↔ Saturn, and Earth ↔ Uranus so all inter-planet traffic is end-to-end encrypted at the transport layer — in addition to JWT application-layer auth. | Ockam node starts at port 54000; channels verified with `ockam secure-channel show`; traffic sniffed between planets shows ciphertext only | 8 | 🔴 Critical |
| US-41 | As a developer, I want Mercury's Zenoh bridge to route through Mars (Ockam) so sensor data from hardware is encrypted in transit to Saturn. | Mercury → Mars → Saturn channel established; plaintext sensor readings not observable on the Docker network; latency overhead < 2ms | 5 | 🔴 Critical |

**Sprint 12 Total: 13 pts**

---

## 11. Appendix

### 11.1 Sprint Velocity Plan

| Sprint | Phase | Story Points | Cumulative | Key Milestone |
|---|---|---|---|---|
| Sprint 1 | Phase 1 | 21 | 21 | Docker + Workspace running |
| Sprint 2 | Phase 1 | 13 | 34 | Port isolation confirmed |
| Sprint 3 | Phase 2 | 21 | 55 | SurrealDB schema + IAM live |
| Sprint 4 | Phase 2 | 19 | 74 | RBAC + Input validation complete |
| Sprint 5 | Phase 3 | 18 | 92 | API CRUD + Swagger UI |
| Sprint 6 | Phase 3 | 18 | 110 | Latency SLA confirmed < 10ms |
| Sprint 7 | Phase 4 | 21 | 131 | Zenoh + Burn AI running |
| Sprint 8 | Phase 4 | 21 | 152 | Vector search pipeline active |
| Sprint 9 | Phase 5 | 21 | 173 | React UI + Tauri bridge |
| Sprint 10 | Phase 5 | 16 | 189 | Release readiness confirmed |
| Sprint 11 | Phase 6 | 21 | 210 | Mercury RTIC firmware + Zenoh bridge |
| Sprint 12 | Phase 7 | 13 | 223 | Mars Ockam secure channels live ✅ |

**Total Story Points: 223**

---

### 11.2 Glossary

| Term | Definition |
|---|---|
| Iron Curtain | The set of Rust compiler directives (`#![deny/forbid]`) that enforce zero-panic, zero-unsafe code standards |
| No-Panic Policy | The architectural requirement that zero runtime panics occur in production code |
| Newtype Pattern | A Rust pattern wrapping primitive types in structs to enforce validation at construction |
| SurrealDB Schema-Full | A SurrealDB mode where tables enforce a strict field schema, rejecting undefined fields |
| Zenoh | Eclipse Zenoh — a unified data in motion, data at rest, and computations protocol |
| Burn | A deep learning framework written in Rust, used for local AI inference without external API calls |
| Loco.rs | A Ruby on Rails-inspired web framework for Rust, built on Axum and SeaORM |
| Tauri Command | A Rust function exposed to the JavaScript frontend via Tauri's IPC bridge, returning `Result<T, E>` |
| ADR | Architecture Decision Record — a document capturing an important architectural decision and its context |
| DoD / DoR | Definition of Done / Definition of Ready — agile completion and readiness checklists |
| RTIC | Real-Time Interrupt-driven Concurrency — a Rust framework for building deterministic embedded systems on ARM Cortex-M (`no_std`) |
| Mercury | The embedded firmware planet; closest to hardware; runs RTIC tasks; no OS, no allocator |
| Mars | The security transport planet; runs Ockam secure channel nodes; encrypts all inter-planet network traffic |
| Ockam | A Rust library for end-to-end encrypted, mutually authenticated secure channels across distributed services |
| no_std | A Rust compilation mode without the standard library, required for bare-metal embedded targets like Mercury |
| HAL | Hardware Abstraction Layer — `libs/hal` wraps low-level peripheral access for RTIC tasks in Mercury |

---

*PT Riyan Solusi Teknologi — Sakaloka-Universe PRD v1.4*
*Infrastructure: Ready | Security: Hardened | Code: Strict*
