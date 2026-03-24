-- SCHEMAFULL table for Sakaloka users
DEFINE TABLE user SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Username (optional, between 3 and 128 chars when present)
DEFINE FIELD username ON user TYPE option<string> ASSERT $value IS NONE OR (string::len($value) >= 3 AND string::len($value) <= 128);

-- Valid email required
DEFINE FIELD email ON user TYPE string ASSERT string::is_email($value);

-- Full display name (optional)
DEFINE FIELD full_name ON user TYPE option<string>;

-- Must be a valid Argon2id PHC string
DEFINE FIELD password_hash ON user TYPE string ASSERT string::starts_with($value, '$argon2id$');

-- Link to the user's organization
DEFINE FIELD organization_id ON user TYPE option<record<organization>>;

-- Link to the user's role
DEFINE FIELD role_id ON user TYPE option<record<role>>;

-- Account status
DEFINE FIELD is_active ON user TYPE bool DEFAULT true;

-- Last login tracking
DEFINE FIELD last_login_at ON user TYPE option<datetime>;

-- Timestamps
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();

-- Unique constraints
DEFINE INDEX user_username_idx ON user COLUMNS username UNIQUE;
DEFINE INDEX user_email_idx ON user COLUMNS email UNIQUE;

-- Lookup indices
DEFINE INDEX user_organization_idx ON user COLUMNS organization_id;
DEFINE INDEX user_role_idx ON user COLUMNS role_id;
