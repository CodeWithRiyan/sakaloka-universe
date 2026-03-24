import type { components } from "./generated"
import type { DefaultResponse, StripNull } from "."
import type { Branch } from "./branch"
import type { IPageMeta } from "./pagination"
import type { Product } from "./product"
import type { User } from "./user"

type Schemas = components["schemas"]

/** Menu item for POS - generated from Loco API with frontend extensions. */
export type Menu = StripNull<Schemas["MenuResponse"]> & {
  description?: string
  brandId?: string
  brandName?: string
  availableStock: number
  trackInventory: boolean
  createdAt?: string
  updatedAt?: string
}

/** Order entity - generated from Loco API with frontend extensions.
 *  Monetary fields are overridden to `string` to match the old API format. */
export type Order = Omit<
  StripNull<Schemas["OrderResponse"]>,
  | "subtotal"
  | "taxAmount"
  | "discountAmount"
  | "totalAmount"
  | "paidAmount"
  | "items"
> & {
  subtotal: string
  taxAmount: string
  discountAmount: string
  totalAmount: string
  paidAmount: string
  internalNotes?: string
  completedAt?: string
  orderDate?: string
  deletedAt?: string
  deletedBy?: string
  createdBy?: string
  updatedBy?: string
  items: {
    id: string
    salesOrderId?: string
    productId: string
    inventoryItemId?: string
    itemName: string
    quantity: number
    unitPrice: string
    totalPrice: string
    product?: Product
  }[]
  customer?: User
  organization?: Branch
}

export type OrderList = Order & {
  _count?: { variants: number }
}

export type OrderDetail = Order & {
  _count?: { variants: number }
}

export interface MenuResponse extends DefaultResponse {
  data: {
    data: Menu[]
    pagination: IPageMeta
  }
}

export interface OrderDetailResponse extends DefaultResponse {
  data: OrderDetail
}

export interface OrderListResponse extends DefaultResponse {
  data: {
    data: OrderList[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

export interface OrderPayload {
  items: {
    productId: string
    quantity: number
    unitPrice?: number
    itemName?: string
  }[]
  paymentMethod: string
  totalPayment?: number
  totalTax?: number
  type: string
  tableNumber?: number
  customerName?: string
  notes?: string
}

// Frontend-only types
export type OrderItem = {
  productId: string
  quantity: number
  name: string
  price: number
  totalPrice?: number
  prevQty?: number
}

export type OrderTabs =
  | "new-transaction"
  | "edit-transaction"
  | "draft"
  | "finished"
