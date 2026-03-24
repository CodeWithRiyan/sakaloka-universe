"use client"

import { InputSearch } from "@/components/custom/input-search"
import Modal from "@/components/custom/modal"
import TableData from "@/components/custom/table-data"
import { Button } from "@/components/ui/button"
import { useGetBrandListQuery } from "@/store/brand/api"
import {
  type ColumnFiltersState,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { Plus } from "lucide-react"
import { useState } from "react"
import { useBrandColumns } from "./columns"
import BrandManagementForm from "./form"

export default function BrandManagement({
  isOpen,
  setIsOpen,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
}) {
  const [isOpenForm, setIsOpenForm] = useState(false)
  const [search, setSearch] = useState<string>("")
  const [sortOrder, setSortOrder] = useState<string | undefined>()
  const [sortBy, setSortBy] = useState<string | undefined>()

  const { data: brand, isLoading } = useGetBrandListQuery({
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
  })

  const columns = useBrandColumns({
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
    data: brand?.data?.data || [],
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
    setIsOpen(false)
  }

  return (
    <Modal
      title="Kelola Brand"
      description="Mengelola brand produk"
      open={isOpen}
      onCancel={handleCancel}
      footer={<></>}
      className="w-full sm:max-w-xl"
    >
      <div className="flex w-full flex-col gap-5 pt-2">
        <div className="flex justify-between gap-10">
          <div className="flex w-full items-center gap-3">
            <InputSearch
              value={search || ""}
              placeholder="Cari brand..."
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
            <Plus strokeWidth={2.6} size={20} className="lg:hidden xl:inline" />
            <span className="hidden lg:inline">Tambah Brand</span>
          </Button>
        </div>

        <div className="rounded-md border">
          <TableData table={table} isLoading={isLoading} />
        </div>

        <BrandManagementForm isOpen={isOpenForm} setIsOpen={setIsOpenForm} />
      </div>
    </Modal>
  )
}
