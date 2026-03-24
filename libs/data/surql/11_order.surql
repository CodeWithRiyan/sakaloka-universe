-- SCHEMAFULL table for orders (POS transactions)
DEFINE TABLE order SCHEMAFULL
    PERMISSIONS
        FOR select WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:read' IN $token.service_scopes
        FOR create, update, delete WHERE 'sakaloka:jupiter' IN $token.aud AND 'db:write' IN $token.service_scopes;

-- Unique order number (e.g. ORD-20260101-0001)
DEFINE FIELD order_number ON order TYPE string ASSERT string::len($value) > 0;

-- Optional link to a registered customer
DEFINE FIELD customer_id ON order TYPE option<record<user>>;

-- Owning organization (required)
DEFINE FIELD organization_id ON order TYPE record<organization>;

-- Order lifecycle status
DEFINE FIELD status ON order TYPE string DEFAULT 'draft';

-- Order type (dine-in, takeaway, delivery, etc.)
DEFINE FIELD type ON order TYPE string DEFAULT 'dine_in';

-- Monetary amounts in smallest currency unit
DEFINE FIELD subtotal ON order TYPE int DEFAULT 0;
DEFINE FIELD tax_amount ON order TYPE int DEFAULT 0;
DEFINE FIELD discount_amount ON order TYPE int DEFAULT 0;
DEFINE FIELD total_amount ON order TYPE int DEFAULT 0;

-- Payment details
DEFINE FIELD payment_method ON order TYPE option<string>;
DEFINE FIELD payment_status ON order TYPE string DEFAULT 'unpaid';
DEFINE FIELD paid_amount ON order TYPE int DEFAULT 0;

-- Optional notes / special instructions
DEFINE FIELD notes ON order TYPE option<string>;

-- POS-specific fields
DEFINE FIELD table_number ON order TYPE option<string>;
DEFINE FIELD customer_name ON order TYPE option<string>;

-- Audit trail
DEFINE FIELD created_by ON order TYPE option<record<user>>;
DEFINE FIELD updated_by ON order TYPE option<record<user>>;

-- Timestamps
DEFINE FIELD created_at ON order TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON order TYPE datetime DEFAULT time::now();

-- Order number must be globally unique
DEFINE INDEX order_number_idx ON order COLUMNS order_number UNIQUE;

-- Lookup indices
DEFINE INDEX order_organization_idx ON order COLUMNS organization_id;
DEFINE INDEX order_customer_idx ON order COLUMNS customer_id;
DEFINE INDEX order_status_idx ON order COLUMNS status;
DEFINE INDEX order_payment_status_idx ON order COLUMNS payment_status;
