import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

import Modal from "@/components/custom/modal"
import ModalConfirm from "@/components/custom/modal-confirm"
import UploadDND, { type FileWithPreview } from "@/components/custom/upload"
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
  useCreateCategoriesMutation,
  useUpdateCategoriesMutation,
} from "@/store/categories/api"
import { type CategoriesList } from "@/types/categories"
import { useState } from "react"
import { useForm } from "react-hook-form"

const categoriesSchema = z.object({
  name: z
    .string()
    .min(1, {
      message: "Nama kategori wajib diisi",
    })
    .max(100, {
      message: "Nama kategori maksimal 100 karakter",
    }),
  description: z.string().optional(),
  image: z.any().optional(),
  deleteImage: z.boolean().optional(),
})

export type CategoriesFormData = z.infer<typeof categoriesSchema>

export default function CategoriesManagementForm({
  isOpen,
  setIsOpen,
  data,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  data?: CategoriesList
}) {
  const [closeWarning, setCloseWarning] = useState(false)
  const [createCategories, { isLoading: isLoadingCreate }] =
    useCreateCategoriesMutation()
  const [updateCategories, { isLoading: isLoadingUpdate }] =
    useUpdateCategoriesMutation()

  const isLoading = isLoadingCreate || isLoadingUpdate

  const form = useForm<CategoriesFormData>({
    resolver: zodResolver(categoriesSchema),
    defaultValues: {
      name: data?.name || "",
      description: data?.description || "",
      deleteImage: false,
    },
  })

  async function onSubmit(value: CategoriesFormData) {
    const formData = new FormData()

    // Add form fields to FormData
    formData.append("name", value.name)

    if (value.description) formData.append("description", value.description)
    if (value.image instanceof File) formData.append("image", value.image)
    if (data && value.deleteImage) formData.append("deleteImage", "true")

    try {
      if (data) {
        await updateCategories({
          id: data.id,
          payload: formData,
        }).unwrap()
      } else {
        await createCategories(formData).unwrap()
      }

      setIsOpen(false)
      form.reset()
    } catch (error) {
      console.error("Gagal menyimpan kategori:", error)
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
        title={data ? "Edit Kategori" : "Tambah Kategori"}
        description={
          data
            ? "Perbarui informasi detail kategori termasuk nama, deskripsi, gambar, dan pengaturan lainnya. Pastikan semua data sudah benar sebelum menyimpan perubahan."
            : "Buat kategori baru dengan mengisi informasi lengkap seperti nama, deskripsi, gambar, dan detail lainnya. Field yang bertanda (*) wajib diisi."
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
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Nama Kategori *</FormLabel>
                        <FormControl>
                          <Input
                            autoComplete="off"
                            placeholder="Masukkan nama kategori"
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
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Deskripsi</FormLabel>
                      <FormControl>
                        <Textarea
                          className="resize-none"
                          placeholder="Masukkan deskripsi kategori"
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
                name="image"
                render={({ field: { onChange, value, name, ...field } }) => (
                  <FormItem className="md:col-span-2">
                    <FormLabel>Logo</FormLabel>
                    <FormControl>
                      <UploadDND
                        {...field}
                        name={name}
                        value={value as FileWithPreview | null}
                        imageMetadata={data?.imageMetadata}
                        onDeleteImageUrl={(value) =>
                          form.setValue("deleteImage", value)
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
