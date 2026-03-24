import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

import Modal from "@/components/custom/modal"
import ModalConfirm from "@/components/custom/modal-confirm"
import UploadDND, { type FileWithPreview } from "@/components/custom/upload"
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
import InputCurrency from "@/components/ui/input-currency"
import InputNumber from "@/components/ui/input-number"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Switch } from "@/components/ui/switch"
import { Textarea } from "@/components/ui/textarea"
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import { useGetBrandListQuery } from "@/store/brand/api"
import { useGetCategoriesListQuery } from "@/store/categories/api"
import {
  useCreateProductMutation,
  useUpdateProductMutation,
} from "@/store/product/api"
import { type ProductList } from "@/types/product"
import { useState } from "react"
import { useForm } from "react-hook-form"

const productSchema = z
  .object({
    name: z
      .string()
      .min(1, {
        message: "Nama Produk wajib diisi",
      })
      .max(100, {
        message: "Nama Produk maksimal 100 karakter",
      }),
    description: z.string().optional(),
    sku: z.string().min(1, {
      message: "SKU wajib diisi",
    }),
    barcode: z.string().optional(),
    basePrice: z.number().min(1, {
      message: "Harga wajib diisi",
    }),
    costPrice: z.number().optional(),
    categoryId: z.string().min(1, {
      message: "Kategori wajib diisi",
    }),
    brandId: z.string().optional(),
    weight: z.number().optional(),
    isFeatured: z.boolean().optional(),
    trackInventory: z.boolean().optional(),
    initialStock: z.number().optional(),
    minStockLevel: z.number().optional(),
    image: z.any().optional(),
    deleteImage: z.boolean().optional(),
  })
  .refine(
    (data) => {
      // Only validate if costPrice is provided and greater than 0
      if (data.costPrice && data.costPrice > 0) {
        return data.basePrice > data.costPrice
      }
      return true // Skip validation if costPrice is not provided
    },
    {
      message: "Harga Jual harus lebih tinggi dari Harga Beli",
      path: ["basePrice"],
    }
  )

export type ProductFormData = z.infer<typeof productSchema>

export default function ProductManagementForm({
  isOpen,
  setIsOpen,
  data,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  data?: ProductList
}) {
  const [closeWarning, setCloseWarning] = useState(false)
  const [createProduct, { isLoading: isLoadingCreate }] =
    useCreateProductMutation()
  const [updateProduct, { isLoading: isLoadingUpdate }] =
    useUpdateProductMutation()
  const { data: categories } = useGetCategoriesListQuery({
    limit: 100,
    page: 1,
  })
  const { data: brand } = useGetBrandListQuery({
    limit: 100,
    page: 1,
  })

  const isLoading = isLoadingCreate || isLoadingUpdate

  const form = useForm<ProductFormData>({
    resolver: zodResolver(productSchema),
    defaultValues: {
      name: data?.name || "",
      description: data?.description || "",
      sku: data?.sku || "",
      barcode: data?.barcode || "",
      basePrice: data?.basePrice ? Number(data?.basePrice) : 0,
      costPrice: data?.costPrice ? Number(data?.costPrice) : 0,
      categoryId: data?.categoryId || "",
      brandId: data?.brandId || "",
      weight: data?.weight ? Number(data?.weight) : 0,
      isFeatured: data?.isFeatured || false,
      trackInventory: data?.trackInventory || true,
      deleteImage: false,
      initialStock: 0,
      minStockLevel: data?.minStockLevel ? Number(data?.minStockLevel) : 0,
    },
  })

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

  async function onSubmit(value: ProductFormData) {
    const formData = new FormData()

    // Add form fields to FormData
    formData.append("name", value.name)
    formData.append("sku", value.sku)
    formData.append("basePrice", value.basePrice.toString())

    if (data && value.deleteImage) formData.append("deleteImage", "true")
    if (value.description) formData.append("description", value.description)
    if (value.barcode) formData.append("barcode", value.barcode)
    if (value.costPrice)
      formData.append("costPrice", value.costPrice.toString())
    if (value.categoryId) formData.append("categoryId", value.categoryId)
    if (value.brandId) formData.append("brandId", value.brandId)
    if (value.weight) formData.append("weight", value.weight.toString())
    if (value.isFeatured)
      formData.append("isFeatured", value.isFeatured.toString())
    if (value.trackInventory)
      formData.append("trackInventory", value.trackInventory.toString())
    if (value.initialStock)
      formData.append("initialStock", value.initialStock.toString())
    if (value.minStockLevel)
      formData.append("minStockLevel", value.minStockLevel.toString())
    if (value.image instanceof File) formData.append("image", value.image)

    try {
      if (data) {
        await updateProduct({
          id: data.id,
          payload: formData,
        }).unwrap()
      } else {
        await createProduct(formData).unwrap()
      }

      setIsOpen(false)
      form.reset()
    } catch (error) {
      console.error("Gagal menyimpan Produk:", error)
    }
  }

  return (
    <>
      <Modal
        title={data ? "Edit Produk" : "Tambah Produk"}
        description={
          data
            ? "Perbarui informasi detail produk termasuk nama, deskripsi, gambar, dan pengaturan lainnya. Pastikan semua data sudah benar sebelum menyimpan perubahan."
            : "Buat produk baru dengan mengisi informasi lengkap seperti nama, deskripsi, gambar, dan detail lainnya. Field yang bertanda (*) wajib diisi."
        }
        open={isOpen}
        onCancel={onCancel}
        footer={<></>}
        className="w-full sm:max-w-4xl"
      >
        <div className="flex flex-col items-stretch gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
              {/* Basic Information Section */}
              <div className="space-y-6">
                <div className="flex items-center gap-2">
                  <h3 className="text-muted-foreground flex-none text-xs font-semibold">
                    Informasi Dasar
                  </h3>
                  <div className="h-[1px] w-full bg-gray-300" />
                </div>

                <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                  <FormField
                    control={form.control}
                    name="image"
                    render={({
                      field: { onChange, value, name, ...field },
                    }) => (
                      <FormItem className="md:col-span-2">
                        <FormLabel>Foto Produk</FormLabel>
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
                              click: "Upload product image",
                              drag: "Drop your product image here",
                            }}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="name"
                    required
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Nama Produk</FormLabel>
                        <FormControl>
                          <Input
                            autoComplete="off"
                            placeholder="Masukkan nama produk"
                            disabled={isLoading}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="sku"
                    required
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>SKU</FormLabel>
                        <FormControl>
                          <Input
                            autoComplete="off"
                            placeholder="Masukkan SKU produk"
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
                          placeholder="Masukkan deskripsi produk"
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

              {/* Pricing Section */}
              <div className="space-y-6">
                <div className="flex items-center gap-2">
                  <h3 className="text-muted-foreground flex-none text-xs font-semibold">
                    Harga
                  </h3>
                  <div className="h-[1px] w-full bg-gray-300" />
                </div>

                <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                  <FormField
                    control={form.control}
                    name="costPrice"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Harga Beli</FormLabel>
                        <FormControl>
                          <InputCurrency
                            {...field}
                            placeholder="Masukkan harga beli"
                            disabled={isLoading}
                            onChange={(e) =>
                              field.onChange(Number(e.target.value))
                            }
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="basePrice"
                    required
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Harga Jual</FormLabel>
                        <FormControl>
                          <InputCurrency
                            {...field}
                            placeholder="Masukkan harga jual"
                            disabled={isLoading}
                            onChange={(e) =>
                              field.onChange(Number(e.target.value))
                            }
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </div>
              </div>

              {/* Product Details Section */}
              <div className="space-y-6">
                <div className="flex items-center gap-2">
                  <h3 className="text-muted-foreground flex-none text-xs font-semibold">
                    Detail Produk
                  </h3>
                  <div className="h-[1px] w-full bg-gray-300" />
                </div>

                <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                  <FormField
                    control={form.control}
                    name="categoryId"
                    required
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Kategori</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          value={field.value}
                          disabled={isLoading}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder="Pilih kategori" />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            {categories?.data?.data?.map((category) => (
                              <SelectItem key={category.id} value={category.id}>
                                {category.name}
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
                    name="brandId"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Brand</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          value={field.value}
                          disabled={isLoading}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder="Pilih brand" />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            {brand?.data?.data?.map((brand) => (
                              <SelectItem key={brand.id} value={brand.id}>
                                {brand.name}
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
                    name="barcode"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Barcode</FormLabel>
                        <FormControl>
                          <Input
                            autoComplete="off"
                            placeholder="Masukkan barcode"
                            disabled={isLoading}
                            {...field}
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="weight"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Berat (gram)</FormLabel>
                        <FormControl>
                          <InputNumber
                            {...field}
                            placeholder="Masukkan berat dalam gram"
                            disabled={isLoading}
                            onChange={(e) =>
                              field.onChange(Number(e.target.value))
                            }
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />

                  {!data && (
                    <FormField
                      control={form.control}
                      name="initialStock"
                      render={({ field }) => (
                        <FormItem>
                          <FormLabel>Stok Awal</FormLabel>
                          <FormControl>
                            <InputNumber
                              {...field}
                              placeholder="Masukkan stok awal"
                              disabled={isLoading}
                              onChange={(e) =>
                                field.onChange(Number(e.target.value))
                              }
                            />
                          </FormControl>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                  )}

                  <FormField
                    control={form.control}
                    name="minStockLevel"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Stok Minimal</FormLabel>
                        <FormControl>
                          <InputNumber
                            {...field}
                            placeholder="Masukkan stok minimal"
                            disabled={isLoading}
                            onChange={(e) =>
                              field.onChange(Number(e.target.value))
                            }
                          />
                        </FormControl>
                        <FormMessage />
                        <FormDescription>
                          Jika stok produk kurang dari stok minimal, pengguna
                          akan mendapatkan notifikasi.
                        </FormDescription>
                      </FormItem>
                    )}
                  />
                </div>
              </div>

              {/* Settings Section */}
              <div className="space-y-6">
                <div className="flex items-center gap-2">
                  <h3 className="text-muted-foreground flex-none text-xs font-semibold">
                    Pengaturan
                  </h3>
                  <div className="h-[1px] w-full bg-gray-300" />
                </div>

                <div className="space-y-4">
                  <FormField
                    control={form.control}
                    name="isFeatured"
                    render={({ field }) => (
                      <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                        <div className="space-y-0.5">
                          <FormLabel className="text-base">
                            Produk Unggulan
                          </FormLabel>
                          <p className="text-muted-foreground text-sm">
                            Tandai produk ini sebagai produk unggulan
                          </p>
                        </div>
                        <FormControl>
                          <Switch
                            checked={field.value}
                            onCheckedChange={field.onChange}
                            disabled={isLoading}
                          />
                        </FormControl>
                      </FormItem>
                    )}
                  />

                  <FormField
                    control={form.control}
                    name="trackInventory"
                    render={({ field }) => (
                      <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                        <div className="space-y-0.5">
                          <FormLabel className="text-base">
                            Lacak Inventori
                          </FormLabel>
                          <p className="text-muted-foreground text-sm">
                            Aktifkan pelacakan stok untuk produk ini
                          </p>
                        </div>
                        <FormControl>
                          <Switch
                            checked={field.value}
                            onCheckedChange={field.onChange}
                            disabled={isLoading}
                          />
                        </FormControl>
                      </FormItem>
                    )}
                  />
                </div>
              </div>

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
