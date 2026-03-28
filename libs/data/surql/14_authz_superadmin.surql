-- Backfill legacy roles that predate permission-based JWT issuance.
LET $admin_permissions = {
  organization: ['read', 'create', 'update', 'delete', 'select'],
  product: ['read', 'create', 'update', 'delete'],
  brand: ['read', 'create', 'update', 'delete'],
  category: ['read', 'create', 'update', 'delete'],
  stock: ['read', 'update'],
  user: ['read', 'create', 'update', 'delete'],
  role: ['read', 'create', 'update', 'delete'],
  pos: ['read', 'create', 'update']
};

LET $editor_permissions = {
  organization: ['read', 'select'],
  product: ['read', 'create', 'update'],
  brand: ['read', 'create', 'update'],
  category: ['read', 'create', 'update'],
  stock: ['read', 'update'],
  pos: ['read', 'create', 'update']
};

LET $viewer_permissions = {
  organization: ['read'],
  product: ['read'],
  brand: ['read'],
  category: ['read'],
  stock: ['read'],
  pos: ['read']
};

LET $superadmin_permissions = {
  organization: ['read', 'create', 'update', 'delete', 'select'],
  product: ['read', 'create', 'update', 'delete'],
  brand: ['read', 'create', 'update', 'delete'],
  category: ['read', 'create', 'update', 'delete'],
  stock: ['read', 'update'],
  user: ['read', 'create', 'update', 'delete'],
  role: ['read', 'create', 'update', 'delete'],
  pos: ['read', 'create', 'update'],
  db: ['read', 'write', 'admin'],
  zenoh: ['publish', 'subscribe']
};

DEFINE FIELD OVERWRITE permissions ON role TYPE option<object> FLEXIBLE;

UPDATE role
SET permissions = $admin_permissions
WHERE permissions = NONE AND string::lowercase(name) IN ['admin', 'owner'];

UPDATE role
SET permissions = $editor_permissions
WHERE permissions = NONE AND string::lowercase(name) IN ['editor', 'manager', 'cashier'];

UPDATE role
SET permissions = $viewer_permissions
WHERE permissions = NONE AND string::lowercase(name) = 'viewer';

-- Seed a deterministic global bootstrap organization, role, and user for dev/admin recovery.
--
-- Email:    superadmin@sakaloka.local
-- Password: sakaloka-dev-01
UPSERT type::record('organization', 'system') MERGE {
  name: 'Sakaloka System',
  type: 'company',
  code: 'SYSTEM',
  description: 'Reserved system organization for global bootstrap access',
  is_active: true
};

UPSERT type::record('role', 'system-superadmin') MERGE {
  name: 'superadmin',
  description: 'Global bootstrap superadmin role',
  organization_id: type::record('organization', 'system'),
  permissions: $superadmin_permissions,
  is_system_role: true,
  is_active: true
};

UPSERT type::record('user', 'superadmin') MERGE {
  email: 'superadmin@sakaloka.local',
  full_name: 'Global Superadmin',
  password_hash: '$argon2id$v=19$m=19456,t=2,p=1$EVOfPDvU0nKV1JCe8Co17w$M+3OAIDWAxKPJOuyNSHtGQ5cwczv4yBvecMpKMWKAi8',
  organization_id: type::record('organization', 'system'),
  role_id: type::record('role', 'system-superadmin'),
  is_active: true
};

UPSERT type::record('organization', 'system') MERGE {
  owner_id: type::record('user', 'superadmin')
};
