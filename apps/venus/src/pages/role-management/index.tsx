"use client"

import { InputSearch } from "@/components/custom/input-search"
import TableData from "@/components/custom/table-data"
import { Button } from "@/components/ui/button"
import useBreakpoint from "@/hooks/use-breakpoint"
import { useGetRoleListQuery } from "@/store/role/api"
import {
  type ColumnFiltersState,
  type VisibilityState,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { useCallback, useEffect, useState } from "react"
import { PiPlus } from "react-icons/pi"
import { useSearchParams } from "react-router"
import { useRoleColumns } from "./columns"
import RoleManagementForm from "./form"

export default function RoleManagement() {
  const [isOpenForm, setIsOpenForm] = useState(false)
  const [searchParams, setSearchParams] = useSearchParams()
  const page = searchParams.get("page") || "1"
  const search = searchParams.get("search")
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")
  const allSearchParams = Object.fromEntries(searchParams.entries())

  const { data: role, isLoading } = useGetRoleListQuery({
    page: Number(page),
    limit: 10,
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
  })

  const columns = useRoleColumns()
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({})
  const [rowSelection, setRowSelection] = useState({})
  const { lg } = useBreakpoint()

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
    data: role?.data.data || [],
    columns,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      columnFilters,
      columnVisibility,
      rowSelection,
    },
  })

  useEffect(() => {
    if (lg) {
      setColumnVisibility({
        name: true,
        permissions: true,
      })
    } else {
      setColumnVisibility({
        name: true,
        permissions: false,
      })
    }
  }, [lg])

  return (
    <div className="flex w-full flex-col gap-5 pt-2">
      <div className="space-y-1 pb-4">
        <p className="text-2xl font-medium">Manajemen Role</p>
        <hr />
      </div>

      <div className="flex justify-between gap-10">
        <div className="flex w-full items-center gap-3">
          <InputSearch
            value={search || ""}
            placeholder="Cari role..."
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

        <Button
          variant="default"
          className="flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
          onClick={() => setIsOpenForm(true)}
        >
          <PiPlus className="text-xl lg:hidden xl:inline" />
          <span className="hidden lg:inline">Tambah Role</span>
        </Button>
      </div>

      <TableData
        table={table}
        isLoading={isLoading}
        pagination={role?.data.pagination}
      />

      <RoleManagementForm isOpen={isOpenForm} setIsOpen={setIsOpenForm} />
    </div>
  )
}
