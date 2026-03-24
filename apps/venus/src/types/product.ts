import type { components } from "./generated"
import type { DefaultResponse, ImageMetadata, StripNull } from "."
import type { Brand } from "./brand"
import type { Categories } from "./categories"
import type { IPageMeta } from "./pagination"

type Schemas = components["schemas"]

/** Product entity - generated from Loco API with frontend extensions. */
export type Product = StripNull<Schemas["ProductResponse"]> & {
  imageMetadata?: ImageMetadata
  images?: string
  metadata?: string
  createdBy?: string
  updatedBy?: string
  deletedBy?: string
  variants?: Product[]
  category?: Pick<Categories, "id" | "name" | "slug">
  brand?: Pick<Brand, "id" | "name" | "slug">
}

export type ProductList = StripNull<Schemas["ProductListResponse"]> & {
  imageMetadata?: ImageMetadata
  images?: string
  metadata?: string
  createdBy?: string
  updatedBy?: string
  deletedBy?: string
  weight?: string
  trackInventory?: boolean
  minStockLevel?: number
  variants?: Product[]
  category?: Pick<Categories, "id" | "name" | "slug">
  brand?: Pick<Brand, "id" | "name" | "slug">
}

export type ProductDetail = Product & {
  _count?: { variants: number }
}

export interface ProductDetailResponse extends DefaultResponse {
  data: ProductDetail
}

export interface ProductListResponse extends DefaultResponse {
  data: {
    data: ProductList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export type ProductPayload = StripNull<Schemas["CreateProductRequest"]> & {
  image?: Blob
  initialStock?: number
}
