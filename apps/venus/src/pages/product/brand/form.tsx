import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

import Modal from "@/components/custom/modal"
import ModalConfirm from "@/components/custom/modal-confirm"
import { UploadDND, type FileWithPreview } from "@/components/custom/upload"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import {
  useCreateBrandMutation,
  useUpdateBrandMutation,
} from "@/store/brand/api"
import { type BrandList } from "@/types/brand"
import { useState } from "react"
import { useForm } from "react-hook-form"

const brandSchema = z.object({
  name: z
    .string()
    .min(1, {
      message: "Nama brand wajib diisi",
    })
    .max(100, {
      message: "Nama brand maksimal 100 karakter",
    }),
  description: z.string().optional(),
  logo: z.any().optional(),
  deleteLogo: z.boolean().optional(),
})

export type BrandFormData = z.infer<typeof brandSchema>

export default function BrandManagementForm({
  isOpen,
  setIsOpen,
  data,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  data?: BrandList
}) {
  const [closeWarning, setCloseWarning] = useState(false)
  const [createBrand, { isLoading: isLoadingCreate }] = useCreateBrandMutation()
  const [updateBrand, { isLoading: isLoadingUpdate }] = useUpdateBrandMutation()

  const isLoading = isLoadingCreate || isLoadingUpdate

  const form = useForm<BrandFormData>({
    resolver: zodResolver(brandSchema),
    defaultValues: {
      name: data?.name || "",
      description: data?.description || "",
      deleteLogo: false,
    },
  })

  async function onSubmit(value: BrandFormData) {
    const formData = new FormData()

    // Add form fields to FormData
    formData.append("name", value.name)

    if (value.description) formData.append("description", value.description)
    if (value.logo instanceof File) formData.append("logo", value.logo)
    if (data && value.deleteLogo) formData.append("deleteLogo", "true")

    try {
      if (data) {
        await updateBrand({
          id: data.id,
          payload: formData,
        }).unwrap()
      } else {
        await createBrand(formData).unwrap()
      }

      setIsOpen(false)
      form.reset()
    } catch (error) {
      console.error("Gagal menyimpan brand:", error)
    }
  }

  const formDirty = form.formState.dirtyFields
  const hasFieldsDirty = Object.keys(formDirty).length > 0

  function onCancel() {
    if (hasFieldsDirty) {
      setCloseWarning(true)
    } else {
      setIsOpen(false)
      form.reset()
    }
  }

  return (
    <>
      <Modal
        title={data ? "Edit Brand" : "Tambah Brand"}
        description={
          data
            ? "Perbarui informasi detail brand termasuk nama, deskripsi, gambar, dan pengaturan lainnya. Pastikan semua data sudah benar sebelum menyimpan perubahan."
            : "Buat brand baru dengan mengisi informasi lengkap seperti nama, deskripsi, gambar, dan detail lainnya. Field yang bertanda (*) wajib diisi."
        }
        open={isOpen}
        onCancel={onCancel}
        footer={<></>}
        className="w-full sm:max-w-xl"
      >
        <div className="flex flex-col items-stretch gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              {/* Basic Information Section */}
              <div className="space-y-4">
                <h3 className="text-lg font-semibold">Informasi Dasar</h3>

                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <FormField
                    control={form.control}
                    name="name"
                    required
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Nama Brand</FormLabel>
                        <FormControl>
                          <Input
                            autoComplete="off"
                            placeholder="Masukkan nama brand"
                            disabled={isLoading}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>

                <FormField
                  control={form.control}
                  name="description"
                  required
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Deskripsi</FormLabel>
                      <FormControl>
                        <Textarea
                          className="resize-none"
                          placeholder="Masukkan deskripsi brand"
                          disabled={isLoading}
                          rows={3}
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              {/* Image Upload Section */}
              <FormField
                control={form.control}
                name="logo"
                render={({ field: { onChange, value, name, ...field } }) => (
                  <FormItem className="md:col-span-2">
                    <FormLabel>Logo</FormLabel>
                    <FormControl>
                      <UploadDND
                        {...field}
                        name={name}
                        value={value as FileWithPreview | null}
                        imageMetadata={data?.logoMetadata}
                        onDeleteImageUrl={(value) =>
                          form.setValue("deleteLogo", value)
                        }
                        onChange={onChange}
                        accept={[".jpeg", ".jpg", ".png", ".webp"]}
                        maxSize={5 * 1024 * 1024}
                        placeholder={{
                          click: "Upload logo brand",
                          drag: "Drop logo brand disini",
                        }}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              {/* Form Actions */}
              <div className="flex gap-4 border-t pt-6">
                <Button
                  type="button"
                  onClick={onCancel}
                  variant="outline"
                  size="lg"
                  disabled={isLoading}
                  className="flex-1 sm:flex-none"
                >
                  Batal
                </Button>
                <Button
                  type="submit"
                  size="lg"
                  disabled={isLoading}
                  className="flex-1 sm:flex-none"
                >
                  {isLoading ? "Menyimpan..." : data ? "Ubah" : "Simpan"}
                </Button>
              </div>
            </form>
          </Form>
        </div>
      </Modal>
      <ModalConfirm
        title="Peringatan"
        icon="warning"
        open={closeWarning}
        okVariant="destructive"
        onOk={() => {
          setCloseWarning(false)
          setIsOpen(false)
          form.reset()
        }}
        okText="Oke"
        onCancel={() => setCloseWarning(false)}
        cancelText="Batal"
        centered
        description={<p className="pt-1">{CLOSE_INPUT_FORM_WARNING}</p>}
      />
    </>
  )
}
