import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

import { Input } from "@/components//ui/input"
import Modal from "@/components/custom/modal"
import ModalConfirm from "@/components/custom/modal-confirm"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import { useAuth } from "@/hooks/use-auth"
import {
  useCreateBranchMutation,
  useUpdateBranchMutation,
} from "@/store/branch/api"
import { type BranchList } from "@/types/branch"
import { useState } from "react"
import { useForm } from "react-hook-form"

const branchTypes = [
  { value: "COMPANY", label: "Perusahaan" },
  { value: "BRANCH", label: "Cabang" },
  { value: "DEPARTMENT", label: "Departemen" },
  { value: "WAREHOUSE", label: "Gudang" },
]

const branchSchema = z.object({
  name: z.string().min(1, {
    message: "Nama cabang toko wajib diisi",
  }),
  type: z.string().min(1, {
    message: "Tipe cabang toko wajib diisi",
  }),
  code: z.string().optional(),
  description: z.string().optional(),
  email: z
    .email({
      message: "Format email tidak valid",
    })
    .optional()
    .or(z.literal("")),
  phone: z.string().optional(),
  website: z
    .url({
      message: "Format website tidak valid",
    })
    .optional()
    .or(z.literal("")),
  address: z.string().optional(),
  city: z.string().optional(),
  state: z.string().optional(),
  postalCode: z.string().optional(),
  taxNumber: z.string().optional(),
  registrationNumber: z.string().optional(),
})

export default function BranchManagementForm({
  isOpen,
  setIsOpen,
  data,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  data?: BranchList
}) {
  const [closeWarning, setCloseWarning] = useState(false)
  const [createBranch, { isLoading: isLoadingCreate }] =
    useCreateBranchMutation()
  const [updateBranch, { isLoading: isLoadingUpdate }] =
    useUpdateBranchMutation()
  const { user } = useAuth()

  const isLoading = isLoadingCreate || isLoadingUpdate

  const form = useForm<z.infer<typeof branchSchema>>({
    resolver: zodResolver(branchSchema),
    defaultValues: {
      name: data?.name || "",
      type: data?.type || "",
      code: data?.code || "",
      description: data?.description || "",
      email: data?.email || "",
      phone: data?.phone || "",
      website: data?.website || "",
      address: data?.address || "",
      city: data?.city || "",
      state: data?.state || "",
      postalCode: data?.postalCode || "",
      taxNumber: data?.taxNumber || "",
      registrationNumber: data?.registrationNumber || "",
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

  async function onSubmit(value: z.infer<typeof branchSchema>) {
    const payload = {
      ...value,
      code: value.code?.length ? value.code : undefined,
      description: value.description?.length ? value.description : undefined,
      email: value.email?.length ? value.email : undefined,
      phone: value.phone?.length ? value.phone : undefined,
      website: value.website?.length ? value.website : undefined,
      address: value.address?.length ? value.address : undefined,
      city: value.city?.length ? value.city : undefined,
      state: value.state?.length ? value.state : undefined,
      postalCode: value.postalCode?.length ? value.postalCode : undefined,
      taxNumber: value.taxNumber?.length ? value.taxNumber : undefined,
      registrationNumber: value.registrationNumber?.length
        ? value.registrationNumber
        : undefined,
    }
    try {
      if (data) {
        await updateBranch({
          id: data.id,
          payload: {
            parentId: data?.parentId,
            country: data?.country,
            ...payload,
          },
        })
      } else {
        await createBranch({
          parentId: user?.organization.id,
          country: "Indonesia",
          ...payload,
        })
      }
    } catch (error) {
      console.error("Gagal membuat cabang toko:", error)
    } finally {
      setIsOpen(false)
      form.reset()
    }
  }

  return (
    <>
      <Modal
        title={data ? "Edit Cabang Toko" : "Tambah Cabang Toko"}
        description={
          data
            ? "Perbarui informasi detail cabang toko termasuk nama, alamat, kontak, dan pengaturan lainnya. Pastikan semua data sudah benar sebelum menyimpan perubahan."
            : "Buat cabang toko baru dengan mengisi informasi lengkap seperti nama, tipe, alamat, kontak, dan detail lainnya. Field yang bertanda (*) wajib diisi."
        }
        open={isOpen}
        onCancel={onCancel}
        footer={<></>}
        className="w-full sm:max-w-4xl"
      >
        <div className="flex flex-col items-stretch gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-5">
              {/* Basic Information */}
              <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="name"
                  required
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Nama</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                  name="type"
                  required
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Tipe</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={field.value}
                      >
                        <FormControl>
                          <SelectTrigger className="w-full">
                            <SelectValue placeholder="Pilih tipe..." />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          {branchTypes.map((branchType) => (
                            <SelectItem
                              key={branchType.value}
                              value={branchType.value}
                            >
                              {branchType.label}
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
                  name="code"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Kode</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                  name="phone"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Nomor Telepon</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                  name="email"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Email</FormLabel>
                      <FormControl>
                        <Input
                          type="email"
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                  name="website"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Website</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="https://example.com"
                          disabled={isLoading}
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>

              {/* Address Information */}
              <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="state"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Provinsi</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                  name="city"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Kota</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                name="address"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Alamat Lengkap</FormLabel>
                    <FormControl>
                      <Input
                        autoComplete="off"
                        placeholder="Ketik disini..."
                        disabled={isLoading}
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="postalCode"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Kode Pos</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                  name="taxNumber"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Nomor Pajak</FormLabel>
                      <FormControl>
                        <Input
                          autoComplete="off"
                          placeholder="Ketik disini..."
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
                        placeholder="Ketik disini..."
                        disabled={isLoading}
                        rows={3}
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="flex gap-4 pt-4">
                <Button
                  type="button"
                  onClick={onCancel}
                  variant="destructive"
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
