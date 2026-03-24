import { InputSearch } from "@/components/custom/input-search"
import TableData from "@/components/custom/table-data"
import { Button } from "@/components/ui/button"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import { setOpenStockForm } from "@/store/stock/action"
import { useGetStockListQuery } from "@/store/stock/api"
import type { StockHistory } from "@/types/stock"
import {
  type ColumnFiltersState,
  type ExpandedState,
  getCoreRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { useCallback, useEffect, useMemo, useState } from "react"
import { LiaEdit } from "react-icons/lia"
import { useSearchParams } from "react-router"
import { useStockColumns } from "./columns"
import StockTransactionDetail from "./detail"
import StockManagementForm from "./form"

export interface StockData {
  id?: string
  productId?: string
  name: string
  sku?: string
  quantityAvailable?: number
  minStockLevel?: number
  quantity?: number
  isParent?: boolean
  reason?: string
  notes?: string
  createdAt?: string
  createdBy?: string
  createdByUser?: StockHistory["createdByUser"]
  subData?: StockData[]
}

export default function StockManagement() {
  const dispatch = useAppDispatch()
  const { isOpenStockForm, isOpenStockTransactionDetail } = useAppSelector(
    (state) => state.stock
  )
  const [searchParams, setSearchParams] = useSearchParams()
  const page = searchParams.get("page") || "1"
  const search = searchParams.get("search")
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")
  const allSearchParams = Object.fromEntries(searchParams.entries())

  const { data: stock, isLoading } = useGetStockListQuery({
    page: Number(page),
    limit: 10,
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
  })

  const columns = useStockColumns()
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [rowSelection, setRowSelection] = useState({})
  const [expanded, setExpanded] = useState<ExpandedState>({})

  const handleSearch = useCallback(
    (value: string) => {
      setSearchParams(
        {
          ...allSearchParams,
          search: value,
        },
        {
          replace: true,
        }
      )
    },
    [allSearchParams, setSearchParams]
  )

  const handleClear = useCallback(() => {
    setSearchParams(
      {
        ...allSearchParams,
        search: "",
      },
      {
        replace: true,
      }
    )
  }, [allSearchParams, setSearchParams])

  const data: StockData[] = useMemo(
    () =>
      stock?.data?.data?.map((stock) => {
        const stockHistory: StockData[] = stock.stockHistory
          .filter((_h, i) => i < 5)
          .map((history) => ({
            name: history.type,
            quantity: history.quantity,
            reason: history.reason,
            notes: history.notes,
            createdAt: history.createdAt,
            createdBy: history.createdBy,
            createdByUser: history.createdByUser,
          }))

        return {
          id: stock.id,
          productId: stock.productId,
          name: stock.product.name,
          sku: stock.product.sku,
          quantityAvailable: stock.quantityAvailable,
          minStockLevel: stock.minStockLevel,
          isParent: true,
          subData:
            stock.stockHistory.length < 5
              ? stockHistory
              : [
                  ...stockHistory,
                  {
                    name: "show-all",
                    id: stock.id,
                    productId: stock.productId,
                  },
                ],
        }
      }) || [],
    [stock?.data?.data]
  )

  useEffect(() => {
    if (data.length > 0) {
      const defaultExpanded = data.reduce((acc, _row, index) => {
        return { ...acc, [index]: true }
      }, {})
      setExpanded(defaultExpanded)
    }
  }, [data])

  const table = useReactTable({
    data,
    columns,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
    onRowSelectionChange: setRowSelection,
    getSubRows: (row) => row.subData,
    onExpandedChange: setExpanded,
    manualPagination: true,
    state: {
      expanded,
      columnFilters,
      rowSelection,
      pagination: {
        pageIndex: Number(page) - 1,
        pageSize: Object.keys(expanded).length ? 9999 : 10, // Dynamic page size
      },
    },
  })
  return (
    <div className="flex w-full flex-col gap-5 pt-2">
      <div className="flex justify-between gap-10">
        <div className="flex w-full items-center gap-3">
          <InputSearch
            value={search || ""}
            placeholder="Cari data..."
            onSearch={handleSearch}
            onClear={handleClear}
            isLoading={isLoading}
            debounceMs={500}
            className="w-full max-w-lg"
            debounceOptions={{
              leading: false,
              trailing: true,
            }}
          />
        </div>

        <div className="flex gap-2">
          <Button
            title="Atur Stok"
            className="flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
            onClick={() => dispatch(setOpenStockForm(true))}
          >
            <LiaEdit className="text-xl lg:hidden xl:inline" />
            <span className="hidden lg:inline">Atur Stok</span>
          </Button>
        </div>
      </div>

      <TableData
        table={table}
        isLoading={isLoading}
        pagination={stock?.data.pagination}
      />

      {isOpenStockTransactionDetail && <StockTransactionDetail />}
      {isOpenStockForm && <StockManagementForm />}
    </div>
  )
}
