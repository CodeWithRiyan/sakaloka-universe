import type { components } from "./generated"
import type { DefaultResponse, StripNull } from "."
import type { IPageMeta } from "./pagination"

type Schemas = components["schemas"]

/** User entity - generated from Loco API with frontend extensions. */
export type User = StripNull<Schemas["UserResponse"]> & {
  username?: string
  organization: {
    id: string
    name: string
    type: string
    code?: string
    description?: string
    parentId?: string
    email?: string
    phone?: string
    website?: string
    address?: string
    city?: string
    state?: string
    country?: string
    postalCode?: number
    taxNumber?: number
    registrationNumber?: number
    settings?: string
    logo?: string
    isActive: boolean
    createdAt: string
    updatedAt: string
    ownerId: string
  }
  role: {
    id: string
    name: string
    organizationId: string
    isSystemRole: boolean
    permissions: { [key: string]: string[] }
    createdBy: string
    isActive: boolean
    createdAt: string
    updatedAt: string
  }
}

export type UserList = User & {
  _count?: { users: number; employees: number; customers: number }
}

export type UserDetail = User | null

export interface UserCurrentResponse extends DefaultResponse {
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
}

export interface UserListResponse extends DefaultResponse {
  data: {
    data: UserList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export interface UserDetailResponse extends DefaultResponse {
  data: UserDetail
}

export interface UserPayload {
  email: string
  fullName: string
  organizationId?: string
  roleId: string
  password?: string
  isActive?: boolean
}
