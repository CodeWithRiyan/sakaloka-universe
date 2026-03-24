-- SCHEMAFULL table for product categories
DEFINE TABLE category SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Category name (required, non-empty)
DEFINE FIELD name ON category TYPE string ASSERT string::len($value) > 0;

-- URL-friendly slug (required, non-empty)
DEFINE FIELD slug ON category TYPE string ASSERT string::len($value) > 0;

-- Optional description
DEFINE FIELD description ON category TYPE option<string>;

-- Self-referencing parent for nested categories
DEFINE FIELD parent_id ON category TYPE option<record<category>>;

-- Optional category image URL
DEFINE FIELD image_url ON category TYPE option<string>;

-- Display sort order
DEFINE FIELD sort_order ON category TYPE int DEFAULT 0;

-- Whether the category is active
DEFINE FIELD is_active ON category TYPE bool DEFAULT true;

-- Owning organization (required)
DEFINE FIELD organization_id ON category TYPE record<organization>;

-- User who created this category
DEFINE FIELD created_by ON category TYPE option<record<user>>;

-- Timestamps
DEFINE FIELD created_at ON category TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON category TYPE datetime DEFAULT time::now();

-- Slug must be unique within an organization
DEFINE INDEX category_slug_org_idx ON category COLUMNS slug, organization_id UNIQUE;

-- Lookup indices
DEFINE INDEX category_organization_idx ON category COLUMNS organization_id;
DEFINE INDEX category_parent_idx ON category COLUMNS parent_id;
