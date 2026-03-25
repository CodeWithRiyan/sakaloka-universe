# Earth API

Earth is the Axum 0.8 REST API that serves as the entry point for all client requests.
All routes are nested under `/api` except the health check.

## Route Map

### Public Routes (No Authentication)

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET | `/health` | `health` | Health check |
| POST | `/api/auth/login` | `login` | Authenticate and get tokens |
| POST | `/api/auth/register` | `register` | Create account + organization |
| POST | `/api/auth/refresh` | `refresh` | Exchange refresh token (501) |

### Protected Routes (JWT Required)

All protected routes require `Authorization: Bearer <token>` header.

#### Auth

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/auth/profile` | *(any valid token)* | Get current user profile |

#### Products

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/products` | `entity:read` | List products (paginated) |
| GET | `/api/products/{id}` | `entity:read` | Get product by ID |
| POST | `/api/products` | `entity:write` | Create product |
| PATCH | `/api/products/{id}` | `entity:write` | Update product |
| DELETE | `/api/products/{id}` | `entity:delete` | Delete product |

#### Brands

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/products/brands` | *(any valid token)* | List brands |
| GET | `/api/products/brands/{id}` | *(any valid token)* | Get brand by ID |
| POST | `/api/products/brands` | `entity:write` | Create brand |
| PATCH | `/api/products/brands/{id}` | `entity:write` | Update brand |
| DELETE | `/api/products/brands/{id}` | `entity:delete` | Delete brand |

#### Categories

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/products/categories` | *(any valid token)* | List categories |
| GET | `/api/products/categories/{id}` | *(any valid token)* | Get category by ID |
| POST | `/api/products/categories` | `entity:write` | Create category |
| PATCH | `/api/products/categories/{id}` | `entity:write` | Update category |
| DELETE | `/api/products/categories/{id}` | `entity:delete` | Delete category |

#### Users

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/users` | `user:read` | List users |
| GET | `/api/users/{id}` | `user:read` | Get user by ID |
| POST | `/api/users` | `user:manage` | Create user |
| PATCH | `/api/users/{id}` | `user:manage` | Update user |
| DELETE | `/api/users/{id}` | `user:manage` | Delete user |

#### Roles

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/roles` | `user:read` | List roles |
| GET | `/api/roles/permissions` | *(any valid token)* | Get permissions matrix |
| GET | `/api/roles/{id}` | `user:read` | Get role by ID |
| POST | `/api/roles` | `user:manage` | Create role |
| PATCH | `/api/roles/{id}` | `user:manage` | Update role |
| DELETE | `/api/roles/{id}` | `user:manage` | Delete role |

#### Organizations

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/organizations` | `entity:read` | List organizations |
| GET | `/api/organizations/current` | `entity:read` | Get current org |
| GET | `/api/organizations/{id}` | `entity:read` | Get org by ID |
| POST | `/api/organizations` | `entity:write` | Create organization |
| POST | `/api/organizations/select` | `entity:write` | Switch active org |
| PATCH | `/api/organizations/{id}` | `entity:write` | Update organization |
| DELETE | `/api/organizations/{id}` | `entity:write` | Delete organization |

#### POS (Point of Sale)

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/pos/menu` | *(any valid token)* | Menu items for POS screen |
| GET | `/api/pos/orders` | *(any valid token)* | List all orders |
| GET | `/api/pos/orders/active` | *(any valid token)* | Active (non-completed) orders |
| GET | `/api/pos/orders/history` | *(any valid token)* | Completed/cancelled orders |
| GET | `/api/pos/orders/{id}` | *(any valid token)* | Get order with items |
| POST | `/api/pos/orders` | `entity:write` | Create new order |
| PATCH | `/api/pos/orders/{id}` | `entity:write` | Update order status/payment |

#### Inventory

| Method | Path | Scope | Description |
|--------|------|-------|-------------|
| GET | `/api/inventory/pos-stock` | *(any valid token)* | List inventory |
| GET | `/api/inventory/pos-stock/low-stock` | *(any valid token)* | Low stock items |
| GET | `/api/inventory/pos-stock/{id}` | *(any valid token)* | Get inventory item |
| GET | `/api/inventory/pos-stock/{id}/history` | *(any valid token)* | Stock movement history |
| POST | `/api/inventory/pos-stock/products/{product_id}/adjust` | `entity:write` | Adjust stock |

## Authentication

Include the JWT in the `Authorization` header:

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
```

## Pagination

List endpoints accept query parameters:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `page` | `1` | Page number (1-indexed) |
| `limit` | `20` | Items per page (max 100) |
| `search` | *(none)* | Full-text search filter |
| `sort_by` | varies | Sort field name |
| `sort_desc` | `false` | Sort descending |

## Error Codes

| HTTP Status | `ApiError` Variant | When |
|-------------|-------------------|------|
| 400 | `BadRequest` | Invalid input or missing required field |
| 401 | `Unauthorized` | Missing/invalid/expired JWT |
| 403 | *(RequireScope)* | Valid JWT but insufficient scope |
| 404 | `NotFound` | Resource does not exist |
| 500 | `Internal` | Server error (logged, details hidden) |
| 501 | `NotImplemented` | Feature not yet available (refresh endpoint) |
