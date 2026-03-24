"use client"

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
import {} from "@/types/user"
import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
// import { Switch } from "@/components/ui/switch";
import { Avatar, AvatarFallback } from "@/components/ui/avatar"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { getAuthData, updateAuthUserData } from "@/lib/auth"
import { useGetRoleListQuery } from "@/store/role/api"
import { useGetUserDetailQuery, useUpdateUserMutation } from "@/store/user/api"
import { PiUser } from "react-icons/pi"
import z from "zod"

const updateUserSchema = z.object({
  fullName: z.string().min(1, {
    message: "Nama cabang toko wajib diisi",
  }),
  email: z.email({
    message: "Format email tidak valid",
  }),
  roleId: z.string().min(1, {
    message: "Role wajib diisi",
  }),
})

export default function UserSetting() {
  const [isEdit, setIsEdit] = useState<boolean>(false)
  const auth = getAuthData()
  const { data: roleData } = useGetRoleListQuery({})
  const { data: user, isFetching: isLoading } = useGetUserDetailQuery(
    auth?.id || ""
  )
  const [handlePatch, { isLoading: isLoadingUpdate }] = useUpdateUserMutation()

  const form = useForm<z.infer<typeof updateUserSchema>>({
    resolver: zodResolver(updateUserSchema),
    defaultValues: {
      fullName: "",
      email: "",
      roleId: "",
    },
  })

  useEffect(() => {
    if (user) {
      form.reset({
        fullName: user.data?.fullName || "",
        email: user.data?.email || "",
        roleId: user.data?.roleId || "",
      })
    }
  }, [form, user])

  const onFinish = async (values: z.infer<typeof updateUserSchema>) => {
    try {
      const result = await handlePatch({
        id: auth?.id || "",
        payload: {
          ...values,
          organizationId: auth?.organization.id,
          roleId: values.roleId,
        },
      })

      if (result?.data) {
        await updateAuthUserData()
      }
    } finally {
      setIsEdit(false)
    }
  }
  return (
    <div className="flex w-full flex-col gap-5 pt-2">
      <div className="space-y-1">
        <p className="text-2xl font-medium">Profil</p>
        <hr />
      </div>
      {/* {isLoading && <ModalLoading />} */}
      <Avatar className="size-32 text-6xl text-slate-500">
        <AvatarFallback>
          <PiUser />
        </AvatarFallback>
      </Avatar>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onFinish)}
          className="w-full space-y-6 md:w-2/3"
        >
          <FormField
            control={form.control}
            name="fullName"
            required
            disabled={isLoading || isLoadingUpdate || !isEdit}
            render={({ field }) => (
              <FormItem>
                <FormLabel>Nama</FormLabel>
                <FormControl>
                  <Input
                    autoComplete="name"
                    placeholder="Masukkan nama"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
                <FormDescription>
                  Nama ini akan tertera didalam report transaksi.
                </FormDescription>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="email"
            required
            disabled={isLoading || isLoadingUpdate || !isEdit}
            render={({ field }) => (
              <FormItem>
                <FormLabel>Email</FormLabel>
                <FormControl>
                  <Input
                    autoComplete="email"
                    placeholder="Masukkan email"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
                <FormDescription>
                  Gunakan email yang masih aktif.
                </FormDescription>
              </FormItem>
            )}
          />

          {!auth?.role?.name?.toLowerCase().includes("owner") && (
            <FormField
              control={form.control}
              name="roleId"
              required
              disabled={isLoading || isLoadingUpdate || !isEdit}
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
          {/* <FormField
            control={form.control}
            name="isTwoFactorEnabled"
            showOptional
            disabled={isLoading || isLoadingUpdate}
            render={({ field }) => (
              <FormItem className="flex flex-row items-center justify-between rounded-lg border p-3 shadow-sm">
                <div className="space-y-0.5">
                  <FormLabel>Verifikasi 2 Langkah</FormLabel>
                  <FormDescription>
                    Buat akun anda menjadi lebih aman dengan verifikasi 2
                    langkah.
                  </FormDescription>
                </div>
                <FormControl>
                  <Switch
                    checked={field.value}
                    onCheckedChange={field.onChange}
                  />
                </FormControl>
              </FormItem>
            )}
          /> */}
          <div className="flex gap-4">
            {isEdit ? (
              <>
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => setIsEdit(false)}
                >
                  Batal
                </Button>
                <Button
                  type="submit"
                  disabled={isLoading || isLoadingUpdate}
                  loading={isLoading || isLoadingUpdate}
                >
                  Simpan
                </Button>
              </>
            ) : (
              <Button type="button" onClick={() => setIsEdit(true)}>
                Edit
              </Button>
            )}
          </div>
        </form>
      </Form>
    </div>
  )
}
