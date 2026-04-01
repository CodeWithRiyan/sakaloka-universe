import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

import Modal from "@/components/custom/modal"
import ModalConfirm from "@/components/custom/modal-confirm"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import { useGetProductListQuery } from "@/store/product/api"
import {
  useCreateBomMutation,
  useUpdateBomMutation,
} from "@/store/bom/api"
import type { Bom } from "@/types/bom"
import { useState } from "react"
import { useForm } from "react-hook-form"

const bomSchema = z.object({
  name: z
    .string()
    .min(1, {
      message: "Nama BOM wajib diisi",
    })
    .max(100, {
      message: "Nama BOM maksimal 100 karakter",
    }),
  productId: z.string().min(1, {
    message: "Produk wajib dipilih",
  }),
  version: z.string().optional(),
  status: z.enum(["draft", "active", "archived"]).optional(),
  effectiveFrom: z.string().optional(),
  effectiveTo: z.string().optional(),
  notes: z.string().optional(),
})

type BomFormValues = z.infer<typeof bomSchema>

interface BomFormProps {
  open: boolean
  onCancel: () => void
  onClose: () => void
  editData?: Bom | null
}

export default function BomForm({
  open,
  onCancel,
  onClose,
  editData,
}: BomFormProps) {
  const [showConfirmClose, setShowConfirmClose] = useState(false)
  const isEditMode = !!editData

  const { data: products } = useGetProductListQuery({
    page: 1,
    limit: 100,
  })

  const [createBom, { isLoading: isCreating }] = useCreateBomMutation()
  const [updateBom, { isLoading: isUpdating }] = useUpdateBomMutation()

  const form = useForm<BomFormValues>({
    resolver: zodResolver(bomSchema),
    defaultValues: {
      name: editData?.name ?? "",
      productId: editData?.productId?.replace("product:", "") ?? "",
      version: editData?.version ?? "1.0.0",
      status: (editData?.status as "draft" | "active" | "archived") ?? "draft",
      effectiveFrom: editData?.effectiveFrom ?? "",
      effectiveTo: editData?.effectiveTo ?? "",
      notes: editData?.notes ?? "",
    },
  })

  const onSubmit = async (values: BomFormValues) => {
    try {
      if (isEditMode && editData) {
        await updateBom({
          id: editData.id.replace("bom:", ""),
          payload: {
            name: values.name,
            version: values.version,
            status: values.status,
            effectiveFrom: values.effectiveFrom || undefined,
            effectiveTo: values.effectiveTo || undefined,
            notes: values.notes,
          },
        }).unwrap()
      } else {
        await createBom({
          name: values.name,
          productId: values.productId,
          version: values.version,
          status: values.status,
          effectiveFrom: values.effectiveFrom || undefined,
          effectiveTo: values.effectiveTo || undefined,
          notes: values.notes,
        }).unwrap()
      }
      onClose()
      form.reset()
    } catch (error) {
      console.error("Failed to save BOM:", error)
    }
  }

  const handleCancel = () => {
    if (form.formState.isDirty) {
      setShowConfirmClose(true)
    } else {
      onCancel()
    }
  }

  const handleConfirmClose = () => {
    setShowConfirmClose(false)
    onClose()
    form.reset()
  }

  return (
    <>
      <Modal
        open={open}
        onCancel={handleCancel}
        title={isEditMode ? "Edit BOM" : "Tambah BOM"}
        description="Form untuk membuat atau mengedit Bill of Materials"
        className="max-w-lg"
      >
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Nama BOM</FormLabel>
                  <FormControl>
                    <Input placeholder="Contoh: Resep Cappuccino" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="productId"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Produk</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Pilih produk" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {products?.data.data.map((product) => (
                        <SelectItem
                          key={product.id}
                          value={product.id.replace("product:", "")}
                        >
                          {product.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="version"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Versi</FormLabel>
                  <FormControl>
                    <Input placeholder="1.0.0" {...field} />
                  </FormControl>
                  <FormDescription>
                    Nomor versi BOM, contoh: 1.0.0
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="status"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Status</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                    value={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Pilih status" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value="draft">Draft</SelectItem>
                      <SelectItem value="active">Active</SelectItem>
                      <SelectItem value="archived">Archived</SelectItem>
                    </SelectContent>
                  </Select>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="effectiveFrom"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Tanggal Efektif Dari</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="effectiveTo"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Tanggal Efektif Hasta</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="notes"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Catatan</FormLabel>
                  <FormControl>
                    <Textarea
                      placeholder="Tambahkan catatan..."
                      {...field}
                      value={field.value ?? ""}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            <div className="flex justify-end gap-2 pt-4">
              <Button
                type="button"
                variant="outline"
                onClick={handleCancel}
              >
                Batal
              </Button>
              <Button type="submit" disabled={isCreating || isUpdating}>
                {isCreating || isUpdating
                  ? "Menyimpan..."
                  : isEditMode
                  ? "Simpan Perubahan"
                  : "Buat BOM"}
              </Button>
            </div>
          </form>
        </Form>
      </Modal>

      <ModalConfirm
        open={showConfirmClose}
        onOk={handleConfirmClose}
        onCancel={() => setShowConfirmClose(false)}
        title="Peringatan"
        icon="warning"
        okVariant="destructive"
        okText="Tutup"
        cancelText="Batal"
        centered
        description={<p className="pt-1">{CLOSE_INPUT_FORM_WARNING}</p>}
      />
    </>
  )
}