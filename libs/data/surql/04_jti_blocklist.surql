-- SCHEMAFULL table to block revoked JWT identifiers
DEFINE TABLE jti_blocklist SCHEMAFULL;

-- The unique JWT ID string
DEFINE FIELD jti ON jti_blocklist TYPE string;

-- Storing the expiry time explicitly allows cleanup jobs to quickly purge expired JTIs
DEFINE FIELD exp ON jti_blocklist TYPE int;

-- When it was blocked
DEFINE FIELD blocked_at ON jti_blocklist TYPE datetime DEFAULT time::now();

-- Fast lookup for JTI validation
DEFINE INDEX jti_blocklist_jti_idx ON jti_blocklist COLUMNS jti UNIQUE;
