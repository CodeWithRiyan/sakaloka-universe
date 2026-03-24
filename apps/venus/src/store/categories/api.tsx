import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams, ResponseError } from "@/types"
import {
  type CategoriesDetailResponse,
  type CategoriesListResponse,
} from "@/types/categories"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const categoriesApi = createApi({
  reducerPath: "categoriesApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Categories", "CategoriesList"],
  endpoints: (builder) => ({
    getCategoriesList: builder.query<
      CategoriesListResponse,
      DefaultSearchParams
    >({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/products/categories?${queryString.stringify({
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
            type: "Categories" as const,
            id,
          })) || []

        return result
          ? [...tags, { type: "CategoriesList" as const, id: "LIST" }]
          : [{ type: "CategoriesList" as const, id: "LIST" }]
      },
    }),
    getCategoriesDetail: builder.query<CategoriesDetailResponse, string>({
      query: (id) => ({
        url: `/products/categories/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [
        { type: "Categories" as const, id },
      ],
    }),
    createCategories: builder.mutation<CategoriesDetailResponse, FormData>({
      query: (data) => ({
        url: `/products/categories`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [
        { type: "CategoriesList" as const, id: "LIST" },
        { type: "Categories" as const, id: "CURRENT" },
      ],
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
    updateCategories: builder.mutation<
      CategoriesDetailResponse,
      {
        id: string
        payload: FormData
      }
    >({
      query: ({ id, payload }) => ({
        url: `/products/categories/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Categories" as const, id },
        { type: "CategoriesList" as const, id: "LIST" },
        { type: "Categories" as const, id: "CURRENT" },
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
    deleteCategories: builder.mutation<CategoriesDetailResponse, string>({
      query: (id) => ({
        url: `/products/categories/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Categories" as const, id },
        { type: "CategoriesList" as const, id: "LIST" },
        { type: "Categories" as const, id: "CURRENT" },
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
  useGetCategoriesListQuery,
  useGetCategoriesDetailQuery,
  useCreateCategoriesMutation,
  useUpdateCategoriesMutation,
  useDeleteCategoriesMutation,
} = categoriesApi
