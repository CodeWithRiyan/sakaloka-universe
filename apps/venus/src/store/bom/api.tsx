import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams } from "@/types"
import {
  type BomDetailResponse,
  type BomListResponse,
  type BomCreatePayload,
  type BomUpdatePayload,
  type BomItemCreatePayload,
  type ProductionRunCreatePayload,
  type ConsumptionCreatePayload,
  type ProductionRunListResponse,
  type ProductionRunDetailResponse,
} from "@/types/bom"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const bomApi = createApi({
  reducerPath: "bomApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Bom", "BomList", "BomItem", "ProductionRun", "ProductionRunList"],
  endpoints: (builder) => ({
    // BOM List
    getBomList: builder.query<BomListResponse, DefaultSearchParams>({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/boms?${queryString.stringify({
          page,
          limit,
          search,
          sortBy,
          sortOrder,
        })}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (result) =>
        result
          ? [
              ...result.data.data.map(({ id }) => ({
                type: "Bom" as const,
                id,
              })),
              { type: "BomList", id: "LIST" },
            ]
          : [{ type: "BomList", id: "LIST" }],
    }),

    // BOM Detail
    getBomDetail: builder.query<BomDetailResponse, string>({
      query: (id) => ({
        url: `/boms/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Bom", id }],
    }),

    // Get active BOM for product
    getBomByProduct: builder.query<BomDetailResponse, string>({
      query: (productId) => ({
        url: `/boms/product/${productId}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, productId) => [
        { type: "Bom", id: productId },
      ],
    }),

    // Create BOM
    createBom: builder.mutation<BomDetailResponse, BomCreatePayload>({
      query: (data) => ({
        url: `/boms`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "BomList", id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("BOM created successfully")
        } catch {
          toast.error("Failed to create BOM")
        }
      },
    }),

    // Update BOM
    updateBom: builder.mutation<
      BomDetailResponse,
      { id: string; payload: BomUpdatePayload }
    >({
      query: ({ id, payload }) => ({
        url: `/boms/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Bom", id },
        { type: "BomList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("BOM updated successfully")
        } catch {
          toast.error("Failed to update BOM")
        }
      },
    }),

    // Delete BOM
    deleteBom: builder.mutation<BomDetailResponse, string>({
      query: (id) => ({
        url: `/boms/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "BomList", id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("BOM archived successfully")
        } catch {
          toast.error("Failed to archive BOM")
        }
      },
    }),

    // Activate BOM
    activateBom: builder.mutation<BomDetailResponse, string>({
      query: (id) => ({
        url: `/boms/${id}/activate`,
        method: "POST",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Bom", id },
        { type: "BomList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("BOM activated successfully")
        } catch {
          toast.error("Failed to activate BOM")
        }
      },
    }),

    // BOM Items - Add
    addBomItem: builder.mutation<
      BomDetailResponse,
      { bomId: string; payload: BomItemCreatePayload }
    >({
      query: ({ bomId, payload }) => ({
        url: `/boms/${bomId}/items`,
        method: "POST",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { bomId }) => [
        { type: "Bom", id: bomId },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("Component added successfully")
        } catch {
          toast.error("Failed to add component")
        }
      },
    }),

    // BOM Items - Update
    updateBomItem: builder.mutation<
      BomDetailResponse,
      { bomId: string; itemId: string; payload: BomItemCreatePayload }
    >({
      query: ({ bomId, itemId, payload }) => ({
        url: `/boms/${bomId}/items/${itemId}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { bomId }) => [
        { type: "Bom", id: bomId },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("Component updated successfully")
        } catch {
          toast.error("Failed to update component")
        }
      },
    }),

    // BOM Items - Delete
    removeBomItem: builder.mutation<
      BomDetailResponse,
      { bomId: string; itemId: string }
    >({
      query: ({ bomId, itemId }) => ({
        url: `/boms/${bomId}/items/${itemId}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { bomId }) => [
        { type: "Bom", id: bomId },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("Component removed successfully")
        } catch {
          toast.error("Failed to remove component")
        }
      },
    }),

    // Production Runs - List
    getProductionRuns: builder.query<
      ProductionRunListResponse,
      { bomId: string } & DefaultSearchParams
    >({
      query: ({ bomId, page = 1, limit = 10 }) => ({
        url: `/boms/${bomId}/production-runs?${queryString.stringify({
          page,
          limit,
        })}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (result) =>
        result
          ? [
              ...result.data.data.map(({ id }) => ({
                type: "ProductionRun" as const,
                id,
              })),
              { type: "ProductionRunList", id: "LIST" },
            ]
          : [{ type: "ProductionRunList", id: "LIST" }],
    }),

    // Production Runs - Detail
    getProductionRunDetail: builder.query<
      ProductionRunDetailResponse,
      { bomId: string; runId: string }
    >({
      query: ({ runId }) => ({
        url: `/boms/${runId.split(":")[0]}/production-runs/${runId}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, { runId }) => [
        { type: "ProductionRun", id: runId },
      ],
    }),

    // Production Runs - Create
    createProductionRun: builder.mutation<
      ProductionRunDetailResponse,
      { bomId: string; payload: ProductionRunCreatePayload }
    >({
      query: ({ bomId, payload }) => ({
        url: `/boms/${bomId}/production-runs`,
        method: "POST",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "ProductionRunList", id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("Production run created successfully")
        } catch {
          toast.error("Failed to create production run")
        }
      },
    }),

    // Production Runs - Update
    updateProductionRun: builder.mutation<
      ProductionRunDetailResponse,
      { bomId: string; runId: string; payload: ProductionRunCreatePayload }
    >({
      query: ({ bomId, runId, payload }) => ({
        url: `/boms/${bomId}/production-runs/${runId}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { runId }) => [
        { type: "ProductionRun", id: runId },
        { type: "ProductionRunList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("Production run updated successfully")
        } catch {
          toast.error("Failed to update production run")
        }
      },
    }),

    // Consumptions - Add
    addConsumption: builder.mutation<
      ProductionRunDetailResponse,
      { bomId: string; runId: string; payload: ConsumptionCreatePayload }
    >({
      query: ({ bomId, runId, payload }) => ({
        url: `/boms/${bomId}/production-runs/${runId}/consumptions`,
        method: "POST",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { runId }) => [
        { type: "ProductionRun", id: runId },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          await queryFulfilled
          toast.success("Consumption recorded successfully")
        } catch {
          toast.error("Failed to record consumption")
        }
      },
    }),
  }),
})

export const {
  useGetBomListQuery,
  useGetBomDetailQuery,
  useGetBomByProductQuery,
  useCreateBomMutation,
  useUpdateBomMutation,
  useDeleteBomMutation,
  useActivateBomMutation,
  useAddBomItemMutation,
  useUpdateBomItemMutation,
  useRemoveBomItemMutation,
  useGetProductionRunsQuery,
  useGetProductionRunDetailQuery,
  useCreateProductionRunMutation,
  useUpdateProductionRunMutation,
  useAddConsumptionMutation,
} = bomApi