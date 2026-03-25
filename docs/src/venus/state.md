# State Management

Venus uses **Redux Toolkit** for global state and **RTK Query** for server state (API caching).

## Store Configuration

```
store/
├── index.ts              # configureStore()
├── rootReducer.ts        # Combined reducers
├── rootMiddleware.ts     # RTK Query middleware chain
├── hooks.ts              # useAppDispatch, useAppSelector
├── base-query.ts         # Axios wrapper + 401 handling
├── cart/                 # POS cart state (local)
│   ├── reducer.ts
│   ├── actions.ts
│   └── constants.ts
├── stock/                # Stock local state
├── product/api.tsx       # RTK Query: products
├── order/api.tsx         # RTK Query: orders
├── user/api.tsx          # RTK Query: users
├── brand/api.tsx         # RTK Query: brands
├── categories/api.tsx    # RTK Query: categories
├── role/api.tsx          # RTK Query: roles
├── branch/api.tsx        # RTK Query: branches
└── utils/                # Generic UI state
```

## RTK Query API Slices

Each API slice uses `baseQueryWithAuth` which automatically:
- Attaches `Authorization: Bearer <token>` to all requests
- Redirects to `/login` on 401 responses
- Uses tag-based cache invalidation

| Slice | Base Path | Tag Types |
|-------|-----------|-----------|
| `productApi` | `/products` | `Product`, `ProductList` |
| `orderApi` | `/pos/orders` | `Order`, `OrderList` |
| `userApi` | `/users` | `User`, `UserList` |
| `brandApi` | `/products/brands` | `Brand`, `BrandList` |
| `categoriesApi` | `/products/categories` | `Category`, `CategoryList` |
| `roleApi` | `/roles` | `Role`, `RoleList` |
| `branchApi` | `/organizations` | `Branch`, `BranchList` |
| `stockApi` | `/inventory/pos-stock` | `Stock`, `StockList` |

## Cart State (Local)

The POS cart is managed entirely in Redux (no API calls until checkout):

| Action | Description |
|--------|-------------|
| `ADD_ITEM` | Add product to cart; increment quantity if exists |
| `REMOVE_ITEM` | Decrease quantity; remove if zero |
| `RESET_CHART` | Clear entire cart |
| `UPDATE_ITEMS` | Load items from a draft transaction |
| `SET_IS_UPDATE` | Flag edit mode for an existing order |
| `SET_OPEN_CHECKOUT` | Toggle checkout modal |
| `SET_OPEN_DRAFT` | Toggle draft selection modal |
| `SET_OPEN_RESULT` | Toggle result modal |
| `SET_TOTAL` | Update computed total |

## Base Query (API Client)

```typescript
// store/base-query.ts
const baseQuery = fetchBaseQuery({
  baseUrl: API_BASE_URL,
  prepareHeaders: (headers) => {
    const token = getTokenData()?.access_token
    if (token) headers.set("Authorization", `Bearer ${token}`)
    return headers
  },
})

// Global 401 handler
export const baseQueryWithAuth: BaseQueryFn = async (args, api, extra) => {
  const result = await baseQuery(args, api, extra)
  if (result.error?.status === 401) {
    clearToken()
    window.location.replace(`/login?callbackUrl=${encodeURIComponent(window.location.pathname)}`)
  }
  return result
}
```

## Custom Hooks

| Hook | Purpose |
|------|---------|
| `useAuth()` | Reactive auth state (user, isLoggedIn) with event-driven updates |
| `useTokenData()` | Access raw token data from localStorage |
| `useAppDispatch()` | Typed Redux dispatch |
| `useAppSelector()` | Typed Redux selector |
| `useBreakpoint()` | Responsive breakpoint detection |
| `useMobile()` | Mobile device detection |
