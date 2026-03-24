import { SortButton } from "@/components/custom/sort-button"
import { useHandleShort } from "@/hooks/use-handle-short"
import { cn } from "@/lib/utils"
import type { BrandList } from "@/types/brand"
import { type ColumnDef } from "@tanstack/react-table"
import ActionRow from "./action-row"

export function useBrandColumns({
  sortBy = "",
  sortOrder = "",
  setSortBy,
  setSortOrder,
}: {
  sortBy?: string
  sortOrder?: string
  setSortBy: (value?: string) => void
  setSortOrder: (value?: string) => void
}): ColumnDef<BrandList>[] {
  const handleSort = useHandleShort({
    setSortBy,
    setSortOrder,
  })

  const columns: ColumnDef<BrandList>[] = [
    {
      accessorKey: "logo",
      size: 80,
      header: () => <p className="ml-4 font-bold">Logo</p>,
      cell: ({ row }) => (
        <div className="ml-4">
          {row.original.logo ? (
            <img
              src={row.original.logo}
              alt={row.original.name}
              className="size-20 rounded-lg object-cover"
            />
          ) : (
            <div className="bg-muted size-20 rounded-lg" />
          )}
        </div>
      ),
    },
    {
      accessorKey: "name",
      size: 1000,
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
        <div className="ml-4 flex flex-col justify-center text-wrap">
          <p className="font-semibold capitalize">{row.original.name}</p>
          <p
            className={cn(
              "text-muted-foreground text-xs",
              !row.original.description && "hidden"
            )}
          >
            {row.original.description}
          </p>
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
