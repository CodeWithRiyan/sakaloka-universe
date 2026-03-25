# IAM Overview

All Identity and Access Management (IAM) logic lives in **`libs/secure`**. No other crate
imports `argon2` or `jsonwebtoken` directly. This is the **single source of truth** for
authentication, authorization, and input validation.

## Components

| Module | Responsibility |
|--------|---------------|
| `libs/secure::argon2` | Password hashing (Argon2id) and verification |
| `libs/secure::jwt` | JWT token issuance and validation (HS256) |
| `libs/secure::jwt::user_claims` | User JWT claims (`UserClaims`) issued after login |
| `libs/secure::jwt::service_claims` | Service JWT claims (`ServiceClaims`) for inter-planet auth |
| `libs/secure::rbac` | Role and Scope enums, permission matrix |
| `libs/secure::rbac::guard` | `RequireScope` Axum middleware layer |
| `libs/secure::newtypes` | Validated input wrappers (`UserId`, `Password`, `EmailAddress`, etc.) |
| `libs/secure::tokens::rotation` | Refresh token hashing and rotation helpers |
| `libs/secure::error` | `SecureError` all IAM error types |

## Auth Flow Summary

1. **Registration** — client sends email + password + org name. Earth creates org, role, user, issues tokens.
2. **Login** — client sends email + password. Earth verifies with Argon2id, issues JWT + refresh token.
3. **Protected requests** — client sends `Authorization: Bearer <jwt>`. The `auth_middleware` validates and injects `UserClaims`.
4. **RBAC enforcement** — `RequireScope` layer checks that `UserClaims.scopes` contains the required scope.
5. **Token refresh** — client sends refresh token. Earth issues new token pair (returns 501 pending `RefreshStore` implementation).

## Newtype Boundary

Raw strings **never** enter business logic. All external inputs are wrapped at the boundary:

| Newtype | Wraps | Validation |
|---------|-------|-----------|
| `UserId` | `String` | Non-empty SurrealDB record ID |
| `EmailAddress` | `String` | Lowercase, contains `@` with non-empty local and domain parts |
| `Username` | `String` | 3-64 chars, alphanumeric + `_` + `-` |
| `Password` | `String` | Min 12 chars, cleared on `Drop` |
| `SessionId` | `Uuid` | Auto-generated UUID v4 |
| `TokenId` | `Uuid` | Auto-generated UUID v4 (JWT `jti`) |
| `Planet` | `String` | Non-empty, lowercase planet name |
| `ServiceName` | `String` | Non-empty service identifier |

## Error Types

All IAM operations return `Result<T, SecureError>`. Key variants:

- `HashFailed` — Argon2 hashing failure
- `VerifyFailed` — Password mismatch
- `JwtEncode` / `JwtDecode` — Token creation or validation failure
- `AudienceMismatch` — JWT `aud` claim does not match expected planet
- `TokenExpired` — JWT past its TTL
- `TokenReused` — Refresh token replay detected
- `InvalidEmail` / `InvalidUsername` / `InvalidPassword` — Input validation failures
- `InvalidInput` — Generic validation failure (e.g., invalid session ID format)
