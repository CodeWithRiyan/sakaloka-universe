"use client"

import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useAppDispatch } from "@/store/hooks"
import {
  setDataStock,
  setOpenStockForm,
  setOpenStockTransactionDetail,
} from "@/store/stock/action"
import { HiDotsHorizontal } from "react-icons/hi"
import { LiaEditSolid, LiaEyeSolid } from "react-icons/lia"
import type { StockData } from "."

export default function ActionRow({ data }: { data: StockData }) {
  const dispatch = useAppDispatch()
  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" className="h-8 w-8 p-0">
            <span className="sr-only">Open menu</span>
            <HiDotsHorizontal className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel className="text-center">Aksi</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            className="min-w-40 cursor-pointer justify-between px-3 py-2"
            onClick={() => {
              dispatch(
                setDataStock({
                  productId: data.productId,
                })
              )
              dispatch(setOpenStockForm(true))
            }}
          >
            <span>Atur Stok</span>
            <LiaEditSolid className="text-xl" />
          </DropdownMenuItem>
          <DropdownMenuItem
            className="min-w-40 cursor-pointer justify-between px-3 py-2"
            onClick={() => {
              dispatch(setOpenStockTransactionDetail(true))
              dispatch(
                setDataStock({
                  id: data.id,
                  productId: data.productId,
                })
              )
            }}
          >
            <span>Lihat Detail</span>
            <LiaEyeSolid className="text-xl" />
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}
