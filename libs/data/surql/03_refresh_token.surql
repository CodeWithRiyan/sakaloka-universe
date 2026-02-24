-- SCHEMAFULL table for secure refresh token rotation
DEFINE TABLE refresh_token SCHEMAFULL;

-- Link to the parent session
DEFINE FIELD session_id ON refresh_token TYPE record<session>;

-- Hash of the refresh token (never store raw!)
DEFINE FIELD token_hash ON refresh_token TYPE string;

-- Expiration of this specific rotation
DEFINE FIELD expires_at ON refresh_token TYPE datetime;

-- If populated, this token was rotated. Reusing it indicates theft!
DEFINE FIELD rotated_at ON refresh_token TYPE option<datetime>;

-- Timestamps
DEFINE FIELD created_at ON refresh_token TYPE datetime DEFAULT time::now();

-- We often query refresh tokens by session
DEFINE INDEX refresh_token_session_idx ON refresh_token COLUMNS session_id;
