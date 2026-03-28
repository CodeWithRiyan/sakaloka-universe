-- SCHEMAFULL table for roles (RBAC)
DEFINE TABLE role SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Role name (required, non-empty)
DEFINE FIELD name ON role TYPE string ASSERT string::len($value) > 0;

-- Optional description of what this role grants
DEFINE FIELD description ON role TYPE option<string>;

-- Permissions object (e.g. { product: ['read', 'update'], organization: ['read', 'select'] })
DEFINE FIELD permissions ON role TYPE option<object> FLEXIBLE;

-- Owning organization (required)
DEFINE FIELD organization_id ON role TYPE record<organization>;

-- Whether this is a system-defined role (cannot be deleted by users)
DEFINE FIELD is_system_role ON role TYPE bool DEFAULT false;

-- Whether the role is active
DEFINE FIELD is_active ON role TYPE bool DEFAULT true;

-- User who created this role
DEFINE FIELD created_by ON role TYPE option<record<user>>;

-- Timestamps
DEFINE FIELD created_at ON role TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON role TYPE datetime DEFAULT time::now();

-- Role name must be unique within an organization
DEFINE INDEX role_name_org_idx ON role COLUMNS name, organization_id UNIQUE;

-- Lookup index
DEFINE INDEX role_organization_idx ON role COLUMNS organization_id;
