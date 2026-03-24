-- SCHEMAFULL table for organizations (companies, branches, warehouses)
DEFINE TABLE organization SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Organization name (required, non-empty)
DEFINE FIELD name ON organization TYPE string ASSERT string::len($value) > 0;

-- Organization type (must be one of the allowed values)
DEFINE FIELD type ON organization TYPE string ASSERT $value IN ['company', 'branch', 'warehouse'];

-- Optional short code identifier
DEFINE FIELD code ON organization TYPE option<string>;

-- Optional description
DEFINE FIELD description ON organization TYPE option<string>;

-- Self-referencing parent for hierarchical org structure
DEFINE FIELD parent_id ON organization TYPE option<record<organization>>;

-- Contact information
DEFINE FIELD email ON organization TYPE option<string>;
DEFINE FIELD phone ON organization TYPE option<string>;
DEFINE FIELD website ON organization TYPE option<string>;

-- Address fields
DEFINE FIELD address ON organization TYPE option<string>;
DEFINE FIELD city ON organization TYPE option<string>;
DEFINE FIELD state ON organization TYPE option<string>;
DEFINE FIELD country ON organization TYPE option<string>;
DEFINE FIELD postal_code ON organization TYPE option<string>;

-- Legal identifiers
DEFINE FIELD tax_number ON organization TYPE option<string>;
DEFINE FIELD registration_number ON organization TYPE option<string>;

-- Optional logo URL
DEFINE FIELD logo ON organization TYPE option<string>;

-- Flexible settings object (e.g. currency, timezone, locale)
DEFINE FIELD settings ON organization TYPE option<object>;

-- Whether the organization is active
DEFINE FIELD is_active ON organization TYPE bool DEFAULT true;

-- Link to the owning user
DEFINE FIELD owner_id ON organization TYPE option<record<user>>;

-- Timestamps
DEFINE FIELD created_at ON organization TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON organization TYPE datetime DEFAULT time::now();

-- Lookup indices
DEFINE INDEX organization_parent_idx ON organization COLUMNS parent_id;
DEFINE INDEX organization_owner_idx ON organization COLUMNS owner_id;
DEFINE INDEX organization_code_idx ON organization COLUMNS code;
