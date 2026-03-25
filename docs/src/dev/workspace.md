# Workspace Structure

Sakaloka-Universe is a Cargo workspace monorepo with strict `apps/` and `libs/` separation.

## Directory Layout

```
sakaloka-universe/
├── apps/                     # Runnable services (binary crates)
│   ├── api/                  # Earth — Axum 0.8 REST API
│   │   ├── src/
│   │   │   ├── main.rs       # Entry point, env loading, server startup
│   │   │   ├── app.rs        # Router construction, CORS, migrations
│   │   │   ├── error.rs      # ApiError enum (Internal, NotFound, BadRequest, etc.)
│   │   │   ├── middleware.rs  # auth_middleware (JWT validation)
│   │   │   ├── controllers/  # Route handlers by domain
│   │   │   │   ├── auth.rs        # Login, register, refresh, profile
│   │   │   │   ├── product.rs     # Product CRUD
│   │   │   │   ├── brand.rs       # Brand CRUD
│   │   │   │   ├── category.rs    # Category CRUD
│   │   │   │   ├── user.rs        # User management
│   │   │   │   ├── role.rs        # Role management
│   │   │   │   ├── organization.rs# Organization management
│   │   │   │   ├── pos.rs         # POS: menu, orders
│   │   │   │   └── stock.rs       # Inventory & stock movements
│   │   │   └── views/        # Request/response types (DTOs)
│   │   └── Dockerfile         # Multi-stage Rust build
│   ├── ockam/                # Mars — Ockam secure transport
│   ├── venus/                # Venus — Tauri + SvelteKit desktop
│   │   └── src-tauri/
│   └── firmware/             # Mercury — RTIC bare-metal (no_std)
├── libs/                     # Shared logic (library crates)
│   ├── core/                 # Domain types and constants
│   │   └── src/models/       # User, Product, Order, Organization, Role, etc.
│   ├── secure/               # IAM: auth, JWT, RBAC, newtypes
│   │   └── src/
│   │       ├── argon2.rs     # Password hashing
│   │       ├── jwt/          # Token issuance and validation
│   │       ├── rbac/         # Roles, scopes, RequireScope middleware
│   │       ├── newtypes.rs   # Validated input wrappers
│   │       └── tokens/       # Refresh token rotation
│   ├── data/                 # Database clients
│   │   ├── src/surreal.rs    # SurrealClient (all DB operations)
│   │   └── surql/            # SurrealQL migration files
│   ├── ai/                   # Burn 0.16 inference engine
│   └── hal/                  # Hardware abstraction (no_std)
├── deploy/                   # Server deployment configs
│   ├── docker-compose.*.yml  # Per-environment compose files
│   ├── deploy.sh             # Deployment script
│   └── config/               # Zenoh ACL configs
├── docs/                     # mdBook documentation (this book)
├── docker-compose.yml        # Local development services
├── Cargo.toml                # Workspace root
└── CLAUDE.md                 # AI assistant instructions
```

## Crate Decision Tree

When adding new code, use this decision tree:

| If you need... | Put it in... |
|----------------|-------------|
| Auth / JWT / RBAC / password / tokens | `libs/secure` (never elsewhere) |
| Domain types / constants | `libs/core` |
| Database / messaging / vector queries | `libs/data` |
| AI / embeddings | `libs/ai` |
| HTTP routes / controllers | `apps/api` |
| Hardware / sensor interfaces | `libs/hal` |

## Models in `libs/core`

| Model | File | Description |
|-------|------|-------------|
| `User` | `models/user.rs` | User account with org and role references |
| `Product` | `models/product.rs` | Product catalog item |
| `Organization` | `models/organization.rs` | Multi-tenant organization |
| `Role` | `models/role.rs` | RBAC role with JSON permissions |
| `Category` | `models/category.rs` | Product category (hierarchical) |
| `Brand` | `models/brand.rs` | Product brand |
| `Order` | `models/order.rs` | POS order header |
| `OrderItem` | `models/order.rs` | POS order line item |
| `InventoryItem` | `models/inventory.rs` | Stock level per product per org |
| `StockMovement` | `models/inventory.rs` | Stock adjustment history |
