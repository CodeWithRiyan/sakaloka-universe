-- SCHEMAFULL table for order line items
DEFINE TABLE order_item SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Link to the parent order (required)
DEFINE FIELD order_id ON order_item TYPE record<order>;

-- Link to the product (required)
DEFINE FIELD product_id ON order_item TYPE record<product>;

-- Snapshot of the product name at time of order (denormalized for history)
DEFINE FIELD item_name ON order_item TYPE string ASSERT string::len($value) > 0;

-- Quantity ordered (must be at least 1)
DEFINE FIELD quantity ON order_item TYPE int ASSERT $value >= 1;

-- Unit price in smallest currency unit at time of order
DEFINE FIELD unit_price ON order_item TYPE int ASSERT $value >= 0;

-- Discount applied to this line item
DEFINE FIELD discount_amount ON order_item TYPE int DEFAULT 0;

-- Total price for this line (quantity * unit_price - discount_amount)
DEFINE FIELD total_price ON order_item TYPE int ASSERT $value >= 0;

-- Optional item-level notes (e.g. "no ice", "extra spicy")
DEFINE FIELD notes ON order_item TYPE option<string>;

-- Timestamp
DEFINE FIELD created_at ON order_item TYPE datetime DEFAULT time::now();

-- Lookup indices
DEFINE INDEX order_item_order_idx ON order_item COLUMNS order_id;
DEFINE INDEX order_item_product_idx ON order_item COLUMNS product_id;
