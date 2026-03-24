import { API_BASE_URL } from "@/constants"
import type { ResponseError } from "@/types"
import type { AuthResponse, TokenData } from "@/types/auth"
import { toast } from "sonner"

// Constants
export const TOKEN_KEY = "auth_token_data"

// State to prevent multiple refresh calls
let isRefreshing = false
let refreshPromise: Promise<string | null> | null = null

/**
 * Utility function untuk trigger custom event ketika auth data berubah
 */
function triggerAuthChange(): void {
  if (typeof window !== "undefined") {
    window.dispatchEvent(new CustomEvent("auth-changed"))
  }
}

/**
 * Save token to localStorage (UPDATED with event trigger)
 */
export function saveToken(authResponse: AuthResponse): void {
  const tokenData: TokenData = {
    access_token: authResponse.data?.access_token || "",
    user: authResponse.data?.user || null,
  }

  localStorage.setItem(TOKEN_KEY, JSON.stringify(tokenData))
  triggerAuthChange() // Trigger event setelah save token
}

/**
 * Update user data (UPDATED with event trigger)
 */
function updateUser(authResponse: AuthResponse): void {
  const tokenData = getTokenData()
  if (!tokenData) return

  const newData: TokenData = {
    ...tokenData,
    user: authResponse.data?.user || null,
  }

  localStorage.setItem(TOKEN_KEY, JSON.stringify(newData))
  triggerAuthChange() // Trigger event setelah update user
}

/**
 * Ambil token data dari localStorage
 */
export function getTokenData(): TokenData | null {
  try {
    const tokenDataStr = localStorage.getItem(TOKEN_KEY)
    if (!tokenDataStr) return null
    return JSON.parse(tokenDataStr)
  } catch (error) {
    console.error("Error getting token data:", error)
    clearToken()
    return null
  }
}

/**
 * Get refresh token
 */
export function getRefreshToken(): string | null {
  const tokenData = getTokenData()
  return tokenData?.access_token || null
}

/**
 * Refresh access token using refresh token (UPDATED with event trigger)
 */
async function performTokenRefresh(
  refreshToken: string
): Promise<string | null> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/refresh`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        refresh_token: refreshToken,
      }),
    })

    if (!response.ok) {
      console.error("Token refresh failed:", response.statusText)
      clearToken()
      return null
    }

    const refreshResponse: AuthResponse = await response.json()

    if (refreshResponse.success && refreshResponse.data?.access_token) {
      saveToken(refreshResponse) // saveToken sudah include triggerAuthChange
      console.log("Token refreshed successfully")
      return refreshResponse.data.access_token
    } else {
      console.error("Token refresh response invalid:", refreshResponse.message)
      clearToken()
      return null
    }
  } catch (error) {
    console.error("Token refresh error:", error)
    clearToken()
    return null
  }
}

/**
 * Refresh token with prevention for multiple calls
 */
export async function refreshTokenIfPossible(): Promise<string | null> {
  // Prevent multiple simultaneous refresh calls
  if (isRefreshing && refreshPromise) {
    return refreshPromise
  }

  const refreshToken = getRefreshToken()
  if (!refreshToken) {
    console.log("No refresh token available")
    clearToken()
    return null
  }

  isRefreshing = true
  refreshPromise = performTokenRefresh(refreshToken)

  try {
    const newAccessToken = await refreshPromise
    return newAccessToken
  } finally {
    isRefreshing = false
    refreshPromise = null
  }
}

/**
 * Get access token with auto refresh
 */
export async function getToken(): Promise<string | null> {
  const tokenData = getTokenData()
  if (!tokenData) return null

  return tokenData.access_token
}

/**
 * Get user data
 */
export function getAuthData(): TokenData["user"] | null {
  const tokenData = getTokenData()
  if (!tokenData) return null

  return tokenData.user
}

/**
 * Check if user is authenticated
 */
export function isAuthenticated(): boolean {
  const tokenData = getTokenData()

  return Boolean(tokenData)
}

export async function login({
  email,
  password,
}: {
  email: string
  password: string
}): Promise<AuthResponse | null> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    })

    const data: AuthResponse = await response.json()

    if (!response.ok) throw new Error(data.error)

    if (data.success && data.data?.access_token) {
      saveToken(data)
      console.log("Login successful, tokens saved")
    }

    return data
  } catch (error) {
    return {
      success: false,
      message: "Login gagal",
      error: (error as Error).message,
    }
  }
}

export async function register({
  email,
  password,
  fullName,
  organizationName,
}: {
  email: string
  password: string
  fullName: string
  organizationName: string
}): Promise<AuthResponse | null> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password, fullName, organizationName }),
    })

    const data: AuthResponse = await response.json()

    if (!response.ok) throw new Error(data.error)

    if (data.success && data.data?.access_token) {
      saveToken(data)
      console.log("Registration successful, tokens saved")
    }

    return data
  } catch (error) {
    return {
      success: false,
      message: "Registrasi gagal",
      error: (error as Error).message,
    }
  }
}

/**
 * Update auth user data (UPDATED with event trigger)
 */
export async function updateAuthUserData(): Promise<void> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/profile`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${await getToken()}`,
      },
    })

    const data: AuthResponse = await response.json()

    if (!response.ok) throw new Error(data.error)

    if (data.success) updateUser(data) // updateUser sudah include triggerAuthChange
  } catch (error) {
    const err = error as ResponseError
    toast.error(err.error?.data?.error)
    console.error("Failed to update user data:", error)
  }
}

/**
 * Manual token refresh
 */
export async function manualRefreshToken(): Promise<boolean> {
  try {
    const newToken = await refreshTokenIfPossible()
    return newToken !== null
  } catch (error) {
    console.error("Manual refresh failed:", error)
    return false
  }
}

/**
 * Remove token from localStorage (UPDATED with event trigger)
 */
export function clearToken(): void {
  localStorage.removeItem(TOKEN_KEY)
  triggerAuthChange() // Trigger event setelah clear token
}

/**
 * Logout user
 */
export function logout(): void {
  clearToken()
  console.log("User logged out, tokens cleared")
}

/**
 * Cek status login
 */
export function isLoggedIn(): boolean {
  return isAuthenticated()
}
