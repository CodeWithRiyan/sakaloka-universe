# Planet Model

Every component in Sakaloka-Universe is named after a planet. Each planet owns a single
bounded context and communicates with others through authenticated, typed interfaces.

## Planet Registry

| Planet | Crate | Role | Port | Runtime |
|--------|-------|------|------|---------|
| Earth | `apps/api` | Axum REST API — entry point for all client traffic | `127.0.0.1:53000` | Tokio async |
| Venus | `apps/venus` | React + Tauri desktop application | *(localhost:5173)* | Tauri webview |

## Shared Libraries

Libraries live in `libs/` and are consumed by multiple planets:

| Library | Responsibility | Consumers |
|---------|---------------|-----------|
| `libs/core` | Domain types, constants | Earth, Venus |
| `libs/secure` | IAM: Argon2id hashing, JWT (HS256), RBAC, newtypes, middleware | Earth |
| `libs/data` | PostgreSQL client via SQLx | Earth |

## Planet Sovereignty Rules

1. **Each planet owns its data** — Earth accesses PostgreSQL through the `sakaloka-data`
   abstraction layer, keeping database concerns out of handler code.
2. **All inter-service calls are authenticated** — Service JWTs with planet-specific `aud` claims.
3. **Libraries are shared, state is not** — `libs/core` provides types, but each planet manages
   its own runtime state.
