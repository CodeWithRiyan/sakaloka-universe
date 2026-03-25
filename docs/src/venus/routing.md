# Routing & Pages

Venus uses **React Router 7.8** with lazy-loaded route components wrapped in a
`ProtectedRoute` guard.

## Route Map

### Public Routes (No Authentication)

| Path | Page | Layout |
|------|------|--------|
| `/` | Landing page | MainLayout (public navbar) |
| `/login` | Login form | MainLayout |
| `/register` | Registration form | MainLayout |
| `/registration-success` | Success confirmation | MainLayout |
| `/confirm-email` | Email confirmation | MainLayout |
| `/confirm-new-email` | New email confirmation | MainLayout |
| `/password-change` | Password reset | MainLayout |

### Private Routes (JWT Required)

| Path | Page | Layout |
|------|------|--------|
| `/dashboard` | Dashboard overview | DashboardLayout (sidebar + navbar) |
| `/dashboard/product` | Product catalog management | DashboardLayout |
| `/dashboard/stock` | Stock/inventory management | DashboardLayout |
| `/dashboard/transaction` | POS checkout & order history | DashboardLayout |

### Settings Routes

| Path | Page | Layout |
|------|------|--------|
| `/dashboard/settings` | User settings | SettingsLayout |
| `/dashboard/settings/branch` | Branch management | SettingsLayout |
| `/dashboard/settings/employee` | Employee management | SettingsLayout |
| `/dashboard/settings/role` | Role & permissions management | SettingsLayout |

## Route Protection

The `ProtectedRoute` component enforces authentication:

- **Unauthenticated** user visiting `/dashboard/*` is redirected to `/`
- **Authenticated** user visiting public pages (`/login`, `/register`) is redirected to `/dashboard`

```typescript
// components/auth/protected-route.tsx
export function ProtectedRoute({ children }: Props) {
  const { isAuthenticated } = useAuth()
  const { pathname } = useLocation()

  if (!isAuthenticated && pathname.includes("/dashboard")) {
    return <Navigate to="/" replace />
  }
  if (isAuthenticated && !pathname.includes("/dashboard")) {
    return <Navigate to="/dashboard" replace />
  }
  return children
}
```

## Lazy Loading

All page components are loaded with `React.lazy()` and wrapped in `Suspense` with a
loading spinner. This enables code splitting — each page is a separate chunk.

## Layouts

| Layout | Description |
|--------|-------------|
| `MainLayout` | Public pages: navbar with login/register links |
| `DashboardLayout` | Private pages: collapsible sidebar + top navbar |
| `SettingsLayout` | Settings sub-pages: nested within DashboardLayout |
