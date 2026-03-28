import type { components } from "./generated"
import type { DefaultResponse, StripNull } from "."
import type { IPageMeta } from "./pagination"

type Schemas = components["schemas"]

export type PermissionType = "create" | "read" | "update" | "delete" | "select"

/** Role entity - generated from Loco API with frontend extensions. */
export type Role = Omit<StripNull<Schemas["RoleResponse"]>, "permissions"> & {
  permissions: Record<string, string[]>
  creator?: { id: string; fullName: string; email: string }
}

export type RoleList = Role & {
  _count?: { users: number }
}

export type RoleDetail = Role & {
  organization?: { id: string; name: string }
  users?: { id: string; fullName: string; email: string }[]
  _count?: { users: number }
}

export interface RoleCurrentResponse {
  success: boolean
  data: {
    id: string
    name: string
    type: string
    code?: string
    description?: string
    address?: string
    logo?: string
    role: string
    membershipType: string
  }
  message: string
}

export interface RoleListResponse extends DefaultResponse {
  data: {
    data: RoleList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export interface RoleDetailResponse extends DefaultResponse {
  data: RoleDetail
}

export interface PermissionResponse extends DefaultResponse {
  data: {
    key: string
    label: string
    description: string
    permissions: { key: string; label: string }[]
  }[]
}

export type RolePayload = StripNull<Schemas["CreateRoleRequest"]>
