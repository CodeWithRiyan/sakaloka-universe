import { useAuth } from "@/hooks/use-auth"
import type { ReactNode } from "react"
import { Navigate, useLocation } from "react-router"

interface Props {
  children: ReactNode
}

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

export default ProtectedRoute
