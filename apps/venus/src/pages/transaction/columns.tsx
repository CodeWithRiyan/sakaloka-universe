import { Badge } from "@/components/ui/badge"
import { useHandleShortByUrl } from "@/hooks/use-handle-short"
import { formatCurrency } from "@/lib/utils"
import type { OrderList } from "@/types/order"
import { type ColumnDef } from "@tanstack/react-table"
import dayjs from "dayjs"
import { useSearchParams } from "react-router"
import { SortButton } from "../../components/custom/sort-button"
import ActionRow from "./action-row"

export function useOrderColumns(): ColumnDef<OrderList>[] {
  const [searchParams] = useSearchParams()
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const handleSort = useHandleShortByUrl()

  const columns: ColumnDef<OrderList>[] = [
    {
      accessorKey: "customerName",
      minSize: 200,
      maxSize: 500,
      header: () => {
        return (
          <SortButton
            title="Pelanggan"
            keyName="customerName"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <div className="flex flex-col">
          <strong className="px-4 capitalize">
            {row.original.customerName}
          </strong>
          <p className="text-muted-foreground px-4 text-xs">
            {row.original.orderNumber}
          </p>
        </div>
      ),
    },
    {
      accessorKey: "createdAt",
      enableResizing: false,
      size: 50,
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
        const date = dayjs(row.getValue("createdAt")).format("DD-MM-YYYY HH:mm")
        return <div className="truncate px-4 capitalize">{date}</div>
      },
    },
    {
      accessorKey: "type",
      size: 20,
      maxSize: 50,
      enableResizing: false,
      header: () => {
        return (
          <SortButton
            title="Tipe Transaksi"
            keyName="type"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <Badge variant={row.original.type === "DINEIN" ? "default" : "success"}>
          {row.original.type === "DINEIN" ? "Dine In" : "Take Away"}
        </Badge>
      ),
    },
    {
      accessorKey: "totalAmount",
      enableResizing: false,
      size: 50,
      header: () => {
        return (
          <SortButton
            title="Total Transaksi"
            keyName="totalAmount"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">
          {row.original.totalAmount && row.original.totalAmount !== "0"
            ? `Rp ${formatCurrency(row.original.totalAmount)}`
            : "-"}
        </p>
      ),
    },
    {
      accessorKey: "totalPayment",
      enableResizing: false,
      size: 50,
      header: () => {
        return (
          <SortButton
            title="Total Pembayaran"
            keyName="paidAmount"
            sortBy={sortBy}
            sortOrder={sortOrder}
            onSort={handleSort}
          />
        )
      },
      cell: ({ row }) => (
        <p className="ml-4">
          {row.original.paidAmount && row.original.paidAmount !== "0"
            ? `Rp ${formatCurrency(row.original.paidAmount)}`
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
