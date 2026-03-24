"use client"

import { InputSearch } from "@/components/custom/input-search"
import TableData from "@/components/custom/table-data"
import { Button } from "@/components/ui/button"
import useBreakpoint from "@/hooks/use-breakpoint"
import { useGetUserListQuery } from "@/store/user/api"
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
import { PiUsers } from "react-icons/pi"
import { useSearchParams } from "react-router"
import { useUserColumns } from "./columns"
import UserFilter from "./filter"
import UserManagementForm from "./form"

export default function EmployeeManagement() {
  const [isOpenForm, setIsOpenForm] = useState(false)
  const [searchParams, setSearchParams] = useSearchParams()
  const page = searchParams.get("page") || "1"
  const search = searchParams.get("search")
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")
  const allSearchParams = Object.fromEntries(searchParams.entries())

  const { data: user, isLoading } = useGetUserListQuery({
    page: Number(page),
    limit: 10,
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
  })

  const columns = useUserColumns()
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([])
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({})
  const [rowSelection, setRowSelection] = useState({})
  const { lg, xl } = useBreakpoint()

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
    data: user?.data.data || [],
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
    if (xl) {
      setColumnVisibility({
        fullName: true,
        roleName: true,
        organizationName: true,
        email: true,
        lastLoginAt: true,
        actions: true,
      })
    } else if (lg) {
      setColumnVisibility({
        fullName: true,
        roleName: true,
        organizationName: false,
        email: true,
        lastLoginAt: false,
        actions: true,
      })
    } else {
      setColumnVisibility({
        fullName: true,
        roleName: false,
        organizationName: false,
        email: false,
        lastLoginAt: false,
        actions: true,
      })
    }
  }, [lg, xl])

  return (
    <div className="flex w-full flex-col gap-5 pt-2">
      <div className="space-y-1 pb-4">
        <p className="text-2xl font-medium">Manajemen Karyawan</p>
        <hr />
      </div>

      <div className="flex justify-between gap-4">
        <div className="flex w-full gap-4">
          <div className="flex-none">
            <UserFilter />
          </div>
          <InputSearch
            value={search || ""}
            placeholder="Cari karyawan..."
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
        </div>
        <Button
          variant="default"
          className="flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
          onClick={() => setIsOpenForm(true)}
        >
          <PiUsers className="text-xl lg:hidden xl:inline" />
          <span className="hidden lg:inline">Tambah Karyawan</span>
        </Button>
      </div>

      <TableData
        table={table}
        isLoading={isLoading}
        pagination={user?.data.pagination}
      />

      <UserManagementForm isOpen={isOpenForm} setIsOpen={setIsOpenForm} />
    </div>
  )
}
