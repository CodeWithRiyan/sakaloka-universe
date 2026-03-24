import { useHandleShortByUrl } from "@/hooks/use-handle-short"
import type { UserList } from "@/types/user"
import { type ColumnDef } from "@tanstack/react-table"
import dayjs from "dayjs"
import { useSearchParams } from "react-router"
import { SortButton } from "../../components/custom/sort-button"
import ActionRow from "./action-row"

export function useUserColumns(): ColumnDef<UserList>[] {
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<UserList>[] = [
    {
      accessorKey: "fullName",
      size: 500,
      minSize: 400,
      maxSize: 5000,
      header: () => {
        return (
          <SortButton
            title="Nama"
            keyName="fullName"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4 capitalize">{row.original.fullName}</p>
      ),
    },
    {
      accessorKey: "roleName",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Role"
            keyName="roleId"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => <p className="ml-4">{row.original.role.name}</p>,
    },
    {
      accessorKey: "organizationName",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Cabang"
            keyName="organizationName"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">{row.original.organization.name}</p>
      ),
    },
    {
      accessorKey: "email",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Email"
            keyName="email"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => <p className="ml-4">{row.original.email}</p>,
    },
    {
      accessorKey: "lastLoginAt",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Terakhir Login"
            keyName="lastLoginAt"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">
          {row.original.lastLoginAt
            ? dayjs(row.original.lastLoginAt).format("DD-MM-YYYY HH:mm:ss")
            : "Belum pernah login"}
        </p>
      ),
    },
    {
      id: "actions",
      size: 20,
      enableResizing: false,
      enableHiding: false,
      cell: ({ row }) => {
        const data = row.original

        return <ActionRow data={data} />
      },
    },
  ]

  return columns
}
