import { Badge } from "@/components/ui/badge"
import { useHandleShortByUrl } from "@/hooks/use-handle-short"
import type { BranchList } from "@/types/branch"
import { type ColumnDef } from "@tanstack/react-table"
import { useSearchParams } from "react-router"
import { SortButton } from "../../components/custom/sort-button"
import ActionRow from "./action-row"

export function useBranchColumns(): ColumnDef<BranchList>[] {
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<BranchList>[] = [
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
      cell: ({ row }) => (
        <div className="ml-4 flex items-center gap-2">
          <p className="capitalize">{row.original.name}</p>
          {!row.original.parentId && <Badge>Pusat</Badge>}
        </div>
      ),
    },
    {
      accessorKey: "address",
      size: 100,
      header: () => <p className="ml-4 font-bold">Alamat</p>,
      cell: ({ row }) => <p className="ml-4">{row.original.address}</p>,
    },
    {
      accessorKey: "phone",
      size: 100,
      header: () => <p className="ml-4 font-bold">Nomor Telepon</p>,
      cell: ({ row }) => <p className="ml-4">{row.original.phone}</p>,
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
