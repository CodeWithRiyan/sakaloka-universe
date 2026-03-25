# Auth API

Authentication endpoints for login, registration, token refresh, and profile.

## `POST /api/auth/login`

Authenticate a user and return tokens.

**Authentication:** None required

### Request

```json
{
  "email": "admin@example.com",
  "password": "correct-horse-battery-staple"
}
```

### Response (200 OK)

```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIs...",
    "refresh_token": "550e8400-e29b-41d4-a716-446655440000",
    "user": {
      "id": "user:01JKXYZ",
      "email": "admin@example.com",
      "full_name": "Admin User",
      "organization": {
        "id": "organization:01ABC",
        "name": "My Company",
        "org_type": "company"
      },
      "role": {
        "id": "role:01DEF",
        "name": "admin",
        "permissions": {
          "entity:read": true,
          "entity:write": true,
          "entity:delete": true,
          "user:read": true,
          "user:manage": true
        }
      },
      "preferences": {}
    }
  }
}
```

### Error Responses

| `error_code` | Message | Cause |
|-------------|---------|-------|
| `invalid_credentials` | Wrong email or password | Email not found or password mismatch |
| `account_disabled` | Account is disabled | `user.is_active` is false |
| `incomplete_profile` | No organization/role assigned | User missing org or role reference |

---

## `POST /api/auth/register`

Create a new user, organization, and admin role. Returns tokens immediately.

**Authentication:** None required

### Request

```json
{
  "email": "newuser@example.com",
  "password": "my-secure-password-123",
  "full_name": "New User",
  "organization_name": "New Company"
}
```

### Response (201 Created)

Same structure as login response. The user is automatically assigned the `admin` role
for the newly created organization.

### Error Responses

| `error_code` | Message | Cause |
|-------------|---------|-------|
| `email_taken` | An account with this email already exists | Duplicate email |
| `validation_error` | Password must be at least 12 characters | Password too short |

### Cleanup on Failure

Registration creates 3 entities (org, role, user) with compensating cleanup:
if any step fails, previously created entities are deleted in reverse order.

---

## `POST /api/auth/refresh`

Exchange a refresh token for new access and refresh tokens.

**Authentication:** None required

**Status:** Returns `501 Not Implemented`. The full rotation flow is pending
the `RefreshStore` trait implementation against SurrealDB.

### Request

```json
{
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

---

## `GET /api/auth/profile`

Return the authenticated user's profile.

**Authentication:** Bearer JWT required

### Response (200 OK)

```json
{
  "success": true,
  "message": "Profile loaded",
  "data": {
    "id": "user:01JKXYZ",
    "email": "admin@example.com",
    "full_name": "Admin User",
    "organization": {
      "id": "organization:01ABC",
      "name": "My Company",
      "org_type": "company"
    },
    "role": {
      "id": "role:01DEF",
      "name": "admin",
      "permissions": { ... }
    },
    "preferences": {}
  }
}
```
