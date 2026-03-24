import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams, ResponseError } from "@/types"
import {
  type BranchCurrentResponse,
  type BranchDetailResponse,
  type BranchListResponse,
  type BranchPayload,
} from "@/types/branch"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"
import { brandApi } from "../brand/api"
import { categoriesApi } from "../categories/api"
import { orderApi } from "../order/api"
import { productApi } from "../product/api"
import { roleApi } from "../role/api"
import { stockApi } from "../stock/api"
import { userApi } from "../user/api"

export const branchApi = createApi({
  reducerPath: "branchApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Branch", "BranchList"],
  endpoints: (builder) => ({
    getBranchList: builder.query<BranchListResponse, DefaultSearchParams>({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/organizations?${queryString.stringify({
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
                type: "Branch" as const,
                id,
              })),
              { type: "BranchList", id: "LIST" },
            ]
          : [{ type: "BranchList", id: "LIST" }],
    }),
    getBranchDetail: builder.query<BranchDetailResponse, string>({
      query: (id) => ({
        url: `/organizations/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Branch", id }],
    }),
    getBranchCurrent: builder.query<BranchCurrentResponse, void>({
      query: () => ({
        url: `/organizations/current`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: [{ type: "Branch", id: "CURRENT" }],
    }),
    selectBranch: builder.mutation<
      BranchCurrentResponse,
      { organizationId: string }
    >({
      query: (data) => ({
        url: `/organizations/select`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "Branch", id: "CURRENT" }],
      onQueryStarted: async (_arg, { dispatch, queryFulfilled }) => {
        await queryFulfilled

        dispatch(
          roleApi.util.invalidateTags([{ type: "RoleList", id: "LIST" }])
        )
        dispatch(
          userApi.util.invalidateTags([{ type: "UserList", id: "LIST" }])
        )
        dispatch(
          productApi.util.invalidateTags([{ type: "ProductList", id: "LIST" }])
        )
        dispatch(
          brandApi.util.invalidateTags([{ type: "BrandList", id: "LIST" }])
        )
        dispatch(
          categoriesApi.util.invalidateTags([
            { type: "CategoriesList", id: "LIST" },
          ])
        )
        dispatch(
          stockApi.util.invalidateTags([{ type: "StockList", id: "LIST" }])
        )
        dispatch(
          orderApi.util.invalidateTags([
            { type: "Menu" },
            { type: "OrderList", id: "LIST" },
          ])
        )
      },
    }),
    createBranch: builder.mutation<BranchDetailResponse, BranchPayload>({
      query: (data) => ({
        url: `/organizations`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [
        { type: "BranchList", id: "LIST" },
        { type: "Branch", id: "CURRENT" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Cabang berhasil dibuat!")
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
    updateBranch: builder.mutation<
      BranchDetailResponse,
      {
        id: string
        payload: BranchPayload
      }
    >({
      query: ({ id, payload }) => ({
        url: `/organizations/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Branch", id },
        { type: "BranchList", id: "LIST" },
        { type: "Branch", id: "CURRENT" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Cabang berhasil diubah!")
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
    deleteBranch: builder.mutation<BranchDetailResponse, string>({
      query: (id) => ({
        url: `/organizations/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Branch", id },
        { type: "BranchList", id: "LIST" },
        { type: "Branch", id: "CURRENT" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Cabang berhasil dihapus!")
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
  useGetBranchListQuery,
  useGetBranchDetailQuery,
  useGetBranchCurrentQuery,
  useSelectBranchMutation,
  useCreateBranchMutation,
  useUpdateBranchMutation,
  useDeleteBranchMutation,
} = branchApi
