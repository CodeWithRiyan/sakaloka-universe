import type { components } from "./generated"
import type { DefaultResponse, StripNull } from "."
import type { IPageMeta } from "./pagination"

type Schemas = components["schemas"]

/** Stock/Inventory entity - generated from Loco API with frontend extensions. */
export type Stock = StripNull<Schemas["StockResponse"]> & {
  reorderLevel?: number
  lastStockMovement?: string
  deletedAt?: string
  deletedBy?: string
  createdBy?: string
  updatedBy?: string
  product: {
    name: string
    sku: string
    basePrice: string
    imageUrl?: string
  }
  location?: {
    name: string
    code: string
    type?: string
  }
  stockHistory: StockHistory[]
}

/** Stock movement history - generated from Loco API with frontend extensions. */
export type StockHistory = StripNull<Schemas["StockHistoryResponse"]> & {
  type: string
  createdByUser?: {
    id: string
    fullName: string
    email: string
  }
}

export type ProductStock = Stock & {
  product?: {
    name: string
    sku: string
    basePrice: string
  }
  location?: {
    name: string
    code: string
  }
}

export type StockList = Stock
export type StockDetail = Stock
export type ProductStockList = ProductStock

export interface StockDetailResponse extends DefaultResponse {
  data: StockDetail
}

export interface StockListResponse extends DefaultResponse {
  data: {
    data: StockList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export interface ProductStockListResponse extends DefaultResponse {
  data: {
    data: ProductStockList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export interface StockPayload {
  quantity: number
  reason: string
  notes?: string
}
