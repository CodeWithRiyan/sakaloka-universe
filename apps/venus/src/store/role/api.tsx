import ToastDescriptionList from "@/components/custom/toast-description-list"
import { getTokenData } from "@/lib/auth"
import type { DefaultSearchParams, ResponseError } from "@/types"
import {
  type PermissionResponse,
  type RoleDetailResponse,
  type RoleListResponse,
  type RolePayload,
} from "@/types/role"
import { createApi } from "@reduxjs/toolkit/query/react"
import queryString from "query-string"
import { toast } from "sonner"
import { baseQueryWithAuth } from "../base-query"

export const roleApi = createApi({
  reducerPath: "roleApi",
  baseQuery: baseQueryWithAuth,
  tagTypes: ["Role", "RoleList"],
  endpoints: (builder) => ({
    getRoleList: builder.query<RoleListResponse, DefaultSearchParams>({
      query: ({ page = 1, limit = 10, search, sortBy, sortOrder }) => ({
        url: `/roles?${queryString.stringify({
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
                type: "Role" as const,
                id,
              })),
              { type: "RoleList", id: "LIST" },
            ]
          : [{ type: "RoleList", id: "LIST" }],
    }),
    getRoleDetail: builder.query<RoleDetailResponse, string>({
      query: (id) => ({
        url: `/roles/${id}`,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: (_result, _error, id) => [{ type: "Role", id }],
    }),
    getPermission: builder.query<PermissionResponse, void>({
      query: () => ({
        url: "/roles/permissions",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      providesTags: () => [{ type: "Role", id: "PERMISSION" }],
    }),
    createRole: builder.mutation<RoleDetailResponse, RolePayload>({
      query: (data) => ({
        url: `/roles`,
        method: "POST",
        body: data,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: [{ type: "RoleList", id: "LIST" }],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Role berhasil dibuat!")
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
    updateRole: builder.mutation<
      RoleDetailResponse,
      {
        id: string
        payload: RolePayload
      }
    >({
      query: ({ id, payload }) => ({
        url: `/roles/${id}`,
        method: "PATCH",
        body: payload,
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, { id }) => [
        { type: "Role", id },
        { type: "RoleList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Role berhasil diubah!")
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
    deleteRole: builder.mutation<RoleDetailResponse, string>({
      query: (id) => ({
        url: `/roles/${id}`,
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${getTokenData()?.access_token}`,
        },
      }),
      invalidatesTags: (_result, _error, id) => [
        { type: "Role", id },
        { type: "RoleList", id: "LIST" },
      ],
      onQueryStarted: async (_arg, { queryFulfilled }) => {
        try {
          const result = await queryFulfilled

          if (result.data.success) {
            toast.success("Role berhasil dihapus!")
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
  useGetRoleListQuery,
  useGetRoleDetailQuery,
  useGetPermissionQuery,
  useCreateRoleMutation,
  useUpdateRoleMutation,
  useDeleteRoleMutation,
} = roleApi
