import type { components } from "./generated"
import type { DefaultResponse, ImageMetadata, StripNull } from "."
import type { IPageMeta } from "./pagination"
import type { Product } from "./product"

type Schemas = components["schemas"]

/** Brand entity - generated from Loco API with frontend extensions. */
export type Brand = StripNull<Schemas["BrandResponse"]> & {
  logoMetadata?: ImageMetadata
  deletedAt?: string
  deletedBy?: string
  createdBy?: string
  updatedBy?: string
}

export type BrandList = Brand & {
  _count?: { products: number }
}

export type BrandDetail = Brand & {
  products?: Product[]
  _count?: { products: number }
}

export interface BrandDetailResponse extends DefaultResponse {
  data: BrandDetail
}

export interface BrandListResponse extends DefaultResponse {
  data: {
    data: BrandList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export type BrandPayload = StripNull<Schemas["CreateBrandRequest"]> & {
  parentId?: string
  sortOrder?: number
  isActive?: boolean
  image?: Blob
}
