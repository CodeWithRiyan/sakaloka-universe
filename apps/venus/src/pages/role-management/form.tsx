import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

import Modal from "@/components/custom/modal"
import ModalConfirm from "@/components/custom/modal-confirm"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
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
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import { cn } from "@/lib/utils"
import {
  useCreateRoleMutation,
  useGetPermissionQuery,
  useUpdateRoleMutation,
} from "@/store/role/api"
import { type RoleList } from "@/types/role"
import { useState } from "react"
import { useForm } from "react-hook-form"

const roleSchema = z.object({
  name: z.string().min(1, {
    message: "Nama role wajib diisi",
  }),
  permissions: z.record(z.string(), z.array(z.string())),
})

export default function RoleManagementForm({
  isOpen,
  setIsOpen,
  data,
}: {
  isOpen: boolean
  setIsOpen: (isOpen: boolean) => void
  data?: RoleList
}) {
  const [closeWarning, setCloseWarning] = useState(false)
  const [createRole, { isLoading: isLoadingCreate }] = useCreateRoleMutation()
  const [updateRole, { isLoading: isLoadingUpdate }] = useUpdateRoleMutation()
  const { data: permission, isLoading: isLoadingPermission } =
    useGetPermissionQuery()

  const isLoading = isLoadingCreate || isLoadingUpdate || isLoadingPermission

  const form = useForm<z.infer<typeof roleSchema>>({
    resolver: zodResolver(roleSchema),
    defaultValues: {
      name: data?.name || "",
      permissions: data?.permissions || {},
    },
  })

  // Helper function to check if a permission is selected
  const isPermissionSelected = (module: string, permission: string) => {
    const selectedPermissions = form.getValues(`permissions.${module}`) || []
    return selectedPermissions.includes(permission)
  }

  // Helper function to toggle permission
  const togglePermission = (module: string, permission: string) => {
    const currentPermissions = form.getValues(`permissions.${module}`) || []
    const newPermissions = currentPermissions.includes(permission)
      ? currentPermissions.filter((p) => p !== permission)
      : [...currentPermissions, permission]

    form.setValue(`permissions.${module}`, newPermissions, {
      shouldValidate: true,
      shouldDirty: true,
    })
  }

  async function onSubmit(value: z.infer<typeof roleSchema>) {
    const payload = value
    try {
      if (data) {
        await updateRole({
          id: data.id,
          payload,
        }).unwrap()
      } else {
        await createRole(payload).unwrap()
      }
    } catch (error) {
      console.error("Gagal menyimpan role:", error)
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
        title={data ? "Edit Role" : "Tambah Role"}
        description={
          data
            ? "Perbarui informasi role termasuk nama dan izin akses. Pastikan semua pengaturan sudah benar sebelum menyimpan perubahan."
            : "Buat role baru dengan mengisi nama dan menentukan izin akses untuk setiap modul. Field yang bertanda (*) wajib diisi."
        }
        open={isOpen}
        onCancel={onCancel}
        footer={<></>}
        className="w-full rounded-lg sm:max-w-6xl"
      >
        <div className="flex flex-col items-stretch gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              {/* Basic Information */}
              <Card className="rounded-lg shadow-xs">
                <CardHeader>
                  <CardTitle>Informasi Dasar</CardTitle>
                  <CardDescription>
                    Isi informasi dasar untuk role yang akan dibuat
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <FormField
                    control={form.control}
                    name="name"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Nama Role *</FormLabel>
                        <FormControl>
                          <Input
                            autoComplete="off"
                            placeholder="Contoh: Admin, Manager, Staff..."
                            disabled={isLoading}
                            {...field}
                          />
                        </FormControl>
                        <FormDescription>
                          Nama role yang mudah diidentifikasi dan menggambarkan
                          fungsinya
                        </FormDescription>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* Permissions */}
              <Card className="rounded-lg shadow-xs">
                <CardHeader>
                  <CardTitle asChild>
                    <div className="flex items-center justify-between gap-2">
                      <span>Pengaturan Izin Akses</span>
                      <div className="flex gap-2">
                        <Button
                          type="button"
                          size="sm"
                          variant="outline"
                          onClick={() => {
                            form.setValue(
                              "permissions",
                              {},
                              {
                                shouldValidate: true,
                                shouldDirty: true,
                              }
                            )
                          }}
                        >
                          Hapus Semua
                        </Button>
                        <Button
                          type="button"
                          size="sm"
                          variant="outline"
                          onClick={() => {
                            const selectAllPermissions = permission?.data
                              ? permission.data.reduce(
                                  (acc, module) => {
                                    acc[module.key] = module.permissions.map(
                                      (p) => p.key
                                    )
                                    return acc
                                  },
                                  {} as { [key: string]: string[] }
                                )
                              : {}
                            form.setValue("permissions", selectAllPermissions, {
                              shouldValidate: true,
                              shouldDirty: true,
                            })
                          }}
                        >
                          Pilih Semua
                        </Button>
                      </div>
                    </div>
                  </CardTitle>
                  <CardDescription>
                    Tentukan izin akses untuk setiap modul sistem. Pilih minimal
                    satu izin untuk setiap modul yang diperlukan.
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-6">
                  {permission?.data?.map((module) => {
                    const selectedPermissions =
                      form
                        .getValues(`permissions.${module.key}`)
                        ?.map((p) => p) || []
                    const hasAnyPermission = selectedPermissions.length > 0
                    const hasAllPermissions = module.permissions.every((p) =>
                      selectedPermissions.includes(p.key)
                    )

                    return (
                      <Card
                        key={module.key}
                        className={cn(
                          "rounded-lg border shadow-xs",
                          hasAnyPermission
                            ? "border-blue-200 bg-blue-50/30"
                            : "border-gray-200"
                        )}
                      >
                        <CardHeader className="pb-5">
                          <div className="flex items-center justify-between">
                            <div className="flex flex-1 items-start space-x-3">
                              <Checkbox
                                id={`${module.key}`}
                                checked={hasAllPermissions}
                                onCheckedChange={(checked) => {
                                  const moduleData = permission?.data?.find(
                                    (m) => m.key === module.key
                                  )
                                  const allPermissions =
                                    moduleData?.permissions?.map(
                                      (p) => p.key
                                    ) || []
                                  form.setValue(
                                    `permissions.${module.key}`,
                                    checked ? allPermissions : [],
                                    { shouldValidate: true, shouldDirty: true }
                                  )
                                }}
                                disabled={isLoading}
                                className="mt-0.5"
                              />
                              <div className="grid flex-1 leading-none">
                                <label
                                  htmlFor={`${module.key}`}
                                  className="cursor-pointer text-start text-lg leading-tight font-semibold text-gray-900"
                                >
                                  {module.label}
                                </label>
                                <p className="text-muted-foreground text-sm leading-relaxed">
                                  {module.description}
                                </p>
                              </div>
                            </div>
                            <div className="text-muted-foreground ml-3 rounded-md bg-gray-100 px-2 py-1 text-xs font-medium">
                              {selectedPermissions.length} /{" "}
                              {module.permissions.length} izin
                            </div>
                          </div>
                        </CardHeader>
                        <CardContent className="pt-0">
                          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
                            {module.permissions.map((permission) => (
                              <div
                                key={permission.key}
                                className="flex items-start space-x-2"
                              >
                                <Checkbox
                                  id={`${module.key}-${permission.key}`}
                                  checked={isPermissionSelected(
                                    module.key,
                                    permission.key
                                  )}
                                  onCheckedChange={() =>
                                    togglePermission(module.key, permission.key)
                                  }
                                  disabled={isLoading}
                                />
                                <div className="grid gap-1.5 leading-none">
                                  <label
                                    htmlFor={`${module.key}-${permission.key}`}
                                    className="cursor-pointer text-sm leading-none font-medium peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                                  >
                                    {permission.label}
                                  </label>
                                </div>
                              </div>
                            ))}
                          </div>
                        </CardContent>
                      </Card>
                    )
                  })}

                  <FormField
                    control={form.control}
                    name="permissions"
                    render={() => (
                      <FormItem>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                </CardContent>
              </Card>

              {/* Action Buttons */}
              <div className="flex flex-col gap-3 pt-4 sm:flex-row">
                <Button
                  type="button"
                  onClick={onCancel}
                  variant="outline"
                  size="lg"
                  disabled={isLoading}
                  className="flex-none"
                >
                  Batal
                </Button>
                <Button
                  type="submit"
                  size="lg"
                  disabled={
                    isLoading ||
                    !Object.keys(form.getValues("permissions"))?.length
                  }
                  className="flex-none"
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
        okText="Ya"
        onCancel={() => setCloseWarning(false)}
        cancelText="Batal"
        centered
        description={<p className="pt-1">{CLOSE_INPUT_FORM_WARNING}</p>}
      />
    </>
  )
}
