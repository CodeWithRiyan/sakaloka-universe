-- SCHEMAFULL table for products
DEFINE TABLE product SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Product name (required, non-empty)
DEFINE FIELD name ON product TYPE string ASSERT string::len($value) > 0;

-- Optional product description
DEFINE FIELD description ON product TYPE option<string>;

-- SKU must be non-empty, unique per organization
DEFINE FIELD sku ON product TYPE string ASSERT string::len($value) > 0;

-- Optional barcode (e.g. UPC, EAN)
DEFINE FIELD barcode ON product TYPE option<string>;

-- Base selling price in smallest currency unit (must be >= 0)
DEFINE FIELD base_price ON product TYPE int ASSERT $value >= 0;

-- Optional cost/purchase price in smallest currency unit
DEFINE FIELD cost_price ON product TYPE option<int>;

-- Link to category
DEFINE FIELD category_id ON product TYPE option<record<category>>;

-- Link to brand
DEFINE FIELD brand_id ON product TYPE option<record<brand>>;

-- Optional product image URL
DEFINE FIELD image_url ON product TYPE option<string>;

-- Optional weight in kg
DEFINE FIELD weight ON product TYPE option<float>;

-- Optional dimensions object (e.g. { length: 10, width: 5, height: 3, unit: "cm" })
DEFINE FIELD dimensions ON product TYPE option<object>;

-- Whether inventory tracking is enabled for this product
DEFINE FIELD track_inventory ON product TYPE bool DEFAULT true;

-- Minimum stock level before low-stock alerts
DEFINE FIELD min_stock_level ON product TYPE int DEFAULT 0;

-- Whether the product is featured/promoted
DEFINE FIELD is_featured ON product TYPE bool DEFAULT false;

-- Optional tags for search and filtering
DEFINE FIELD tags ON product TYPE option<array>;

-- Owning organization (required)
DEFINE FIELD organization_id ON product TYPE record<organization>;

-- User who created this product
DEFINE FIELD created_by ON product TYPE option<record<user>>;

-- Soft-delete timestamp
DEFINE FIELD deleted_at ON product TYPE option<datetime>;

-- Timestamps
DEFINE FIELD created_at ON product TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON product TYPE datetime DEFAULT time::now();

-- SKU must be unique within an organization
DEFINE INDEX product_sku_org_idx ON product COLUMNS sku, organization_id UNIQUE;

-- Lookup indices
DEFINE INDEX product_organization_idx ON product COLUMNS organization_id;
DEFINE INDEX product_category_idx ON product COLUMNS category_id;
DEFINE INDEX product_brand_idx ON product COLUMNS brand_id;
DEFINE INDEX product_barcode_idx ON product COLUMNS barcode;
