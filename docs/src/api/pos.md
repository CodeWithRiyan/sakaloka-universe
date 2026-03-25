# POS API

Point of Sale endpoints for menu browsing, order creation, and order management.

All POS routes require a valid JWT. Write operations additionally require `entity:write` scope.

## `GET /api/pos/menu`

List products available for the POS order screen with pagination.

**Scope:** Any valid token

### Query Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `page` | `1` | Page number |
| `limit` | `20` | Items per page |
| `search` | *(none)* | Search by product name |

### Response

```json
{
  "success": true,
  "message": "Menu retrieved",
  "data": {
    "data": [
      {
        "id": "product:01ABC",
        "name": "Espresso",
        "sku": "BEV-001",
        "base_price": 25000,
        "image_url": null,
        "category_id": "category:01DEF",
        "category_name": "Beverages",
        "is_featured": true
      }
    ],
    "pagination": { "page": 1, "limit": 20, "total": 45, "total_pages": 3 },
    "filters": { ... }
  }
}
```

Categories are batch-fetched to avoid N+1 queries.

---

## `GET /api/pos/orders`

List all orders with pagination.

**Scope:** Any valid token

---

## `GET /api/pos/orders/active`

List active orders (excludes `completed` and `cancelled` status).

**Scope:** Any valid token

---

## `GET /api/pos/orders/history`

List completed and cancelled orders.

**Scope:** Any valid token

---

## `GET /api/pos/orders/{id}`

Get a single order with its line items.

**Scope:** Any valid token

### Response

```json
{
  "success": true,
  "message": "Order retrieved",
  "data": {
    "id": "order:01ABC",
    "order_number": "ORD-20260325-a1b2c3d4",
    "order_type": "dine_in",
    "status": "pending",
    "subtotal": 75000,
    "tax_amount": 7500,
    "total_amount": 82500,
    "payment_method": null,
    "payment_status": "unpaid",
    "table_number": "T-05",
    "customer_name": "John",
    "items": [
      {
        "id": "order_item:01XYZ",
        "product_id": "product:01ABC",
        "item_name": "Espresso",
        "quantity": 3,
        "unit_price": 25000,
        "total_price": 75000
      }
    ]
  }
}
```

---

## `POST /api/pos/orders`

Create a new order with line items.

**Scope:** `entity:write`

### Request

```json
{
  "order_type": "dine_in",
  "items": [
    {
      "product_id": "product:01ABC",
      "quantity": 3,
      "item_name": "Espresso"
    }
  ],
  "payment_method": "cash",
  "notes": "Extra hot",
  "table_number": "T-05",
  "customer_name": "John"
}
```

### Pricing Logic

- `unit_price` is taken from `product.base_price` (server-side, not from client)
- `total_price` = `unit_price` x `quantity` per item
- `subtotal` = sum of all item `total_price`
- `tax_amount` = `subtotal` / 10 (10% tax)
- `total_amount` = `subtotal` + `tax_amount`

Products are batch-fetched by ID to avoid N+1 queries.

### Order Number Format

Auto-generated: `ORD-{YYYYMMDD}-{8-char-uuid}`

Example: `ORD-20260325-a1b2c3d4`

---

## `PATCH /api/pos/orders/{id}`

Update an existing order (status, payment, notes, etc.).

**Scope:** `entity:write`

### Request

All fields are optional:

```json
{
  "status": "completed",
  "payment_method": "cash",
  "payment_status": "paid",
  "paid_amount": 82500,
  "notes": "Updated note",
  "table_number": "T-06",
  "customer_name": "Jane"
}
```

Cancelled orders cannot be modified (returns `order_cancelled` error).
