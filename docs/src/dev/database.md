# Database Schema

Sakaloka-Universe uses **SurrealDB v3.0.4** as its primary data store (Jupiter).
The schema is SCHEMAFULL and defined via SurrealQL migration files.

## Connection

The API connects via WebSocket:

```
ws://127.0.0.1:58000  (local development)
ws://sakaloka-surrealdb:8000  (Docker network)
```

## Migration Files

Migrations live in `libs/data/surql/` and are applied in alphabetical order at startup:

| File | Table | Description |
|------|-------|-------------|
| `00_init.surql` | *(setup)* | Database initialization, namespace, and database selection |
| `01_user.surql` | `user` | User accounts with email, password hash, org/role refs |
| `02_session.surql` | `session` | Active sessions linked to users |
| `03_refresh_token.surql` | `refresh_token` | Hashed refresh tokens for rotation |
| `04_jti_blocklist.surql` | `jti_blocklist` | Revoked JWT token IDs |
| `05_token.surql` | `token` | Service token storage |
| `06_product.surql` | `product` | Product catalog |
| `07_organization.surql` | `organization` | Multi-tenant organizations |
| `08_role.surql` | `role` | RBAC roles with JSON permissions |
| `09_category.surql` | `category` | Product categories (hierarchical) |
| `10_brand.surql` | `brand` | Product brands |
| `11_order.surql` | `order` | POS order headers |
| `12_order_item.surql` | `order_item` | POS order line items |
| `13_inventory.surql` | `inventory` | Stock levels per product per org |

## Key Tables

### `user`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `record(user)` | SurrealDB record ID |
| `email` | `string` | Unique email (lowercase) |
| `full_name` | `option<string>` | Display name |
| `password_hash` | `string` | Argon2id PHC hash |
| `organization_id` | `option<record(organization)>` | Belongs to org |
| `role_id` | `option<record(role)>` | Assigned role |
| `is_active` | `bool` | Account active flag |
| `last_login_at` | `option<datetime>` | Last login timestamp |
| `created_at` | `datetime` | Creation timestamp |

### `order`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `record(order)` | SurrealDB record ID |
| `order_number` | `string` | Unique: `ORD-YYYYMMDD-XXXXXXXX` |
| `organization_id` | `string` | Owning organization |
| `order_type` | `string` | `dine_in`, `takeaway`, `delivery` |
| `status` | `string` | `pending`, `preparing`, `ready`, `completed`, `cancelled` |
| `subtotal` | `int` | Sum of item prices (cents) |
| `tax_amount` | `int` | Tax (10% of subtotal) |
| `total_amount` | `int` | subtotal + tax |
| `payment_method` | `option<string>` | `cash`, `card`, `qris` |
| `payment_status` | `string` | `unpaid`, `paid`, `refunded` |

### `product`

| Field | Type | Description |
|-------|------|-------------|
| `id` | `record(product)` | SurrealDB record ID |
| `name` | `string` | Product name |
| `sku` | `string` | Stock keeping unit |
| `base_price` | `int` | Price in cents |
| `category_id` | `option<record(category)>` | Category reference |
| `brand_id` | `option<record(brand)>` | Brand reference |
| `organization_id` | `string` | Owning organization |
| `track_inventory` | `bool` | Whether stock is tracked |
| `is_featured` | `bool` | Featured on POS menu |

## Applying Migrations

Migrations are automatically applied at API startup in `app::run_migrations()`.

The migration directory is resolved from:
1. `MIGRATIONS_DIR` environment variable (Docker)
2. `CARGO_MANIFEST_DIR/../../libs/data/surql/` fallback (local dev)

## Query Client

All database operations go through `libs/data::surreal::SurrealClient`. Key method groups:

- **User**: `find_user_by_email`, `find_user_by_id`, `create_user`, `update_user`, `delete_user`
- **Product**: `find_product`, `find_products_by_ids`, `list_products`, `create_product`, `update_product`
- **Order**: `find_order`, `list_orders`, `create_order`, `update_order`, `list_order_items`, `create_order_item`
- **Organization**: `find_organization`, `create_organization`, `update_organization_owner`
- **Role**: `find_role`, `create_role`, `update_role`
- **Category/Brand**: `find_category`, `find_categories_by_ids`, `find_brand`, `find_brands_by_ids`
- **Inventory**: `find_inventory_item`, `list_low_stock`, `update_inventory_stock`, `create_stock_movement`
- **Session**: `create_session`
