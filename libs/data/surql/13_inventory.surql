-- ============================================================
-- SCHEMAFULL tables for inventory management
-- ============================================================

-- ------------------------------------------------------------
-- inventory_item: tracks current stock levels per product per org
-- ------------------------------------------------------------
DEFINE TABLE inventory_item SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Link to the product (required)
DEFINE FIELD product_id ON inventory_item TYPE record<product>;

-- Owning organization (required)
DEFINE FIELD organization_id ON inventory_item TYPE record<organization>;

-- Optional SKU override at the inventory level
DEFINE FIELD sku ON inventory_item TYPE option<string>;

-- Storage location identifier (e.g. shelf, bin, warehouse zone)
DEFINE FIELD location ON inventory_item TYPE option<string>;

-- Current stock quantities
DEFINE FIELD quantity_on_hand ON inventory_item TYPE int DEFAULT 0;
DEFINE FIELD quantity_reserved ON inventory_item TYPE int DEFAULT 0;
DEFINE FIELD quantity_available ON inventory_item TYPE int DEFAULT 0;

-- Stock level thresholds for alerts and reordering
DEFINE FIELD min_stock_level ON inventory_item TYPE int DEFAULT 0;
DEFINE FIELD max_stock_level ON inventory_item TYPE option<int>;
DEFINE FIELD reorder_point ON inventory_item TYPE option<int>;

-- Timestamps
DEFINE FIELD created_at ON inventory_item TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON inventory_item TYPE datetime DEFAULT time::now();

-- Each product can only have one inventory record per organization
DEFINE INDEX inventory_item_product_org_idx ON inventory_item COLUMNS product_id, organization_id UNIQUE;

-- Lookup indices
DEFINE INDEX inventory_item_organization_idx ON inventory_item COLUMNS organization_id;

-- ------------------------------------------------------------
-- stock_movement: immutable audit log of inventory changes
-- ------------------------------------------------------------
DEFINE TABLE stock_movement SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Link to the inventory item (required)
DEFINE FIELD inventory_item_id ON stock_movement TYPE record<inventory_item>;

-- Type of movement (e.g. "purchase", "sale", "adjustment", "transfer", "return")
DEFINE FIELD movement_type ON stock_movement TYPE string ASSERT string::len($value) > 0;

-- Quantity moved (positive for inbound, negative for outbound)
DEFINE FIELD quantity ON stock_movement TYPE int;

-- Optional reference to the source document (e.g. "order", "purchase_order")
DEFINE FIELD reference_type ON stock_movement TYPE option<string>;

-- Optional ID of the referenced document
DEFINE FIELD reference_id ON stock_movement TYPE option<string>;

-- Optional notes
DEFINE FIELD notes ON stock_movement TYPE option<string>;

-- User who performed this movement
DEFINE FIELD created_by ON stock_movement TYPE option<record<user>>;

-- Timestamp (immutable — movements are append-only)
DEFINE FIELD created_at ON stock_movement TYPE datetime DEFAULT time::now();

-- Lookup indices
DEFINE INDEX stock_movement_item_idx ON stock_movement COLUMNS inventory_item_id;
DEFINE INDEX stock_movement_type_idx ON stock_movement COLUMNS movement_type;
DEFINE INDEX stock_movement_reference_idx ON stock_movement COLUMNS reference_type, reference_id;
