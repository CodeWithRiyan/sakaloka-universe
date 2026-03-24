"use client"

import { InputSearch } from "@/components/custom/input-search"
import TableData from "@/components/custom/table-data"
import { Button } from "@/components/ui/button"
import { useGetProductListQuery } from "@/store/product/api"
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
import BrandManagement from "./brand"
import CategoriesManagement from "./categories"
import { useProductColumns } from "./columns"
import ProductFilter from "./filter"
import ProductManagementForm from "./form"

export default function ProductManagement() {
  const [isOpenForm, setIsOpenForm] = useState(false)
  const [isOpenBrand, setIsOpenBrand] = useState(false)
  const [isOpenCategory, setIsOpenCategory] = useState(false)
  const [searchParams, setSearchParams] = useSearchParams()
  const page = searchParams.get("page") || "1"
  const search = searchParams.get("search")
  const sortBy = searchParams.get("sortBy")
  const sortOrder = searchParams.get("sortOrder")
  const categoryId = searchParams.get("categoryId")
  const brandId = searchParams.get("brandId")
  const isFeatured = searchParams.get("isFeatured")

  const allSearchParams = Object.fromEntries(searchParams.entries())

  const { data: product, isFetching: isLoading } = useGetProductListQuery({
    page: Number(page),
    limit: 10,
    search: search || undefined,
    sortBy: sortBy || undefined,
    sortOrder: sortOrder || undefined,
    categoryId: categoryId || undefined,
    brandId: brandId || undefined,
    isFeatured:
      isFeatured === "true" ? true : isFeatured === "false" ? false : undefined,
  })

  const columns = useProductColumns()
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
    data: product?.data?.data || [],
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
    <div className="flex w-full flex-col gap-5 pt-2">
      <div className="flex justify-end gap-2">
        <Button
          variant="secondary"
          className="flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
          onClick={() => setIsOpenBrand(true)}
        >
          Kelola Brand
        </Button>
        <Button
          variant="secondary"
          className="flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
          onClick={() => setIsOpenCategory(true)}
        >
          Kelola Kategori
        </Button>
        <Button
          className="flex flex-none items-center gap-3 rounded-lg px-5 text-sm"
          onClick={() => setIsOpenForm(true)}
        >
          <Plus strokeWidth={2.6} size={20} className="lg:hidden xl:inline" />
          <span className="hidden lg:inline">Tambah produk</span>
        </Button>
      </div>
      <div className="flex justify-between gap-4">
        <InputSearch
          value={search || ""}
          placeholder="Cari menu..."
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
          <ProductFilter />
        </div>
      </div>

      <TableData
        table={table}
        isLoading={isLoading}
        pagination={product?.data.pagination}
      />

      {isOpenForm && (
        <ProductManagementForm isOpen={isOpenForm} setIsOpen={setIsOpenForm} />
      )}
      {isOpenCategory && (
        <CategoriesManagement
          isOpen={isOpenCategory}
          setIsOpen={setIsOpenCategory}
        />
      )}
      {isOpenBrand && (
        <BrandManagement isOpen={isOpenBrand} setIsOpen={setIsOpenBrand} />
      )}
    </div>
  )
}
