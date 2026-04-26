# Architecture Overview

Sakaloka-Universe follows a **planet-based microservice architecture** where each service
is named after a planet and owns a single bounded context.

```
┌──────────────────────────────────────────────────┐
│                 Sakaloka-Universe                 │
│                                                  │
│  🌸 Venus (Desktop)  ─── HTTPS ──→  🌍 Earth    │
│                                      (Axum API)  │
│                                          │       │
│                                          ▼       │
│                                     PostgreSQL   │
└──────────────────────────────────────────────────┘
```

## Key Design Principles

### 1. Planet Sovereignty
Each planet owns its data store exclusively. Earth accesses PostgreSQL through the
`sakaloka-data` abstraction layer, keeping database concerns out of handler code.

### 2. Authenticated Inter-Planet Communication
All inter-service calls use **Service JWTs** issued by the `sakaloka:iam` authority.
The `aud` claim scopes each token to its intended recipient.

### 3. Iron Curtain
The compiler enforces correctness. Every library crate has:
```rust
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
```

### 4. Single IAM Layer
All authentication logic lives in `libs/secure`. No other crate imports `argon2` or
`jsonwebtoken`. This is enforced by Clippy and verified in CI.
