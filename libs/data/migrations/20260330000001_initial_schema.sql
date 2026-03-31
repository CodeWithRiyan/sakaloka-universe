-- Migration: 20260330000001_initial_schema
-- Description: Initial PostgreSQL schema for Sakaloka Universe
-- Created: 2026-03-30

-- ============================================================================
-- Extensions
-- ============================================================================

CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- ============================================================================
-- Auto-update trigger function for updated_at
-- ============================================================================

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- 1. organizations
-- ============================================================================

CREATE TABLE organizations (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    type        TEXT NOT NULL DEFAULT 'company',
    code        TEXT,
    description TEXT,
    parent_id   UUID REFERENCES organizations(id),
    email       TEXT,
    phone       TEXT,
    website     TEXT,
    address     TEXT,
    city        TEXT,
    state       TEXT,
    country     TEXT,
    postal_code TEXT,
    tax_number  TEXT,
    registration_number TEXT,
    logo        TEXT,
    settings    JSONB,
    is_active   BOOLEAN DEFAULT true,
    owner_id    UUID,  -- no FK yet (circular with users)
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_organizations_code ON organizations(code) WHERE code IS NOT NULL;

CREATE TRIGGER trg_organizations_updated_at
    BEFORE UPDATE ON organizations
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 2. roles
-- ============================================================================

CREATE TABLE roles (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    description     TEXT,
    permissions     JSONB,
    organization_id UUID NOT NULL REFERENCES organizations(id),
    is_system_role  BOOLEAN DEFAULT false,
    is_active       BOOLEAN DEFAULT true,
    created_by      UUID,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (name, organization_id)
);

CREATE TRIGGER trg_roles_updated_at
    BEFORE UPDATE ON roles
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 3. users
-- ============================================================================

CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username        TEXT,
    email           TEXT NOT NULL UNIQUE,
    full_name       TEXT,
    password_hash   TEXT NOT NULL,
    organization_id UUID REFERENCES organizations(id),
    role_id         UUID REFERENCES roles(id),
    is_active       BOOLEAN DEFAULT true,
    last_login_at   TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_org ON users(organization_id);

CREATE TRIGGER trg_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 4. sessions
-- ============================================================================

CREATE TABLE sessions (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_sessions_token ON sessions(token_hash);

-- ============================================================================
-- 5. brands
-- ============================================================================

CREATE TABLE brands (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    slug            TEXT NOT NULL,
    description     TEXT,
    logo            TEXT,
    website         TEXT,
    is_active       BOOLEAN DEFAULT true,
    organization_id UUID NOT NULL REFERENCES organizations(id),
    created_by      UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (slug, organization_id)
);

CREATE INDEX idx_brands_org ON brands(organization_id);

CREATE TRIGGER trg_brands_updated_at
    BEFORE UPDATE ON brands
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 6. categories
-- ============================================================================

CREATE TABLE categories (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    slug            TEXT NOT NULL,
    description     TEXT,
    parent_id       UUID REFERENCES categories(id),
    image_url       TEXT,
    sort_order      BIGINT DEFAULT 0,
    is_active       BOOLEAN DEFAULT true,
    organization_id UUID NOT NULL REFERENCES organizations(id),
    created_by      UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (slug, organization_id)
);

CREATE INDEX idx_categories_org ON categories(organization_id);

CREATE TRIGGER trg_categories_updated_at
    BEFORE UPDATE ON categories
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 7. products
-- ============================================================================

CREATE TABLE products (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    description     TEXT,
    sku             TEXT NOT NULL,
    barcode         TEXT,
    base_price      BIGINT NOT NULL DEFAULT 0,
    cost_price      BIGINT,
    category_id     UUID REFERENCES categories(id),
    brand_id        UUID REFERENCES brands(id),
    image_url       TEXT,
    weight          DOUBLE PRECISION,
    dimensions      JSONB,
    track_inventory BOOLEAN DEFAULT true,
    min_stock_level BIGINT DEFAULT 0,
    is_featured     BOOLEAN DEFAULT false,
    tags            TEXT[],
    organization_id UUID NOT NULL REFERENCES organizations(id),
    created_by      UUID REFERENCES users(id),
    deleted_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (sku, organization_id)
);

CREATE INDEX idx_products_org   ON products(organization_id);
CREATE INDEX idx_products_cat   ON products(category_id);
CREATE INDEX idx_products_brand ON products(brand_id);

CREATE TRIGGER trg_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 8. orders
-- ============================================================================

CREATE TABLE orders (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_number    TEXT NOT NULL UNIQUE,
    customer_id     UUID REFERENCES users(id),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    status          TEXT NOT NULL DEFAULT 'pending',
    type            TEXT NOT NULL DEFAULT 'dine_in',
    subtotal        BIGINT DEFAULT 0,
    tax_amount      BIGINT DEFAULT 0,
    discount_amount BIGINT DEFAULT 0,
    total_amount    BIGINT DEFAULT 0,
    payment_method  TEXT,
    payment_status  TEXT NOT NULL DEFAULT 'unpaid',
    paid_amount     BIGINT DEFAULT 0,
    notes           TEXT,
    table_number    TEXT,
    customer_name   TEXT,
    created_by      UUID REFERENCES users(id),
    updated_by      UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_orders_status CHECK (status IN ('pending', 'confirmed', 'preparing', 'ready', 'completed', 'cancelled')),
    CONSTRAINT chk_orders_type   CHECK (type IN ('dine_in', 'takeaway', 'delivery'))
);

CREATE INDEX idx_orders_org     ON orders(organization_id);
CREATE INDEX idx_orders_status  ON orders(status);
CREATE INDEX idx_orders_created ON orders(created_at DESC);

CREATE TRIGGER trg_orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 9. order_items
-- ============================================================================

CREATE TABLE order_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id        UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id      UUID NOT NULL REFERENCES products(id),
    item_name       TEXT NOT NULL,
    quantity        INTEGER NOT NULL DEFAULT 1,
    unit_price      BIGINT NOT NULL DEFAULT 0,
    discount_amount BIGINT DEFAULT 0,
    total_price     BIGINT NOT NULL DEFAULT 0,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_items_order ON order_items(order_id);

-- ============================================================================
-- 10. inventory_items
-- ============================================================================

CREATE TABLE inventory_items (
    id                 UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id         UUID NOT NULL REFERENCES products(id),
    organization_id    UUID NOT NULL REFERENCES organizations(id),
    sku                TEXT,
    location           TEXT,
    quantity_on_hand   INTEGER DEFAULT 0,
    quantity_reserved  INTEGER DEFAULT 0,
    quantity_available INTEGER DEFAULT 0,
    min_stock_level    INTEGER DEFAULT 0,
    max_stock_level    INTEGER,
    reorder_point      INTEGER,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (product_id, organization_id)
);

CREATE INDEX idx_inventory_org ON inventory_items(organization_id);

CREATE TRIGGER trg_inventory_items_updated_at
    BEFORE UPDATE ON inventory_items
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- ============================================================================
-- 11. stock_movements
-- ============================================================================

CREATE TABLE stock_movements (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inventory_item_id UUID NOT NULL REFERENCES inventory_items(id),
    movement_type     TEXT NOT NULL,
    quantity          INTEGER NOT NULL,
    reference_type    TEXT,
    reference_id      TEXT,
    notes             TEXT,
    created_by        UUID REFERENCES users(id),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_stock_movements_type CHECK (movement_type IN ('purchase', 'sale', 'adjustment', 'damage', 'return'))
);

CREATE INDEX idx_stock_movements_item ON stock_movements(inventory_item_id);

-- ============================================================================
-- 12. schema_migrations
-- ============================================================================

CREATE TABLE schema_migrations (
    name       TEXT PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Trigram indexes (pg_trgm) for fuzzy / LIKE search
-- ============================================================================

CREATE INDEX idx_trgm_products_name      ON products      USING gin (name gin_trgm_ops);
CREATE INDEX idx_trgm_products_sku       ON products      USING gin (sku gin_trgm_ops);
CREATE INDEX idx_trgm_users_email        ON users         USING gin (email gin_trgm_ops);
CREATE INDEX idx_trgm_users_full_name    ON users         USING gin (full_name gin_trgm_ops);
CREATE INDEX idx_trgm_organizations_name ON organizations USING gin (name gin_trgm_ops);
CREATE INDEX idx_trgm_brands_name        ON brands        USING gin (name gin_trgm_ops);
CREATE INDEX idx_trgm_categories_name    ON categories    USING gin (name gin_trgm_ops);

-- ============================================================================
-- Record this migration
-- ============================================================================

INSERT INTO schema_migrations (name) VALUES ('20260330000001_initial_schema');
