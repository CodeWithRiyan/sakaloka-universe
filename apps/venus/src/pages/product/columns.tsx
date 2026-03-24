import { Badge } from "@/components/ui/badge"
import { useHandleShortByUrl } from "@/hooks/use-handle-short"
import { cn, formatCurrency } from "@/lib/utils"
import type { ProductList } from "@/types/product"
import { type ColumnDef } from "@tanstack/react-table"
import { useSearchParams } from "react-router"
import { SortButton } from "../../components/custom/sort-button"
import ActionRow from "./action-row"

export function useProductColumns(): ColumnDef<ProductList>[] {
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<ProductList>[] = [
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
      size: 200,
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
        <div className="ml-4 flex flex-col justify-center">
          <div className="flex items-center gap-2">
            <span className="font-semibold capitalize">
              {row.original.name}
            </span>
            {row.original.isFeatured && <Badge>Featured</Badge>}
          </div>
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
      accessorKey: "categoryName",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Kategori"
            keyName="categoryName"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">{row.original.category?.name || "-"}</p>
      ),
    },
    {
      accessorKey: "brandName",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Brand"
            keyName="brandName"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">{row.original.brand?.name || "-"}</p>
      ),
    },
    {
      accessorKey: "costPrice",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Harga Beli"
            keyName="costPrice"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">
          {row.original.costPrice
            ? `Rp ${formatCurrency(row.original.costPrice)}`
            : "-"}
        </p>
      ),
    },
    {
      accessorKey: "basePrice",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Harga Jual"
            keyName="basePrice"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">
          {row.original.basePrice
            ? `Rp ${formatCurrency(row.original.basePrice)}`
            : "-"}
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
