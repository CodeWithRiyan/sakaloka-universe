import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams, ResponseError } from "@/types"
import { type BrandDetailResponse, type BrandListResponse } from "@/types/brand"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const brandApi = createApi({
  reducerPath: "brandApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Brand", "BrandList"],
  endpoints: (builder) => ({
    getBrandList: builder.query<BrandListResponse, DefaultSearchParams>({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/products/brands?${queryString.stringify({
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
      providesTags: (result) => {
        const tags =
          result?.data?.data?.map(({ id }) => ({
            type: "Brand" as const,
            id,
          })) || []

        return result
          ? [...tags, { type: "BrandList" as const, id: "LIST" }]
          : [{ type: "BrandList" as const, id: "LIST" }]
      },
    }),
    getBrandDetail: builder.query<BrandDetailResponse, string>({
      query: (id) => ({
        url: `/products/brands/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Brand" as const, id }],
    }),
    createBrand: builder.mutation<BrandDetailResponse, FormData>({
      query: (data) => ({
        url: `/products/brands`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "BrandList" as const, id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Kategori berhasil dibuat!")
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
    updateBrand: builder.mutation<
      BrandDetailResponse,
      {
        id: string
        payload: FormData
      }
    >({
      query: ({ id, payload }) => ({
        url: `/products/brands/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Brand" as const, id },
        { type: "BrandList" as const, id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Kategori berhasil diubah!")
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
    deleteBrand: builder.mutation<BrandDetailResponse, string>({
      query: (id) => ({
        url: `/products/brands/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Brand" as const, id },
        { type: "BrandList" as const, id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Kategori berhasil dihapus!")
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
  useGetBrandListQuery,
  useGetBrandDetailQuery,
  useCreateBrandMutation,
  useUpdateBrandMutation,
  useDeleteBrandMutation,
} = brandApi
