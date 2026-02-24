-- SCHEMAFULL table for tracking active user sessions
DEFINE TABLE session SCHEMAFULL;

-- Relational link to the user
DEFINE FIELD user_id ON session TYPE record<user>;

-- Metadata for audit trails
DEFINE FIELD user_agent ON session TYPE option<string>;
DEFINE FIELD ip_address ON session TYPE option<string>;

-- Timestamps
DEFINE FIELD created_at ON session TYPE datetime DEFAULT time::now();
DEFINE FIELD expires_at ON session TYPE datetime;

-- Index for querying active sessions per user
DEFINE INDEX session_user_idx ON session COLUMNS user_id;
