"use client"

import ModalConfirm from "@/components/custom/modal-confirm"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useDeleteCategoriesMutation } from "@/store/categories/api"
import type { CategoriesList } from "@/types/categories"
import { useState } from "react"
import { HiDotsHorizontal } from "react-icons/hi"
import { LiaEditSolid, LiaTrashAltSolid } from "react-icons/lia"
import CategoriesManagementForm from "./form"

export default function ActionRow({ data }: { data: CategoriesList }) {
  const [isOpenForm, setIsOpenForm] = useState<boolean>(false)
  const [open, setOpen] = useState<boolean>(false)
  const [handleDelete, { isLoading: isLoadingDelete }] =
    useDeleteCategoriesMutation()

  const onDelete = async () => {
    await handleDelete(data.id)
    setOpen(false)
  }

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
            onClick={() => setIsOpenForm(true)}
          >
            <span>Ubah</span>
            <LiaEditSolid className="text-xl" />
          </DropdownMenuItem>
          <DropdownMenuItem
            className="min-w-40 cursor-pointer justify-between px-3 py-2"
            onClick={() => {
              setOpen(true)
            }}
          >
            <span>Hapus</span>
            <LiaTrashAltSolid className="text-xl" />
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <ModalConfirm
        open={open}
        onCancel={() => setOpen(false)}
        onOk={onDelete}
        okVariant={"destructive"}
        okText="Hapus"
        centered
        loading={isLoadingDelete}
        description={
          <>
            <span>Apakah kamu ingin menghapus kategori </span>
            <strong>{data.name}</strong>
          </>
        }
      />
      <CategoriesManagementForm
        isOpen={isOpenForm}
        setIsOpen={setIsOpenForm}
        data={data}
      />
    </>
  )
}
