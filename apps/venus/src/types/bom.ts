import type { components } from "./generated"
import type { DefaultResponse, StripNull } from "."
import type { IPageMeta } from "./pagination"

type Schemas = components["schemas"]

/** BOM entity - generated from OpenAPI with frontend extensions. */
export type Bom = StripNull<Schemas["BomResponse"]>

/** BOM with its items. */
export type BomWithItems = StripNull<Schemas["BomWithItemsResponse"]> & {
  items: BomItem[]
}

/** BOM item - a component in a BOM. */
export type BomItem = StripNull<Schemas["BomItemResponse"]>

/** Production run entity. */
export type ProductionRun = StripNull<Schemas["ProductionRunResponse"]>

/** Production run with consumptions. */
export type ProductionRunWithConsumption = StripNull<Schemas["ProductionRunWithConsumptionResponse"]> & {
  consumptions: Consumption[]
}

/** Consumption record. */
export type Consumption = StripNull<Schemas["ConsumptionResponse"]>

/** BOM detail response. */
export interface BomDetailResponse extends DefaultResponse {
  data: BomWithItems
}

/** BOM list response. */
export interface BomListResponse extends DefaultResponse {
  data: {
    data: Bom[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

/** Production run list response. */
export interface ProductionRunListResponse extends DefaultResponse {
  data: {
    data: ProductionRun[]
    pagination: IPageMeta
    filters: {
      sortBy: string
      sortOrder: string
    }
  }
}

/** Production run detail response. */
export interface ProductionRunDetailResponse extends DefaultResponse {
  data: ProductionRunWithConsumption
}

/** Payload for creating a BOM. */
export type BomCreatePayload = StripNull<Schemas["CreateBomRequest"]>

/** Payload for updating a BOM. */
export type BomUpdatePayload = StripNull<Schemas["UpdateBomRequest"]>

/** Payload for creating a BOM item. */
export type BomItemCreatePayload = StripNull<Schemas["CreateBomItemRequest"]>

/** Payload for updating a BOM item. */
export type BomItemUpdatePayload = StripNull<Schemas["UpdateBomItemRequest"]>

/** Payload for creating a production run. */
export type ProductionRunCreatePayload = StripNull<Schemas["CreateProductionRunRequest"]>

/** Payload for updating a production run. */
export type ProductionRunUpdatePayload = StripNull<Schemas["UpdateProductionRunRequest"]>

/** Payload for creating a consumption record. */
export type ConsumptionCreatePayload = StripNull<Schemas["CreateConsumptionRequest"]>

/** BOM cost breakdown. */
export type BomCostBreakdown = StripNull<Schemas["BomCostBreakdownResponse"]>