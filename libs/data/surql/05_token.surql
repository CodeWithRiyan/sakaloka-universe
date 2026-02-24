-- ============================================================
-- 🌍 Sakaloka-Universe — SurrealDB Token Definition
-- ============================================================
-- Configures Jupiter to natively validate incoming Service JWTs 
-- from Earth (or other planets) signed with the workspace secret.

-- Use the dev secret by default. 
-- IN PRODUCTION: This file MUST be templated to replace the VALUE string
-- with the actual `SAKALOKA_JWT_SECRET` environment variable.

DEFINE ACCESS jupiter ON NAMESPACE TYPE JWT ALGORITHM HS256 KEY "test-secret-at-least-32-chars-long!";
