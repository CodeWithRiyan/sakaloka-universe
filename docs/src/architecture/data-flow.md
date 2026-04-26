# Data Flow

This page describes how requests flow through the Sakaloka-Universe system, from the client
to the database and back.

## Request Lifecycle

```
Venus (Desktop)
  |
  |  HTTPS / Bearer JWT
  v
Earth (Axum API)
  |
  +-- 1. CORS check (tower-http CorsLayer)
  +-- 2. Tracing layer (tower-http TraceLayer)
  +-- 3. Route matching (/api/*)
  +-- 4. Auth middleware (JWT validation -> UserClaims injection)
  +-- 5. RBAC scope guard (RequireScope layer)
  +-- 6. Handler execution
  |     +-- Read claims from Extension<UserClaims>
  |     +-- Call PostgreSQL via SQLx (parameterized queries)
  |     +-- Return Json<ApiResponse<T>>
  |
  v
PostgreSQL
  |
  +-- PostgreSQL via SQLx connection pool
```

## Authentication Flow

### Login

```
Client --POST /api/auth/login--> Earth
  |
  +-- 1. Find user by email (PostgreSQL)
  +-- 2. Verify password (Argon2id via libs/secure)
  +-- 3. Load organization + role
  +-- 4. Issue access token (JWT HS256, 15 min TTL)
  +-- 5. Generate refresh token (UUID, SHA-256 hashed)
  +-- 6. Persist session record (PostgreSQL)
  +-- 7. Update last_login_at (best-effort)
  +-- 8. Return { access_token, refresh_token, user_profile }
```

### Protected Request

```
Client --GET /api/products (Bearer token)--> Earth
  |
  +-- 1. auth_middleware extracts Bearer token from Authorization header
  +-- 2. Decode + validate JWT (signature, expiry, audience)
  +-- 3. Inject UserClaims into request extensions
  +-- 4. RequireScope checks claims.scopes contains required scope
  +-- 5. Handler reads Extension<UserClaims> and executes business logic
  +-- 6. Return paginated response
```

## Database Query Pattern

All database queries use **parameterized SQL** to prevent injection:

```rust
// Good - parameterized
sqlx::query_as("SELECT * FROM users WHERE email = $1 LIMIT 1")
  .bind(&email)

// Forbidden - string interpolation
sqlx::query(&format!("SELECT * FROM users WHERE email = '{email}'"))
```

## Batch Fetch Pattern (N+1 Prevention)

List endpoints that enrich items with related data use batch fetching:

```
1. Fetch page of items (e.g., products)
2. Collect unique foreign key IDs into a HashSet
3. Batch-fetch related entities: SELECT * FROM category WHERE id IN $ids
4. Build HashMap<ID, Entity> for O(1) lookup
5. Map over items using the HashMap
```

This is used in: menu endpoint (products -> categories), product list (products -> categories + brands),
and order creation (items -> products).

