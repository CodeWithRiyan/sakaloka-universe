import { Badge } from "@/components/ui/badge"
import { useHandleShortByUrl } from "@/hooks/use-handle-short"
import { cn, formatCurrency } from "@/lib/utils"
import type { Bom } from "@/types/bom"
import { type ColumnDef } from "@tanstack/react-table"
import { useSearchParams } from "react-router"
import { SortButton } from "../../components/custom/sort-button"
import BomActionRow from "./action-row"

interface UseBomColumnsProps {
  onEdit?: (bom: Bom) => void
}

export function useBomColumns({ onEdit }: UseBomColumnsProps = {}): ColumnDef<Bom>[] {
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<Bom>[] = [
    {
      accessorKey: "name",
      size: 200,
      header: () => {
        return (
          <SortButton
            title="Nama BOM"
            keyName="name"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <div className="ml-4 flex flex-col justify-center">
          <span className="font-semibold capitalize">
            {row.original.name}
          </span>
          <p
            className={cn(
              "text-muted-foreground text-xs",
              !row.original.notes && "hidden"
            )}
          >
            {row.original.notes}
          </p>
        </div>
      ),
    },
    {
      accessorKey: "productId",
      size: 150,
      header: "Produk ID",
      cell: ({ row }) => (
        <p className="ml-4 text-xs text-muted-foreground">
          {row.original.productId}
        </p>
      ),
    },
    {
      accessorKey: "version",
      size: 80,
      header: () => {
        return (
          <SortButton
            title="Versi"
            keyName="version"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => <p className="ml-4">{row.original.version}</p>,
    },
    {
      accessorKey: "status",
      size: 100,
      header: "Status",
      cell: ({ row }) => {
        const status = row.original.status
        const variant = status === "active" ? "default" : status === "draft" ? "secondary" : "outline"
        return (
          <Badge variant={variant} className="ml-4">
            {status}
          </Badge>
        )
      },
    },
    {
      accessorKey: "totalCost",
      size: 120,
      header: () => {
        return (
          <SortButton
            title="Total Biaya"
            keyName="totalCost"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">
          {row.original.totalCost
            ? `Rp ${formatCurrency(row.original.totalCost)}`
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

        return <BomActionRow data={data} onEdit={onEdit} />
      },
    },
  ]

  return columns
}