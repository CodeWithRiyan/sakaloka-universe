-- SCHEMAFULL table for Sakaloka users
DEFINE TABLE user SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Username must be between 3 and 128 chars
DEFINE FIELD username ON user TYPE string ASSERT string::len($value) >= 3 AND string::len($value) <= 128;

-- Valid email required
DEFINE FIELD email_address ON user TYPE string ASSERT string::is_email($value);

-- Must be a valid Argon2id PHC string
DEFINE FIELD password_hash ON user TYPE string ASSERT string::starts_with($value, '$argon2id$');

-- Role must be one of the known enums in RBAC matrix
DEFINE FIELD role ON user TYPE string ASSERT $value IN ['admin', 'editor', 'viewer'];

-- Timestamps
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();

-- Unique constraints
DEFINE INDEX user_username_idx ON user COLUMNS username UNIQUE;
DEFINE INDEX user_email_idx ON user COLUMNS email_address UNIQUE;
