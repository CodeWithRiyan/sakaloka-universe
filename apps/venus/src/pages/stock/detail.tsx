"use client"

import { InputSearch } from "@/components/custom/input-search"
import Modal from "@/components/custom/modal"
import TableData from "@/components/custom/table-data"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  type ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@/components/ui/chart"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import {
  setDataStock,
  setOpenStockForm,
  setOpenStockTransactionDetail,
} from "@/store/stock/action"
import { useGetStockListQuery } from "@/store/stock/api"
import {
  type ColumnFiltersState,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { Plus } from "lucide-react"
import { useMemo, useState } from "react"
import { CartesianGrid, Line, LineChart, XAxis } from "recharts"
import { useStockDetailColumns } from "./columns"

export default function StockTransactionDetail() {
  const dispatch = useAppDispatch()
  const {
    isOpenStockTransactionDetail: isOpen,
    data: { id },
  } = useAppSelector((state) => state.stock)
  const { data: stockList } = useGetStockListQuery({})
  const stockDetail = useMemo(
    () => stockList?.data.data?.find((f) => f.id === id),
    [id, stockList?.data.data]
  )

  const quantityAvailable = stockDetail?.quantityAvailable || 0
  const minStockLevel = stockDetail?.minStockLevel || 0

  const [search, setSearch] = useState<string>("")
  const [sortOrder, setSortOrder] = useState<string | undefined>()
  const [sortBy, setSortBy] = useState<string | undefined>()

  const { isFetching: isLoading } = useGetStockListQuery({
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
  })

  const columns = useStockDetailColumns({
    sortBy,
    sortOrder,
    setSortBy,
    setSortOrder,
  })

  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [rowSelection, setRowSelection] = useState({})

  const handleSearch = (value: string) => {
    setSearch(value)
  }

  const handleClear = () => {
    setSearch("")
  }

  const table = useReactTable({
    data: stockDetail?.stockHistory || [],
    columns,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onRowSelectionChange: setRowSelection,
    state: {
      columnFilters,
      rowSelection,
    },
  })

  const handleCancel = () => {
    dispatch(setOpenStockTransactionDetail(false))
    dispatch(
      setDataStock({
        id: undefined,
        productId: undefined,
      })
    )
  }

  const chartData = [
    { month: "January", in: 186, out: 80 },
    { month: "February", in: 305, out: 200 },
    { month: "March", in: 237, out: 120 },
    { month: "April", in: 73, out: 190 },
    { month: "May", in: 209, out: 130 },
    { month: "June", in: 214, out: 140 },
  ]
  const chartConfig = {
    in: {
      label: "Masuk",
      color: "green",
    },
    out: {
      label: "Keluar",
      color: "red",
    },
  } satisfies ChartConfig

  return (
    <Modal
      open={isOpen}
      onCancel={handleCancel}
      footer={<></>}
      className="w-full sm:max-w-2xl md:max-w-4xl lg:max-w-6xl"
    >
      <div className="flex w-full flex-col gap-5 pt-4">
        <div className="grid grid-cols-1 md:grid-cols-2">
          <div className="flex gap-4">
            <img
              src={stockDetail?.product.imageUrl}
              alt={stockDetail?.product.name}
              className="size-40 flex-none rounded-lg object-cover"
            />
            <div className="">
              {quantityAvailable < minStockLevel && quantityAvailable === 0 && (
                <Badge variant="destructive">{`Stok ${stockDetail?.quantityAvailable}`}</Badge>
              )}
              {quantityAvailable <= minStockLevel && quantityAvailable > 0 && (
                <Badge variant="warning">{`Stok ${stockDetail?.quantityAvailable}`}</Badge>
              )}
              {quantityAvailable > minStockLevel && (
                <Badge>{`Stok ${stockDetail?.quantityAvailable}`}</Badge>
              )}
              <p className="truncate text-xl font-bold">
                {stockDetail?.product.name}
              </p>
              <p className="truncate text-sm">{stockDetail?.product.sku}</p>
              <Button
                variant="default"
                className="mt-4 flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
                onClick={() => {
                  dispatch(setOpenStockForm(true))
                }}
              >
                <Plus strokeWidth={2.6} size={20} />
                <span>Atur Stock</span>
              </Button>
            </div>
          </div>
          <div className="space-y-4">
            <div>
              <h4 className="text-xl font-semibold">
                Data Transaksi Keluar Masuk Stok
              </h4>
              <p className="text-muted-foreground text-sm">
                Januari - Juni 2025
              </p>
            </div>
            <ChartContainer config={chartConfig} className="h-[140px] w-full">
              <LineChart
                accessibilityLayer
                data={chartData}
                margin={{
                  left: 12,
                  right: 12,
                }}
              >
                <CartesianGrid vertical={false} />
                <XAxis
                  dataKey="month"
                  tickLine={false}
                  axisLine={false}
                  tickMargin={8}
                  tickFormatter={(value) => value.slice(0, 3)}
                />
                <ChartTooltip
                  cursor={false}
                  content={<ChartTooltipContent />}
                />
                <Line
                  dataKey="in"
                  type="monotone"
                  stroke="var(--color-in)"
                  strokeWidth={2}
                  dot={false}
                />
                <Line
                  dataKey="out"
                  type="monotone"
                  stroke="var(--color-out)"
                  strokeWidth={2}
                  dot={false}
                />
              </LineChart>
            </ChartContainer>
          </div>
        </div>
        <div className="flex justify-between gap-10">
          <InputSearch
            value={search || ""}
            placeholder="Cari transaksi..."
            onSearch={handleSearch}
            onClear={handleClear}
            isLoading={isLoading}
            debounceMs={500}
            containerClassName="w-full"
            className="w-full"
            debounceOptions={{
              leading: false,
              trailing: true,
            }}
          />
        </div>

        <TableData table={table} isLoading={isLoading} />
      </div>
    </Modal>
  )
}
