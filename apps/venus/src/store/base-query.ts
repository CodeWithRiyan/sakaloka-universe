import { API_BASE_URL } from "@/constants"
import { clearToken, getTokenData } from "@/lib/auth"
import type {
  BaseQueryFn,
  FetchArgs,
  FetchBaseQueryError,
} from "@reduxjs/toolkit/query"
import { fetchBaseQuery } from "@reduxjs/toolkit/query/react"

const baseQuery = fetchBaseQuery({
  baseUrl: API_BASE_URL,
  prepareHeaders: (headers) => {
    const token = getTokenData()?.access_token
    if (token) {
      headers.set("Authorization", `Bearer ${token}`)
    }
    return headers
  },
})

export const baseQueryWithAuth: BaseQueryFn<
  string | FetchArgs,
  unknown,
  FetchBaseQueryError
> = async (args, api, extraOptions) => {
  const result = await baseQuery(args, api, extraOptions)
  const location = window.location.pathname

  // Handle 401 globally
  if (result.error && result.error.status === 401) {
    console.log("Unauthorized, logging out...")
    clearToken()
    window.location.replace(
      `/login?callbackUrl=${encodeURIComponent(location)}`
    )
  }

  return result
}
