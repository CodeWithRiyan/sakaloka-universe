# 8-Planet Model

Every component in Sakaloka-Universe is named after a planet. Each planet owns a single
bounded context and communicates with others through authenticated, typed interfaces.

## Planet Registry

| Planet | Crate | Role | Port | Runtime |
|--------|-------|------|------|---------|
| Earth | `apps/api` | Axum REST API — entry point for all client traffic | `127.0.0.1:53000` | Tokio async |
| Mars | `apps/ockam` | Ockam secure transport broker for inter-planet channels | `127.0.0.1:54000` | Tokio async |
| Venus | `apps/venus` | Tauri + SvelteKit desktop application | *(no port)* | Tauri webview |
| Mercury | `apps/firmware` | RTIC bare-metal firmware for ARM Cortex-M4 | *(embedded)* | `no_std` RTIC |
| Jupiter | *(external)* | SurrealDB graph/document database | `127.0.0.1:58000` | Docker |
| Saturn | *(external)* | Eclipse Zenoh pub/sub message bus | `127.0.0.1:57447` | Docker |
| Uranus | *(external)* | Qdrant vector search engine | `127.0.0.1:56333` | Docker |
| Neptune | `libs/ai` | Burn 0.16 local AI inference engine | *(Zenoh only)* | Tokio async |

## Shared Libraries

Libraries live in `libs/` and are consumed by multiple planets:

| Library | Responsibility | Consumers |
|---------|---------------|-----------|
| `libs/core` | Domain types (`User`, `Product`, `Order`, `Organization`, `Role`, `Category`, `Brand`, `InventoryItem`) and constants | All planets |
| `libs/secure` | IAM: Argon2id hashing, JWT (HS256), RBAC, newtypes, middleware | Earth, Mars |
| `libs/data` | Database clients: `SurrealClient`, `ZenohClient`, `QdrantClient` | Earth, Neptune |
| `libs/ai` | Burn inference engine | Neptune |
| `libs/hal` | Hardware abstraction layer (`no_std`) | Mercury |

## Planet Sovereignty Rules

1. **Each planet owns its data** — Earth never writes directly to Jupiter's tables; it goes
   through the `sakaloka-data` abstraction layer.
2. **All inter-planet calls are authenticated** — Service JWTs with planet-specific `aud` claims.
3. **Libraries are shared, state is not** — `libs/core` provides types, but each planet manages
   its own runtime state.
4. **Mercury is isolated** — the firmware crate is `no_std` and excluded from the workspace build.
   Build with: `cargo build -p sakaloka-firmware --target thumbv7em-none-eabihf`
