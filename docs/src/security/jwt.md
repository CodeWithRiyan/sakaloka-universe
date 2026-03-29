# JWT Tokens

Sakaloka-Universe uses **two JWT types**, both signed with HS256 using the same
`SAKALOKA_JWT_SECRET` environment variable (minimum 32 characters).

## User JWT (`UserClaims`)

Issued by Earth after successful login or registration.

| Claim | Type | Description |
|-------|------|-------------|
| `sub` | `String` | User ID (SurrealDB record ID, e.g., `user:01JKXYZ`) |
| `role` | `String` | Role name (e.g., `admin`, `editor`, `viewer`) |
| `scopes` | `Vec<String>` | RBAC scopes (e.g., `["entity:read", "entity:write"]`) |
| `session_id` | `String` | Session UUID — links to the `session` table |
| `jti` | `String` | Unique token ID (UUID v4) — prevents replay attacks |
| `aud` | `Vec<String>` | `["sakaloka:earth"]` — only valid on Earth |
| `iss` | `String` | `"sakaloka:iam"` |
| `iat` | `u64` | Issued-at timestamp (Unix seconds) |
| `exp` | `u64` | Expiration timestamp — **15 minutes** after `iat` |

### Issuance

```rust
use sakaloka_secure::jwt::user_claims::issue_user_token;

let token = issue_user_token(
    &jwt_keys,
    &user_id,      // UserId newtype
    "admin",       // role name
    &["entity:read", "entity:write"],  // scopes
    &session_id,   // SessionId newtype
    Some("organization:01JKXYZ"),  // org_id
)?;
```

### Validation

The `auth_middleware` in `apps/api` automatically validates the Bearer token on every
protected route. It checks:

1. Signature validity (HS256)
2. Token not expired (`exp` > now)
3. Audience contains `"sakaloka:earth"`
4. Injects `UserClaims` into Axum request extensions

## Service JWT (`ServiceClaims`)

Used for authenticated inter-planet communication (e.g., Earth to Jupiter).

| Claim | Type | Description |
|-------|------|-------------|
| `sub` | `String` | Planet identity (e.g., `"sakaloka-api"`) |
| `planet` | `String` | Source planet name |
| `service_scopes` | `Vec<String>` | Service-level permissions |
| `jti` | `String` | Unique token ID |
| `aud` | `Vec<String>` | Target planet (e.g., `["sakaloka:jupiter"]`) |
| `iss` | `String` | `"sakaloka:iam"` |
| `iat` | `u64` | Issued-at timestamp |
| `exp` | `u64` | Expiration — **5 minutes** after `iat` |

### Audience Isolation

A token issued for Jupiter (`aud: ["sakaloka:jupiter"]`) is **rejected** by Saturn, even if
the signature is valid. This prevents token misuse across planet boundaries.

## Refresh Tokens

- Generated as UUID v4 strings
- **SHA-256 hashed** before storage (plaintext never persisted)
- Stored in the `session` table alongside the `session_id`
- Used to issue new access tokens without re-authenticating

```rust
use sakaloka_secure::tokens::rotation::hash_refresh_token;

let refresh_token = TokenId::new().to_string();
let token_hash = hash_refresh_token(&refresh_token);
// Store token_hash in SurrealDB, return refresh_token to client
```

## Security Properties

- **Short-lived access tokens** (15 min) limit the window of a stolen token
- **JTI claim** enables token blocklisting for immediate revocation
- **Audience-scoped** tokens prevent cross-planet replay
- **Refresh tokens are hashed** — database compromise does not expose usable tokens
- **Password never in JWT** — only the user ID and role are embedded
