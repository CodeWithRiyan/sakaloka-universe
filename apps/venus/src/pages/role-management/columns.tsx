import { Badge } from "@/components/ui/badge"
import { useHandleShortByUrl } from "@/hooks/use-handle-short"
import type { RoleList } from "@/types/role"
import { type ColumnDef } from "@tanstack/react-table"
import { useSearchParams } from "react-router"
import { SortButton } from "../../components/custom/sort-button"
import ActionRow from "./action-row"

export function useRoleColumns(): ColumnDef<RoleList>[] {
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<RoleList>[] = [
    {
      accessorKey: "name",
      size: 500,
      minSize: 400,
      maxSize: 5000,
      header: () => {
        return (
          <SortButton
            title="Nama"
            keyName="name"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => <p className="ml-4 capitalize">{row.original.name}</p>,
    },
    {
      accessorKey: "permissions",
      size: 300,
      header: "Hak Akses",
      cell: ({ row }) => (
        <div className="flex flex-wrap gap-2 capitalize">
          {Object.keys(row.original.permissions).map((key) => {
            const data = row.original.permissions[key]
            return (
              <Badge key={key}>
                <span>{`${key} `}</span>
                <strong>{`(${data.length})`}</strong>
              </Badge>
            )
          })}
        </div>
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
