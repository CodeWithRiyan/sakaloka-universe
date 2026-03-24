import {
  type ColumnFiltersState,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { useCallback, useState } from "react"

import { InputSearch } from "@/components/custom/input-search"
import TableData from "@/components/custom/table-data"
import { useGetOrderHistoryListQuery } from "@/store/order/api"
import { useSearchParams } from "react-router"
import { useOrderColumns } from "./columns"
import OrderFilter from "./filter"

export default function FinishedTransaction() {
  const [searchParams, setSearchParams] = useSearchParams()
  // const page = searchParams.get("page") || "1";
  const search = searchParams.get("search")
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")
  const fromDate = searchParams.get("fromDate") as string
  const toDate = searchParams.get("toDate") as string

  const allSearchParams = Object.fromEntries(searchParams.entries())

  const { data: order, isFetching: isLoading } = useGetOrderHistoryListQuery({
    page: Number(searchParams.get("page") || "1"),
    limit: 100,
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
    fromDate,
    toDate,
  })

  const columns = useOrderColumns()
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [rowSelection, setRowSelection] = useState({})

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

  const table = useReactTable({
    data: order?.data || [],
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

  return (
    <div className="flex w-full flex-col gap-5">
      <div className="flex justify-between gap-4">
        <InputSearch
          value={search || ""}
          placeholder="Cari transaksi..."
          onSearch={handleSearch}
          onClear={handleClear}
          isLoading={isLoading}
          debounceMs={500}
          containerClassName="w-full"
          debounceOptions={{
            leading: false,
            trailing: true,
          }}
        />

        <div className="flex-none">
          <OrderFilter />
        </div>
      </div>

      <TableData table={table} isLoading={isLoading} />
    </div>
  )
}
