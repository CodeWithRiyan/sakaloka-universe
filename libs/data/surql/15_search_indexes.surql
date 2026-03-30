-- Standard indexes for search performance.
--
-- These speed up the `CONTAINS` substring search pattern used in list queries.
-- Even without full-text search indexes, standard indexes on name fields help
-- SurrealDB narrow down candidates before applying the CONTAINS filter.
--
-- Note: Full-text search (SEARCH ANALYZER / @@) requires SurrealDB server
-- and is not available in kv-mem mode. For now, we use standard indexes.
DEFINE INDEX IF NOT EXISTS product_name_idx ON product FIELDS name;
DEFINE INDEX IF NOT EXISTS product_sku_idx ON product FIELDS sku;
DEFINE INDEX IF NOT EXISTS category_name_idx ON category FIELDS name;
DEFINE INDEX IF NOT EXISTS brand_name_idx ON brand FIELDS name;
DEFINE INDEX IF NOT EXISTS organization_name_idx ON organization FIELDS name;
DEFINE INDEX IF NOT EXISTS user_email_idx ON user FIELDS email;
DEFINE INDEX IF NOT EXISTS user_fullname_idx ON user FIELDS full_name;
DEFINE INDEX IF NOT EXISTS role_name_idx ON role FIELDS name;
