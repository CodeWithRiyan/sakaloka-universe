# RBAC Matrix

Sakaloka-Universe uses **Role-Based Access Control** implemented in `libs/secure::rbac`.
Every protected route declares the scope it requires, and the `RequireScope` middleware
enforces it.

## Roles

| Role | Description |
|------|-------------|
| `Admin` | Full access — organization owner or administrator |
| `Editor` | Read + write access to entities — manager, cashier |
| `Viewer` | Read-only access to entities and search |
| `Service` | Inter-planet service identity — full DB and Zenoh access |

### Role Mapping from Database

The `map_rbac_role()` function in the auth controller maps DB role names to RBAC enum variants:

| DB Role Name | RBAC Role |
|-------------|-----------|
| `admin`, `owner` | `Admin` |
| `editor`, `manager`, `cashier` | `Editor` |
| *(anything else)* | `Viewer` (with warning log) |

## Scopes

| Scope | String | Description |
|-------|--------|-------------|
| `EntityRead` | `entity:read` | Read products, categories, brands, orders, inventory |
| `EntityWrite` | `entity:write` | Create and update entities |
| `EntityDelete` | `entity:delete` | Delete entities |
| `SearchRead` | `search:read` | Query vector search (Qdrant) |
| `UserRead` | `user:read` | List and view users, roles |
| `UserManage` | `user:manage` | Create, update, delete users and roles |
| `DbRead` | `db:read` | Direct database read (service-only) |
| `DbWrite` | `db:write` | Direct database write (service-only) |
| `DbAdmin` | `db:admin` | Database administration (service-only) |
| `ZenohPublish` | `zenoh:publish` | Publish to Zenoh topics (service-only) |
| `ZenohSubscribe` | `zenoh:subscribe` | Subscribe to Zenoh topics (service-only) |

## Permission Matrix

| Scope | Admin | Editor | Viewer | Service |
|-------|-------|--------|--------|---------|
| `entity:read` | yes | yes | yes | |
| `entity:write` | yes | yes | | |
| `entity:delete` | yes | | | |
| `search:read` | yes | yes | yes | |
| `user:read` | yes | | | |
| `user:manage` | yes | | | |
| `db:read` | | | | yes |
| `db:write` | | | | yes |
| `db:admin` | | | | yes |
| `zenoh:publish` | | | | yes |
| `zenoh:subscribe` | | | | yes |

## Route Protection

Routes declare required scopes using the `RequireScope` tower layer:

```rust
use sakaloka_secure::rbac::{guard::RequireScope, Scope};

// Read routes
let read_routes = Router::new()
    .route("/products", get(list))
    .route("/products/{id}", get(show))
    .route_layer(RequireScope::new(Scope::EntityRead));

// Write routes
let write_routes = Router::new()
    .route("/products", post(create))
    .route("/products/{id}", patch(update))
    .route_layer(RequireScope::new(Scope::EntityWrite));

// Delete routes
let delete_routes = Router::new()
    .route("/products/{id}", delete(remove))
    .route_layer(RequireScope::new(Scope::EntityDelete));
```

## How It Works

1. User logs in — role name is read from the database
2. `allowed_scopes(&role)` returns the list of scopes for that role
3. Scopes are embedded in the JWT `scopes` claim
4. On each request, `auth_middleware` validates JWT and injects `UserClaims`
5. `RequireScope` checks that `UserClaims.scopes` contains the required scope
6. If missing, returns `403 Forbidden`
