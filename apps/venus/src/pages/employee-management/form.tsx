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
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import { useGetBranchListQuery } from "@/store/branch/api"
import { useGetRoleListQuery } from "@/store/role/api"
import { useCreateUserMutation, useUpdateUserMutation } from "@/store/user/api"
import { type UserList } from "@/types/user"
import { useState } from "react"
import { useForm } from "react-hook-form"

export default function UserManagementForm({
  isOpen,
  setIsOpen,
  data,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  data?: UserList
}) {
  const userSchema = z.object({
    fullName: z.string().min(1, {
      message: "Nama cabang toko wajib diisi",
    }),
    email: z.email({
      message: "Format email tidak valid",
    }),
    organizationId: z.string().min(1, {
      message: "Cabang toko wajib diisi",
    }),
    roleId: z.string().min(1, {
      message: "Role wajib diisi",
    }),
    password: data
      ? z.string().optional()
      : z.string().min(1, {
          message: "Password wajib diisi",
        }),
  })

  const [closeWarning, setCloseWarning] = useState(false)
  const [createUser, { isLoading: isLoadingCreate }] = useCreateUserMutation()
  const [updateUser, { isLoading: isLoadingUpdate }] = useUpdateUserMutation()
  const { data: branchData } = useGetBranchListQuery({})
  const { data: roleData } = useGetRoleListQuery({})

  const isLoading = isLoadingCreate || isLoadingUpdate

  const form = useForm<z.infer<typeof userSchema>>({
    resolver: zodResolver(userSchema),
    defaultValues: {
      email: data?.email || "",
      fullName: data?.fullName || "",
      organizationId: data?.organizationId || "",
      roleId: data?.roleId || "",
      password: data ? undefined : "",
    },
  })

  async function onSubmit(value: z.infer<typeof userSchema>) {
    const payload = value

    try {
      if (data) {
        await updateUser({
          id: data.id,
          payload,
        })
      } else {
        await createUser(payload)
      }
    } catch (error) {
      console.error("Gagal membuat cabang toko:", error)
    } finally {
      setIsOpen(false)
      form.reset()
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
        title={data ? "Ubah Karyawan" : "Tambah Karyawan"}
        description={
          data
            ? "Perbarui informasi detail karyawan termasuk nama, email, cabang toko, role, dan status aktif. Pastikan semua data sudah benar sebelum menyimpan perubahan."
            : "Buat karyawan baru dengan mengisi informasi lengkap seperti nama, email, cabang toko, role, dan status aktif. Field yang bertanda (*) wajib diisi."
        }
        open={isOpen}
        onCancel={onCancel}
        footer={<></>}
        className="w-full sm:max-w-4xl"
      >
        <div className="flex flex-col items-stretch gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-5">
              <FormField
                control={form.control}
                name="fullName"
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

              <div className="grid grid-cols-1 items-start gap-4 md:grid-cols-2">
                <FormField
                  control={form.control}
                  name="email"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Email</FormLabel>
                      <FormControl>
                        <Input
                          type="email"
                          autoComplete="new-email"
                          autoCorrect="off"
                          autoCapitalize="off"
                          placeholder="Ketik disini..."
                          disabled={isLoading}
                          {...field}
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {!data && (
                  <FormField
                    control={form.control}
                    name="password"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Password</FormLabel>
                        <FormControl>
                          <Input
                            {...field}
                            disabled={isLoading}
                            autoComplete="new-password"
                            placeholder="******"
                            type="password"
                          />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                )}

                <FormField
                  control={form.control}
                  name="organizationId"
                  required
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Cabang</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        defaultValue={field.value}
                      >
                        <FormControl>
                          <SelectTrigger className="w-full">
                            <SelectValue placeholder="Pilih penempatan cabang" />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          {branchData?.data.data.map((branch) => (
                            <SelectItem key={branch.id} value={branch.id}>
                              {branch.name}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {!data?.role?.name?.toLowerCase().includes("owner") && (
                  <FormField
                    control={form.control}
                    name="roleId"
                    required
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Role</FormLabel>
                        <Select
                          onValueChange={field.onChange}
                          defaultValue={field.value}
                        >
                          <FormControl>
                            <SelectTrigger className="w-full">
                              <SelectValue placeholder="Pilih role" />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            {roleData?.data.data.map((role) => (
                              <SelectItem key={role.id} value={role.id}>
                                {role.name}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                )}
              </div>

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
