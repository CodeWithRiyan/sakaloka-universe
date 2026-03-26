# Coding Standards

These standards are enforced by CI and apply to all code in the workspace.

## Iron Curtain Rules

Every `lib.rs` and `main.rs` must include these directives:

```rust
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
```

See [Iron Curtain Rules](../architecture/iron-curtain.md) for full details.

## Error Handling

| Location | Pattern |
|----------|---------|
| `libs/` crates | `thiserror` for typed, structured errors |
| `apps/api` binary | `anyhow` allowed only at the entry point |
| Test code | `.unwrap()` allowed inside `#[test]` blocks only |

All handlers return `Result<Json<ApiResponse<T>>, ApiError>`.

## API Response Format

All API responses use a consistent envelope:

```json
{
  "success": true,
  "message": "Operation successful",
  "data": { ... }
}
```

Error responses:

```json
{
  "success": false,
  "error_code": "invalid_credentials",
  "message": "Wrong email or password",
  "data": null
}
```

Paginated responses add metadata:

```json
{
  "success": true,
  "message": "Items retrieved",
  "data": {
    "data": [ ... ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8
    },
    "filters": { ... }
  }
}
```

## Database Query Rules

1. **Always use parameterized queries** — never string interpolation
2. **Batch fetch related entities** — avoid N+1 queries with `SELECT * FROM table WHERE id IN $ids`
3. **Use `libs/data::SurrealClient`** methods — never raw queries in controllers

## Naming Conventions

| Item | Convention | Example |
|------|-----------|---------|
| Crate names | `sakaloka-{name}` | `sakaloka-api`, `sakaloka-secure` |
| Module files | `snake_case.rs` | `user_claims.rs` |
| Struct names | `PascalCase` | `UserClaims`, `ApiResponse` |
| Function names | `snake_case` | `find_user_by_email` |
| Constants | `SCREAMING_SNAKE_CASE` | `DEFAULT_TOKEN_TTL` |
| Route paths | `kebab-case` | `/api/pos/orders/active` |
| SurrealDB tables | `snake_case` | `order_item`, `stock_movement` |

## Controller Pattern

Each controller is a **directory module** with separate files for routes, handlers, and optional helpers:

```
controllers/
└── item/
    ├── mod.rs       # Re-exports routes (pub use routes::routes)
    ├── routes.rs    # Route definitions with RBAC layers
    ├── handlers.rs  # Handler functions
    └── helpers.rs   # (optional) Domain-specific helpers
```

**`routes.rs`** — defines the Axum router:

```rust
pub fn routes() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/items", get(handlers::list))
        .route_layer(RequireScope::new(Scope::EntityRead));
    let write_routes = Router::new()
        .route("/items", post(handlers::create))
        .route_layer(RequireScope::new(Scope::EntityWrite));
    Router::new().merge(read_routes).merge(write_routes)
}
```

**`handlers.rs`** — handler functions:

```rust
pub async fn list(
    State(state): State<AppState>,
    Extension(claims): Extension<UserClaims>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<ItemResponse>>, ApiError> {
    let org_id = crate::helpers::org_resolver::resolve_caller_org(&state, &claims.sub).await?;
    // Call state.db methods, return ApiResponse
}
```

### Shared Helpers (`src/helpers/`)

| Helper | Purpose |
|--------|---------|
| `org_resolver` | `resolve_caller_org(state, user_id)` — extracts caller's org ID from JWT claims |
| `error_map` | `db_err(err, context)` — maps DB errors to `ApiError::Internal` with tracing |
| `patch_builder` | `PatchBuilder` — fluent builder for partial-update JSON payloads |

## Documentation Rules

Every public function must have:
- A `///` doc comment describing its purpose
- A `# Errors` section if it returns `Result`
- A `# Examples` section with a working doctest

CI enforces 100% doc coverage with: `RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps`
