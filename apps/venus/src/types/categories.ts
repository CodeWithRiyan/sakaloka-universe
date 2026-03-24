import type { components } from "./generated"
import type { DefaultResponse, ImageMetadata, StripNull } from "."
import type { IPageMeta } from "./pagination"
import type { Product } from "./product"

type Schemas = components["schemas"]

/** Category entity - generated from Loco API with frontend extensions. */
export type Categories = StripNull<Schemas["CategoryResponse"]> & {
  imageMetadata?: ImageMetadata
  deletedAt?: string
  deletedBy?: string
  createdBy?: string
  updatedBy?: string
  parent?: Categories
  children?: Categories[]
}

export type CategoriesList = Categories & {
  _count?: { products: number }
}

export type CategoriesDetail = Categories & {
  products?: Product[]
  _count?: { products: number }
}

export interface CategoriesDetailResponse extends DefaultResponse {
  data: CategoriesDetail
}

export interface CategoriesListResponse extends DefaultResponse {
  data: {
    data: CategoriesList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export type CategoriesPayload = StripNull<Schemas["CreateCategoryRequest"]> & {
  sortOrder?: number
  isActive?: boolean
  image?: Blob
}
