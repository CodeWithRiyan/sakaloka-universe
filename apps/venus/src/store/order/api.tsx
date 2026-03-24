import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type {
  DefaultResponse,
  DefaultSearchParams,
  ResponseError,
} from "@/types"
import {
  type MenuResponse,
  type OrderDetailResponse,
  type OrderList,
  type OrderListResponse,
  type OrderPayload,
} from "@/types/order"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const orderApi = createApi({
  reducerPath: "orderApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Order", "OrderList", "OrderHistory", "OrderActive", "Menu"],
  endpoints: (builder) => ({
    getMenuList: builder.query<
      MenuResponse,
      DefaultSearchParams & {
        categoryId?: string
        brandId?: string
        isFeatured?: boolean
      }
    >({
      query: ({ page = 1, limit = 10, ...rest }) => ({
        url: `pos/menu?${queryString.stringify({
          page,
          limit,
          ...rest,
        })}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: [{ type: "Menu" }],
    }),
    getOrderHistoryList: builder.query<
      DefaultResponse & { data: OrderList[] },
      DefaultSearchParams & {
        fromDate?: string
        toDate?: string
      }
    >({
      query: ({ page = 1, limit = 10, ...rest }) => ({
        url: `pos/orders/history?${queryString.stringify({
          page,
          limit,
          ...rest,
        })}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (result) =>
        result
          ? [
              ...result.data.map(({ id }) => ({
                type: "Order" as const,
                id,
              })),
              { type: "OrderHistory", id: "LIST" },
            ]
          : [{ type: "OrderHistory", id: "LIST" }],
    }),
    getOrderActiveList: builder.query<
      DefaultResponse & { data: OrderList[] },
      void
    >({
      query: () => ({
        url: `pos/orders/active`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (result) =>
        result
          ? [
              ...result.data.map(({ id }) => ({
                type: "Order" as const,
                id,
              })),
              { type: "OrderActive", id: "LIST" },
            ]
          : [{ type: "OrderActive", id: "LIST" }],
    }),
    getOrderList: builder.query<OrderListResponse, DefaultSearchParams>({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `pos/orders?${queryString.stringify({
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
              ...(result.data.data &&
                result.data.data.map(({ id }) => ({
                  type: "Order" as const,
                  id,
                }))),
              { type: "OrderList", id: "LIST" },
            ]
          : [{ type: "OrderList", id: "LIST" }],
    }),
    getOrderDetail: builder.query<OrderDetailResponse, string>({
      query: (id) => ({
        url: `pos/orders/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Order", id }],
    }),
    createOrder: builder.mutation<OrderDetailResponse, OrderPayload>({
      query: (data) => ({
        url: `pos/orders`,
        method: "POST",
        body: {
          ...data,
          paymentMethod: undefined,
          totalPayment: undefined,
          totalTax: undefined,
        },
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "OrderList", id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled, dispatch }) => {
        try {
          const result = await queryFulfilled
          if (result.data.success) {
            toast.success("Produk berhasil dibuat!")
            dispatch({ type: "OrderHistoryList", id: "LIST" })
            dispatch({ type: "OrderActiveList", id: "LIST" })
            dispatch({ type: "Menu" })
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
    updateOrder: builder.mutation<
      OrderDetailResponse,
      {
        id: string
        payload: OrderPayload
      }
    >({
      query: ({ id, payload }) => ({
        url: `pos/orders/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Order", id },
        { type: "OrderList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled, dispatch }) => {
        try {
          const result = await queryFulfilled
          if (result.data.success) {
            toast.success("Produk berhasil diubah!")
            dispatch({ type: "OrderHistoryList", id: "LIST" })
            dispatch({ type: "OrderActiveList", id: "LIST" })
            dispatch({ type: "Menu" })
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
    deleteOrder: builder.mutation<OrderDetailResponse, string>({
      query: (id) => ({
        url: `pos/orders/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Order", id },
        { type: "OrderList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled, dispatch }) => {
        try {
          const result = await queryFulfilled
          if (result.data.success) {
            toast.success("Produk berhasil dihapus!")
            dispatch({ type: "OrderHistoryList", id: "LIST" })
            dispatch({ type: "OrderActiveList", id: "LIST" })
            dispatch({ type: "Menu" })
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
  useGetOrderActiveListQuery,
  useGetOrderHistoryListQuery,
  useGetMenuListQuery,
  useGetOrderListQuery,
  useGetOrderDetailQuery,
  useCreateOrderMutation,
  useUpdateOrderMutation,
  useDeleteOrderMutation,
} = orderApi
