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
import type { OrderList } from "@/types/order"
import { HiDotsHorizontal } from "react-icons/hi"
import { LiaEyeSolid } from "react-icons/lia"

export default function ActionRow({ data }: { data: OrderList }) {
  void data
  // const dispatch = useAppDispatch()
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
          <DropdownMenuItem className="min-w-40 cursor-pointer justify-between px-3 py-2">
            <span>Lihat Detail</span>
            <LiaEyeSolid className="text-xl" />
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}
