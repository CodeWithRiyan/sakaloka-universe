import type { components } from "./generated"
import type { DefaultResponse, StripNull } from "."
import type { IPageMeta } from "./pagination"

type Schemas = components["schemas"]

/** Branch/Organization entity - generated from Loco API with frontend extensions. */
export type Branch = StripNull<Schemas["OrganizationResponse"]> & {
  parent?: string
  children?: Branch[]
}

export type BranchList = Branch & {
  _count?: { users: number; employees: number; customers: number }
}

export type BranchDetail = Branch & {
  users?: { id: string; email: string; fullName: string }[]
  _count?: { employees: number; customers: number; salesOrders: number; inventoryItems: number }
}

export interface BranchDetailResponse extends DefaultResponse {
  data: BranchDetail
}

export interface BranchCurrentResponse extends DefaultResponse {
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

export interface BranchListResponse extends DefaultResponse {
  data: {
    data: BranchList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
      sortType?: string
    }
  }
}

export type BranchPayload = StripNull<Schemas["CreateOrgRequest"]>
