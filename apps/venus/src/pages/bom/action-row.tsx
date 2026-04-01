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
import {
  useActivateBomMutation,
  useDeleteBomMutation,
} from "@/store/bom/api"
import type { Bom } from "@/types/bom"
import { useState } from "react"
import { HiDotsHorizontal } from "react-icons/hi"
import { LiaEditSolid, LiaTrashAltSolid } from "react-icons/lia"
import { MdOutlineCheckCircle } from "react-icons/md"
import BomForm from "./form"

interface BomActionRowProps {
  data: Bom
  onEdit?: (bom: Bom) => void
}

export default function BomActionRow({ data, onEdit }: BomActionRowProps) {
  const [isOpenForm, setIsOpenForm] = useState<boolean>(false)
  const [openDelete, setOpenDelete] = useState<boolean>(false)
  const [openActivate, setOpenActivate] = useState<boolean>(false)

  const [handleDelete, { isLoading: isLoadingDelete }] = useDeleteBomMutation()
  const [handleActivate, { isLoading: isLoadingActivate }] = useActivateBomMutation()

  const onDelete = async () => {
    await handleDelete(data.id.replace("bom:", ""))
    setOpenDelete(false)
  }

  const onActivate = async () => {
    await handleActivate(data.id.replace("bom:", ""))
    setOpenActivate(false)
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
            onClick={() => {
              setIsOpenForm(true)
              onEdit?.(data)
            }}
          >
            <span>Ubah</span>
            <LiaEditSolid className="text-xl" />
          </DropdownMenuItem>
          {data.status !== "active" && (
            <DropdownMenuItem
              className="min-w-40 cursor-pointer justify-between px-3 py-2"
              onClick={() => setOpenActivate(true)}
            >
              <span>Aktifkan</span>
              <MdOutlineCheckCircle className="text-xl text-green-500" />
            </DropdownMenuItem>
          )}
          <DropdownMenuItem
            className="min-w-40 cursor-pointer justify-between px-3 py-2"
            onClick={() => setOpenDelete(true)}
          >
            <span>Hapus</span>
            <LiaTrashAltSolid className="text-xl" />
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <ModalConfirm
        open={openDelete}
        onCancel={() => setOpenDelete(false)}
        onOk={onDelete}
        okVariant={"destructive"}
        okText="Hapus"
        centered
        loading={isLoadingDelete}
        description={
          <>
            <span>Apakah kamu ingin menghapus BOM </span>
            <strong>{data.name}</strong>
          </>
        }
      />

      <ModalConfirm
        open={openActivate}
        onCancel={() => setOpenActivate(false)}
        onOk={onActivate}
        okText="Aktifkan"
        centered
        loading={isLoadingActivate}
        description={
          <>
            <span>Aktifkan BOM </span>
            <strong>{data.name}</strong>
            <span>? BOM ini akan menjadi versi aktif.</span>
          </>
        }
      />

      <BomForm
        open={isOpenForm}
        onCancel={() => setIsOpenForm(false)}
        onClose={() => setIsOpenForm(false)}
        editData={data}
      />
    </>
  )
}