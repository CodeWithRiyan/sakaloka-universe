import type { components } from "./generated"
import type { DefaultResponse } from "."

type Schemas = components["schemas"]

/** Login response shape from Loco API. */
type ApiLoginResponse = Schemas["LoginResponse"]
type ApiOrgSummary = Schemas["OrgSummary"]
type ApiRoleSummary = Schemas["RoleSummary"]

export interface TokenData {
  access_token: ApiLoginResponse["access_token"]
  user: {
    id: string
    email: string
    fullName: string
    organization: ApiOrgSummary
    role: ApiRoleSummary
    preferences: Record<string, unknown>
  } | null
}

export interface AuthResponse extends DefaultResponse {
  data?: TokenData
}
