# Fix Authorization Inconsistency and Seed Global Superadmin

## Summary
Refactor authorization so the **database role permissions** become the single source of truth for user access, while keeping route enforcement **scope-based**. Remove the current hardcoded role-to-scope issuance path from login/register, derive JWT `scopes` from `role.permissions` in DB, and add a deterministic dev migration that seeds a **global superadmin** user with full permissions. Keep the change within the current modular monolith and preserve Iron Curtain constraints: no panics, no unsafe, documented public items, and clippy-clean code.

## Key Changes
### Authorization model
- Keep **authentication** flow unchanged at a high level: login still finds user, verifies password, loads org + role, issues JWT, persists session.
- Change **authorization source of truth** from `map_rbac_role()` + `allowed_scopes()` to `role.permissions` loaded from DB.
- Keep `RequireScope` route guard as-is conceptually: it still checks `claims.scopes`.
- Treat `role` in JWT as informational only; real access comes from `scopes`.
- Replace the hardcoded role-name mapping in auth helpers with a deterministic permission-to-scope extractor that reads `serde_json::Value` from `role.permissions`.

### Permission shape contract
- Standardize backend authorization on one accepted DB shape: `role.permissions` is a JSON object of module keys to permission arrays, e.g. `{ "entity": ["read", "write"], "user": ["read", "manage"] }`.
- Add a backend normalization function that converts the stored JSON object into canonical scope strings used everywhere else, e.g.:
  - `entity + read -> entity:read`
  - `entity + write -> entity:write`
  - `user + manage -> user:manage`
- Reject or safely ignore malformed permission entries during token issuance; do not silently invent permissions.
- Keep the existing canonical scope vocabulary from `libs/secure/src/rbac/mod.rs` as the only allowed emitted scopes.
- Add a backend validator/helper for permission JSON so login, register, and role CRUD all use the same normalization rules.

### Auth flow refactor
- Update auth helpers so `issue_tokens_and_session` receives the resolved role record or normalized scopes, not only a role name.
- Update login to derive scopes from the loaded DB role before issuing JWT.
- Update register/bootstrap org creation to create the admin role with DB-backed permissions in the normalized object shape, then issue JWT from those stored permissions instead of hardcoded scope lists.
- Remove the hardcoded `map_rbac_role()` dependency from the access-token issuance path.
- Keep the RBAC matrix only if still needed for service-to-service tokens or explicit internal defaults; do not use it for user JWT issuance anymore.
- If the matrix remains, document it as non-user auth only to avoid future drift.

### Role and permission endpoints
- Keep role CRUD endpoints public contract-compatible where practical, but align backend expectations with the normalized object shape.
- Update `/api/roles/permissions` to remain the canonical list of available permission strings; its output should continue to be the allowed canonical scopes.
- Add validation in create/update role handlers so only known permission modules/actions can be stored.
- Ensure `RoleSummary.permissions` in auth response returns the stored DB permissions object, not a synthetic hardcoded object.

### Global superadmin seed migration
- Add a new ordered Surreal migration after current schema migrations to seed:
  - one global/system role with full permissions
  - one global superadmin user assigned to that role
- Seed behavior:
  - user is active immediately
  - no email verification flow is introduced in this refactor
  - role is system-defined / protected from normal mutation
- Use a deterministic seed identity for development:
  - email: `superadmin@sakaloka.local`
  - password: documented dev bootstrap password stored as an Argon2id hash in migration-generated data or seeded through application-safe hashing workflow chosen by implementer
- If organization is required by schema/business rules, create one explicit system organization for the seed and mark the superadmin role/user under it; otherwise treat “global” as a reserved system organization pattern rather than special null behavior.
- Make the migration idempotent by checking existing email/role records before insert or by using stable record IDs/unique constraints.

## Interfaces and Behavior
- JWT `UserClaims` shape does not need a breaking change; keep:
  - `role: String`
  - `scopes: Vec<String>`
- Backend-internal helper API should change to something equivalent to:
  - normalize permission JSON -> canonical scope list
  - issue tokens from role/scopes rather than role-name mapping
- Public login/register response shape stays the same.
- Route guards stay scope-based, not role-name-based.
- Role records in DB become the authoritative permission model for user access.

## Test Plan
- Unit tests for permission normalization:
  - valid object shape maps to expected scope strings
  - unknown modules/actions are rejected or ignored per chosen validation rule
  - duplicate permissions collapse cleanly
  - empty permission object yields no scopes
- Auth helper tests:
  - login-issued token contains scopes derived from DB role permissions
  - register-issued token contains scopes derived from newly created DB role record
  - no user JWT path depends on hardcoded role-name matrix anymore
- Guard tests:
  - request with required scope passes
  - request missing scope returns `403`
  - request with claims but empty scopes fails correctly
- Migration/seed tests or verification scenario:
  - migration creates system role + superadmin once
  - rerunning migrations does not duplicate seed data
  - seeded superadmin can log in and receives full canonical scopes
- Repo-wide verification:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - targeted auth/data tests under `cargo test --workspace`
  - any public-item additions include full docs to satisfy missing-docs enforcement

## Assumptions and Defaults
- This refactor is **authorization-only**; email confirmation/provider integration such as Resend is out of scope.
- Login identifier remains **email**.
- The canonical permission vocabulary remains the current scope list defined in secure RBAC code.
- The frontend-permissions object shape is adopted as the DB-backed source format for user roles.
- The global superadmin seed uses `superadmin@sakaloka.local` and a documented development bootstrap password unless you later replace that value before implementation.
- Existing user routes and guards remain scope-based; no switch to direct role-name checks is planned.
