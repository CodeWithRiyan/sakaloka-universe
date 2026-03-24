import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams, ResponseError } from "@/types"
import {
  type ProductStockListResponse,
  type StockDetailResponse,
  type StockListResponse,
  type StockPayload,
} from "@/types/stock"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const stockApi = createApi({
  reducerPath: "stockApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Stock", "StockList", "ProductStockList"],
  endpoints: (builder) => ({
    getStockList: builder.query<StockListResponse, DefaultSearchParams>({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/inventory/pos-stock?${queryString.stringify({
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
                type: "Stock" as const,
                id,
              })),
              { type: "StockList", id: "LIST" },
            ]
          : [{ type: "StockList", id: "LIST" }],
    }),
    getLowStockProductList: builder.query<
      ProductStockListResponse,
      DefaultSearchParams
    >({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/inventory/pos-stock/low-stock?${queryString.stringify({
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
                type: "Stock" as const,
                id,
              })),
              { type: "ProductStockList", id: "LIST" },
            ]
          : [{ type: "ProductStockList", id: "LIST" }],
    }),
    getStockDetail: builder.query<StockDetailResponse, string>({
      query: (id) => ({
        url: `/inventory/pos-stock/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Stock", id }],
    }),
    createStock: builder.mutation<
      StockDetailResponse,
      {
        id: string
        payload: StockPayload
      }
    >({
      query: (data) => ({
        url: `/inventory/pos-stock/products/${data.id}/adjust`,
        method: "POST",
        body: data.payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [
        { type: "StockList", id: "LIST" },
        { type: "ProductStockList", id: "LIST" },
        { type: "Stock", id: "CURRENT" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Stock berhasil diubah!")
          } else {
            toast.error(result.data.error || "Error", {
              description: result.data.errors && (
                <ToastDescriptionList
                  list={result.data.errors}
                  variant="error"
                />
              ),
            })
          }
        } catch (error) {
          const err = error as ResponseError
          toast.error(err.error.data.error || "Error", {
            description: err.error.data.errors && (
              <ToastDescriptionList
                list={err.error.data.errors}
                variant="error"
              />
            ),
          })
        }
      },
    }),
  }),
})

export const {
  useGetStockListQuery,
  useGetLowStockProductListQuery,
  useGetStockDetailQuery,
  useCreateStockMutation,
} = stockApi
