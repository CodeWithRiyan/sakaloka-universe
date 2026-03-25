# Authentication (Frontend)

Venus handles authentication via the Earth API, storing tokens in localStorage and
using event-driven state synchronization.

## Auth Flow

```
1. User enters email + password on /login
2. POST /api/auth/login -> Earth API
3. Receive { access_token, refresh_token, user }
4. Save to localStorage key: "auth_token_data"
5. Dispatch "auth-changed" custom event
6. ProtectedRoute detects auth -> redirect to /dashboard
7. All subsequent API calls include Bearer token
```

## Token Storage

Tokens are stored in **localStorage** under the key `auth_token_data`:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "user:01JKXYZ",
    "email": "admin@example.com",
    "fullName": "Admin User",
    "organization": { "id": "...", "name": "My Company", "orgType": "company" },
    "role": { "id": "...", "name": "admin", "permissions": { ... } },
    "preferences": {}
  }
}
```

## Auth Functions (`lib/auth.ts`)

| Function | Description |
|----------|-------------|
| `login({ email, password })` | POST to `/auth/login`, save tokens |
| `register({ email, password, fullName, organizationName })` | POST to `/auth/register`, save tokens |
| `saveToken(authResponse)` | Save response to localStorage, emit event |
| `getTokenData()` | Retrieve full token data |
| `getAuthData()` | Get user object only |
| `isAuthenticated()` | Check if token exists |
| `clearToken()` | Remove from localStorage, emit event |
| `logout()` | Clear token and user data |
| `updateAuthUserData()` | Refresh profile from `/auth/profile` |
| `refreshTokenIfPossible()` | Attempt token refresh (deduped) |

## Event-Driven Sync

Auth state changes trigger a custom DOM event:

```typescript
window.dispatchEvent(new CustomEvent("auth-changed"))
```

The `useAuth()` hook listens for both:
- `auth-changed` — same-tab auth updates
- `storage` — cross-tab localStorage changes

This ensures all components react immediately to login/logout without prop drilling.

## 401 Global Handler

RTK Query's `baseQueryWithAuth` intercepts all 401 responses:

1. Clears the stored token
2. Redirects to `/login?callbackUrl={currentPath}`

After re-login, the callback URL redirects the user back to their original page.

## Route Guards

The `ProtectedRoute` wrapper checks `useAuth().isAuthenticated`:

- `/dashboard/*` without auth redirects to `/`
- `/login`, `/register` with auth redirects to `/dashboard`
