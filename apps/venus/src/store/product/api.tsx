import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams, ResponseError } from "@/types"
import {
  type ProductDetailResponse,
  type ProductListResponse,
} from "@/types/product"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const productApi = createApi({
  reducerPath: "productApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Product", "ProductList"],
  endpoints: (builder) => ({
    getProductList: builder.query<
      ProductListResponse,
      DefaultSearchParams & {
        categoryId?: string
        brandId?: string
        isFeatured?: boolean
      }
    >({
      query: ({
        page = 1,
        limit = 10,
        search,
        sortBy,
        sortOrder,
        categoryId,
        brandId,
        isFeatured,
      }) => ({
        url: `/products?${queryString.stringify({
          page,
          limit,
          search,
          sortBy,
          sortOrder,
          categoryId,
          brandId,
          isFeatured,
        })}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (result) =>
        result
          ? [
              ...result.data.data.map(({ id }) => ({
                type: "Product" as const,
                id,
              })),
              { type: "ProductList", id: "LIST" },
            ]
          : [{ type: "ProductList", id: "LIST" }],
    }),
    getProductDetail: builder.query<ProductDetailResponse, string>({
      query: (id) => ({
        url: `/products/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Product", id }],
    }),
    createProduct: builder.mutation<ProductDetailResponse, FormData>({
      query: (data) => ({
        url: `/products`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "ProductList", id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled
          if (result.data.success) {
            toast.success("Produk berhasil dibuat!")
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
    updateProduct: builder.mutation<
      ProductDetailResponse,
      {
        id: string
        payload: FormData
      }
    >({
      query: ({ id, payload }) => ({
        url: `/products/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Product", id },
        { type: "ProductList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled
          if (result.data.success) {
            toast.success("Produk berhasil diubah!")
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
    deleteProduct: builder.mutation<ProductDetailResponse, string>({
      query: (id) => ({
        url: `/products/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Product", id },
        { type: "ProductList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled
          if (result.data.success) {
            toast.success("Produk berhasil dihapus!")
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
  useGetProductListQuery,
  useGetProductDetailQuery,
  useCreateProductMutation,
  useUpdateProductMutation,
  useDeleteProductMutation,
} = productApi
