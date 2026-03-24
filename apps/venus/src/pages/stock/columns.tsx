import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { useHandleShort, useHandleShortByUrl } from "@/hooks/use-handle-short"
import { cn } from "@/lib/utils"
import {
  setDataStock,
  setOpenStockTransactionDetail,
} from "@/store/stock/action"
import type { StockHistory } from "@/types/stock"
import { type ColumnDef, type Row } from "@tanstack/react-table"
import dayjs from "dayjs"
import { ChevronRightCircle } from "lucide-react"
import { useDispatch } from "react-redux"
import { useSearchParams } from "react-router"
import type { StockData } from "."
import { SortButton } from "../../components/custom/sort-button"
import ActionRow from "./action-row"

export function useStockColumns(): ColumnDef<StockData>[] {
  const dispatch = useDispatch()
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<StockData>[] = [
    {
      id: "expander",
      size: 0,
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => (
        <>
          {row.getCanExpand() ? (
            <div className="flex justify-center">
              <Button
                variant="ghost"
                size="icon_sm"
                {...{
                  onClick: row.getToggleExpandedHandler(),
                  style: { cursor: "pointer" },
                }}
              >
                <ChevronRightCircle
                  size={20}
                  className={cn(
                    "transition-all duration-300",
                    row.getIsExpanded() && "rotate-90"
                  )}
                />
              </Button>
            </div>
          ) : null}
        </>
      ),
    },
    {
      accessorKey: "name",
      size: 100,
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
      meta: {
        colSpan: ({ row }: { row: Row<StockData> }): number =>
          row.original.name === "show-all" ? row.getAllCells().length : 1,
      },
      cell: ({ row }) => {
        const quantityAvailable = row.original.quantityAvailable
        const minStockLevel = row.original.minStockLevel

        if (row.original.name === "show-all") {
          return (
            <Button
              variant="ghost"
              className="w-full"
              onClick={() => {
                console.log({
                  id: row.original.id,
                  productId: row.original.productId,
                })
                dispatch(setOpenStockTransactionDetail(true))
                dispatch(
                  setDataStock({
                    id: row.original.id,
                    productId: row.original.productId,
                  })
                )
              }}
            >
              Tampilkan Semua
            </Button>
          )
        }

        if (quantityAvailable === undefined || minStockLevel === undefined) {
          return (
            <p
              className={cn(
                "text-primary ml-4 font-semibold capitalize",
                row.original.name === "OUT" && "text-red-500",
                row.original.name === "IN" && "text-green-500"
              )}
            >
              {row.original.name === "IN"
                ? "MASUK"
                : row.original.name === "OUT"
                  ? "KELUAR"
                  : row.original.name}
            </p>
          )
        }

        return (
          <div className="ml-4 flex flex-col gap-1">
            <div className="flex items-center gap-2">
              <span className="font-semibold capitalize">
                {row.original.name}
              </span>
              {quantityAvailable < minStockLevel && quantityAvailable === 0 && (
                <Badge variant="destructive">{`Stok ${row.original.quantityAvailable}`}</Badge>
              )}
              {quantityAvailable <= minStockLevel && quantityAvailable > 0 && (
                <Badge variant="warning">{`Stok ${row.original.quantityAvailable}`}</Badge>
              )}
              {quantityAvailable > minStockLevel && (
                <Badge>{`Stok ${row.original.quantityAvailable}`}</Badge>
              )}
            </div>
            <p className="rounded-md text-xs leading-[100%]">
              {row.original.sku}
            </p>
          </div>
        )
      },
    },
    {
      accessorKey: "reason",
      size: 100,
      header: () => <p className="ml-4 font-bold">Alasan</p>,
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => <p className="ml-4">{row.original.reason}</p>,
    },
    {
      accessorKey: "quantity",
      size: 100,
      header: () => <p className="ml-4 font-bold">Jumlah Stok</p>,
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => <p className="ml-4">{row.original.quantity}</p>,
    },
    {
      accessorKey: "createdAt",
      size: 100,
      header: () => <p className="ml-4 font-bold">Tanggal</p>,
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => {
        if (row.original.isParent) return null

        return (
          <div className="ml-4 text-start">
            <p className="font-semibold">
              {dayjs(row.original.createdAt).format("DD-MM-YYYY HH:mm:ss")}
            </p>
            <p className="text-muted-foreground text-xs">
              {row.original.createdByUser?.fullName || ""}
            </p>
          </div>
        )
      },
    },
    {
      accessorKey: "notes",
      size: 100,
      header: () => <p className="ml-4 font-bold">Notes</p>,
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => <p className="ml-4">{row.original.notes}</p>,
    },
    {
      id: "actions",
      size: 20,
      enableResizing: false,
      enableHiding: false,
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => {
        const data = row.original

        if (!data.isParent) return null

        return (
          <div className="mr-4 flex justify-end">
            <ActionRow data={data} />
          </div>
        )
      },
    },
  ]

  return columns
}

export function useStockDetailColumns({
  sortBy = "",
  sortOrder = "",
  setSortBy,
  setSortOrder,
}: {
  sortBy?: string
  sortOrder?: string
  setSortBy: (value?: string) => void
  setSortOrder: (value?: string) => void
}): ColumnDef<StockHistory>[] {
  const handleSort = useHandleShort({
    setSortBy,
    setSortOrder,
  })

  const columns: ColumnDef<StockHistory>[] = [
    {
      accessorKey: "name",
      size: 100,
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
      cell: ({ row }) => {
        return (
          <p
            className={cn(
              "text-primary ml-4 font-semibold capitalize",
              row.original.type === "OUT" && "text-red-500",
              row.original.type === "IN" && "text-green-500"
            )}
          >
            {row.original.type === "IN"
              ? "MASUK"
              : row.original.type === "OUT"
                ? "KELUAR"
                : row.original.type}
          </p>
        )
      },
    },
    {
      accessorKey: "reason",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Alasan"
            keyName="reason"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => <p className="ml-4">{row.original.reason}</p>,
    },
    {
      accessorKey: "quantity",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Jumlah Stok"
            keyName="quantity"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      meta: {
        style: ({
          row,
        }: {
          row: Row<StockData>
        }): React.CSSProperties | undefined =>
          row.original.name === "show-all"
            ? {
                display: "none",
              }
            : undefined,
      },
      cell: ({ row }) => <p className="ml-4">{row.original.quantity}</p>,
    },
    {
      accessorKey: "createdAt",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Tanggal"
            keyName="createdAt"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => {
        return (
          <div className="ml-4 text-start">
            <p className="font-semibold">
              {dayjs(row.original.createdAt).format("DD-MM-YYYY HH:mm:ss")}
            </p>
            <p className="text-muted-foreground text-xs">
              {row.original.createdByUser?.fullName || ""}
            </p>
          </div>
        )
      },
    },
    {
      accessorKey: "notes",
      size: 100,
      header: () => {
        return (
          <SortButton
            title="Catatan"
            keyName="notes"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => <p className="ml-4">{row.original.notes}</p>,
    },
  ]

  return columns
}
