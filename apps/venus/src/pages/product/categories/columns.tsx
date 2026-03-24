import { SortButton } from "@/components/custom/sort-button"
import { useHandleShort } from "@/hooks/use-handle-short"
import { cn } from "@/lib/utils"
import type { CategoriesList } from "@/types/categories"
import { type ColumnDef } from "@tanstack/react-table"
import ActionRow from "./action-row"

export function useCategoriesColumns({
  sortBy = "",
  sortOrder = "",
  setSortBy,
  setSortOrder,
}: {
  sortBy?: string
  sortOrder?: string
  setSortBy: (value?: string) => void
  setSortOrder: (value?: string) => void
}): ColumnDef<CategoriesList>[] {
  const handleSort = useHandleShort({
    setSortBy,
    setSortOrder,
  })

  const columns: ColumnDef<CategoriesList>[] = [
    {
      accessorKey: "imageUrl",
      size: 80,
      header: () => <p className="ml-4 font-bold">Gambar</p>,
      cell: ({ row }) => (
        <div className="ml-4">
          {row.original.imageUrl ? (
            <img
              src={row.original.imageUrl}
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
        <div className="ml-4 flex max-w-2xs flex-col justify-center">
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
