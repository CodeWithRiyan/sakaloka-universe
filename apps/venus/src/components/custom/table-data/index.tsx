/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { cn } from "@/lib/utils"
import type { IPageMeta } from "@/types/pagination"
import { flexRender, type Table as ITable } from "@tanstack/react-table"
import { useSearchParams } from "react-router"
import { ScaleLoader } from "react-spinners"
import BasicPagination from "../basic-pagination"

export default function TableData<T>({
  table,
  isLoading,
  pagination,
}: {
  table: ITable<T>
  isLoading?: boolean
  pagination?: IPageMeta
}) {
  const [searchParams, setSearchParams] = useSearchParams()
  const allSearchParams = Object.fromEntries(searchParams)

  const onPageChange = (page: number) => {
    setSearchParams({
      ...allSearchParams,
      page: page.toString(),
    })
  }

  return (
    <div className="flex flex-col items-end gap-2">
      <div className="w-full rounded-lg border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead
                      key={header.id}
                      style={{ width: `${header.getSize()}px` }}
                    >
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </TableHead>
                  )
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {isLoading && (
              <TableRow>
                <TableCell
                  colSpan={table.getAllColumns().length}
                  className="relative h-60 text-center"
                >
                  <div className="text-primary absolute inset-0 flex items-center justify-center">
                    <ScaleLoader color="currentColor" />
                  </div>
                </TableCell>
              </TableRow>
            )}
            {!!table.getRowModel().rows?.length &&
              !isLoading &&
              table.getRowModel().rows.map((row, rowIndex) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                  className={cn(!!row.subRows.length && "bg-gray-50/50")}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell
                      key={cell.id}
                      className={cn(
                        (cell.column.columnDef.meta as any)?.className
                      )}
                      style={(cell.column.columnDef.meta as any)?.style?.({
                        row,
                        index: rowIndex,
                      })}
                      colSpan={(cell.column.columnDef.meta as any)?.colSpan?.({
                        row,
                        index: rowIndex,
                      })}
                    >
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            {!table.getRowModel().rows?.length && !isLoading && (
              <TableRow>
                <TableCell
                  colSpan={table.getAllColumns().length}
                  className="h-24 text-center"
                >
                  No results.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      {pagination && (
        <BasicPagination
          totalPages={pagination?.pages || 1}
          initialPage={pagination?.page}
          onPageChange={onPageChange}
        />
      )}
    </div>
  )
}
