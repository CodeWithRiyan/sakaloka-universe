// hooks/useAuth.ts
import {
  getAuthData,
  getTokenData,
  isAuthenticated,
  TOKEN_KEY,
} from "@/lib/auth"
import type { TokenData } from "@/types/auth"
import { useEffect, useState } from "react"

/**
 * Custom hook untuk mendapatkan auth data yang reaktif terhadap perubahan localStorage
 */
export function useAuth() {
  const [user, setUser] = useState<TokenData["user"] | null>(() =>
    getAuthData()
  )
  const [isLoggedIn, setIsLoggedIn] = useState<boolean>(() => isAuthenticated())

  useEffect(() => {
    // Function untuk update auth state
    const updateAuthState = () => {
      const authData = getAuthData()
      const authenticated = isAuthenticated()

      setUser(authData)
      setIsLoggedIn(authenticated)
    }

    // Listen untuk storage events (perubahan localStorage dari tab/window lain)
    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === TOKEN_KEY || e.key === null) {
        updateAuthState()
      }
    }

    // Listen untuk custom events (perubahan dari window yang sama)
    const handleAuthChange = () => {
      updateAuthState()
    }

    // Add event listeners
    window.addEventListener("storage", handleStorageChange)
    window.addEventListener("auth-changed", handleAuthChange)

    // Cleanup
    return () => {
      window.removeEventListener("storage", handleStorageChange)
      window.removeEventListener("auth-changed", handleAuthChange)
    }
  }, [])

  return {
    user,
    isLoggedIn,
    isAuthenticated: isLoggedIn,
  }
}

/**
 * Custom hook untuk mendapatkan token data lengkap
 */
export function useTokenData() {
  const [tokenData, setTokenData] = useState<TokenData | null>(() =>
    getTokenData()
  )

  useEffect(() => {
    const updateTokenData = () => {
      const data = getTokenData()
      setTokenData(data)
    }

    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === TOKEN_KEY || e.key === null) {
        updateTokenData()
      }
    }

    const handleAuthChange = () => {
      updateTokenData()
    }

    window.addEventListener("storage", handleStorageChange)
    window.addEventListener("auth-changed", handleAuthChange)

    return () => {
      window.removeEventListener("storage", handleStorageChange)
      window.removeEventListener("auth-changed", handleAuthChange)
    }
  }, [])

  return tokenData
}
