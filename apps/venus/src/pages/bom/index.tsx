"use client"

import { InputSearch } from "@/components/custom/input-search"
import TableData from "@/components/custom/table-data"
import { Button } from "@/components/ui/button"
import { useGetBomListQuery } from "@/store/bom/api"
import {
  type ColumnFiltersState,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { Plus } from "lucide-react"
import { useCallback, useState } from "react"
import { useSearchParams } from "react-router"
import { useBomColumns } from "./columns"
import BomForm from "./form"

export default function BomManagement() {
  const [isOpenForm, setIsOpenForm] = useState(false)
  const [searchParams, setSearchParams] = useSearchParams()
  const page = searchParams.get("page") || "1"
  const search = searchParams.get("search")
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")

  const allSearchParams = Object.fromEntries(searchParams.entries())

  const { data: bomData, isFetching: isLoading } = useGetBomListQuery({
    page: Number(page),
    limit: 10,
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
  })

  const columns = useBomColumns({
    onEdit: () => {
      setIsOpenForm(true)
    },
  })
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
    data: bomData?.data.data ?? [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onColumnFiltersChange: setColumnFilters,
    onRowSelectionChange: setRowSelection,
    state: {
      columnFilters,
      rowSelection,
    },
    pageCount: bomData?.data.pagination.pages ?? 0,
    manualPagination: true,
  })

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-col justify-between gap-4 md:flex-row md:items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">BOM</h1>
          <p className="text-muted-foreground">Kelola Bill of Materials</p>
        </div>
        <Button onClick={() => setIsOpenForm(true)}>
          <Plus className="mr-2 h-4 w-4" />
          Tambah BOM
        </Button>
      </div>

      <div className="flex items-center gap-2">
        <InputSearch
          onSearch={handleSearch}
          onClear={handleClear}
          defaultValue={search ?? ""}
          placeholder="Cari BOM..."
        />
      </div>

      <TableData
        table={table}
        isLoading={isLoading}
        pagination={bomData?.data.pagination}
      />

      {isOpenForm && (
        <BomForm
          open={isOpenForm}
          onCancel={() => setIsOpenForm(false)}
          onClose={() => setIsOpenForm(false)}
        />
      )}
    </div>
  )
}
