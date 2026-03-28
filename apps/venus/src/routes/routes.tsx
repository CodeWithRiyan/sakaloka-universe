import { lazy } from "react"
import { Navigate, type RouteObject } from "react-router"

// Public
const LoginPage = lazy(() => import("@/pages/login"))
const RegisterPage = lazy(() => import("@/pages/register"))
const RegistrationSuccessPage = lazy(
  () => import("@/pages/registration-success")
)
const ConfirmEmailPage = lazy(() => import("@/pages/confirm-email"))
const PasswordChangePage = lazy(() => import("@/pages/password-change"))

// Private Main
const DashboardLayout = lazy(
  () => import("@/components/layouts/private/dashboard-layout")
)
const Dashboard = lazy(() => import("@/pages/dashboard"))
const Product = lazy(() => import("@/pages/product"))
const Stock = lazy(() => import("@/pages/stock"))
const TransactionPage = lazy(() => import("@/pages/transaction"))

// Private Settings
const SettingsLayout = lazy(
  () => import("@/components/layouts/private/settings-layout")
)
const UserSetting = lazy(() => import("@/pages/user-setting"))
const BranchManagement = lazy(() => import("@/pages/branch-management"))
const EmployeeManagement = lazy(() => import("@/pages/employee-management"))
const RoleManagement = lazy(() => import("@/pages/role-management"))

// Not Found
const NotFound = lazy(() => import("@/pages/not-found"))

export const PublicRoute: RouteObject[] = [
  { path: "/", element: <Navigate to="/login" replace /> },
  { path: "/login", element: <LoginPage /> },
  { path: "/register", element: <RegisterPage /> },
  { path: "/registration-success", element: <RegistrationSuccessPage /> },
  { path: "/confirm-new-email", element: <ConfirmEmailPage /> },
  { path: "/confirm-email", element: <ConfirmEmailPage /> },
  { path: "/password-change", element: <PasswordChangePage /> },
]

export const PrivateRoute: RouteObject[] = [
  {
    id: "dashboard-layout",
    path: "/dashboard",
    element: <DashboardLayout />,
    children: [
      { id: "dashboard", path: "/dashboard", element: <Dashboard /> },
      {
        id: "product",
        path: "/dashboard/product",
        element: <Product />,
      },
      {
        id: "stock",
        path: "/dashboard/stock",
        element: <Stock />,
      },
      {
        id: "transaction",
        path: "/dashboard/transaction",
        element: <TransactionPage />,
      },
      { id: "dashboard-not-found", path: "*", element: <NotFound /> },
    ],
  },
  {
    id: "settings-layout",
    path: "/dashboard/settings",
    element: <SettingsLayout />,
    children: [
      {
        id: "user-setting",
        path: "/dashboard/settings",
        element: <UserSetting />,
      },
      {
        id: "branch-setting",
        path: "/dashboard/settings/branch",
        element: <BranchManagement />,
      },
      {
        id: "employee-management",
        path: "/dashboard/settings/employee",
        element: <EmployeeManagement />,
      },
      {
        id: "role-management",
        path: "/dashboard/settings/role",
        element: <RoleManagement />,
      },
      {
        id: "dashboard-settings-not-found",
        path: "/dashboard/settings/*",
        element: <NotFound />,
      },
    ],
  },
]

export const NotFoundRoute: RouteObject = {
  path: "*",
  element: <NotFound />,
}
