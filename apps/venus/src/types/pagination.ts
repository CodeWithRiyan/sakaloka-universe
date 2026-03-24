import type { components } from "./generated"

type Schemas = components["schemas"]

/** Pagination metadata - generated from Loco API. */
export type IPageMeta = Schemas["PageMeta"]

// Frontend-only types
export interface IPageMetaDetail {
  prev_path: string
  prev_title: string
  next_path: string
  next_title: string
}

export interface IPaginationParams {
  page: number
  limit: number
}
