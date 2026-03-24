-- SCHEMAFULL table for product brands
DEFINE TABLE brand SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Brand name (required, non-empty)
DEFINE FIELD name ON brand TYPE string ASSERT string::len($value) > 0;

-- URL-friendly slug (required, non-empty)
DEFINE FIELD slug ON brand TYPE string ASSERT string::len($value) > 0;

-- Optional description
DEFINE FIELD description ON brand TYPE option<string>;

-- Optional brand logo URL
DEFINE FIELD logo ON brand TYPE option<string>;

-- Optional brand website
DEFINE FIELD website ON brand TYPE option<string>;

-- Whether the brand is active
DEFINE FIELD is_active ON brand TYPE bool DEFAULT true;

-- Owning organization (required)
DEFINE FIELD organization_id ON brand TYPE record<organization>;

-- User who created this brand
DEFINE FIELD created_by ON brand TYPE option<record<user>>;

-- Timestamps
DEFINE FIELD created_at ON brand TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON brand TYPE datetime DEFAULT time::now();

-- Slug must be unique within an organization
DEFINE INDEX brand_slug_org_idx ON brand COLUMNS slug, organization_id UNIQUE;

-- Lookup index
DEFINE INDEX brand_organization_idx ON brand COLUMNS organization_id;
