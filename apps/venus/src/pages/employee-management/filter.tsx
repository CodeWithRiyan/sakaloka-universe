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
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { useGetBranchListQuery } from "@/store/branch/api"
import { useGetRoleListQuery } from "@/store/role/api"
import { zodResolver } from "@hookform/resolvers/zod"
import { Filter } from "lucide-react"
import queryString from "query-string"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import { useLocation, useNavigate } from "react-router"
import z from "zod"

const filterUserSchema = z.object({
  roleId: z.string().optional(),
  branchId: z.string().optional(),
})

export default function UserFilter() {
  const [open, setOpen] = useState(false)
  const { data: role, isFetching: isLoadingRole } = useGetRoleListQuery({
    limit: 100,
    page: 1,
  })
  const { data: branch, isFetching: isLoadingBrach } = useGetBranchListQuery({
    limit: 100,
    page: 1,
  })
  const isLoading = isLoadingRole || isLoadingBrach

  const location = useLocation()
  const navigate = useNavigate()
  const searchParams = queryString.parse(location.search)
  const roleId = searchParams.roleId as string
  const branchId = searchParams.branchId as string

  const filterLengthActive = [roleId, branchId].filter((item) =>
    Boolean(item)
  ).length

  const form = useForm<z.infer<typeof filterUserSchema>>({
    resolver: zodResolver(filterUserSchema),
    defaultValues: {
      roleId: "",
      branchId: "",
    },
  })

  useEffect(() => {
    form.setValue("roleId", roleId || "")
    form.setValue("branchId", branchId || "")
  }, [roleId, branchId, open, form])

  function onReset() {
    navigate(
      `${location.pathname}?${queryString.stringify({
        ...searchParams,
        roleId: undefined,
        branchId: undefined,
      })}`,
      { replace: true }
    )
    setTimeout(() => {
      form.reset()
      setOpen(false)
    }, 1000)
  }

  function onSubmit(value: z.infer<typeof filterUserSchema>) {
    navigate(
      `${location.pathname}?${queryString.stringify({
        ...searchParams,
        roleId: value.roleId || undefined,
        branchId: value.branchId || undefined,
      })}`,
      { replace: true }
    )
    setTimeout(() => {
      setOpen(false)
    }, 1000)
  }

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="outline" className="relative md:w-32">
          {!!filterLengthActive && (
            <div className="bg-primary text-primary-foreground absolute top-0 left-0 z-50 flex h-5 min-w-5 -translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full p-2 text-xs shadow-md duration-500">
              {filterLengthActive}
            </div>
          )}
          <Filter className="size-4" />
          <span className="hidden sm:inline">Filter</span>
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="grid gap-4">
          <div className="space-y-2">
            <h4 className="leading-none font-medium">Filter</h4>
            <p className="text-muted-foreground text-sm">Fliter data produk</p>
          </div>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
              <div className="grid gap-2">
                <FormField
                  control={form.control}
                  name="roleId"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Role</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        value={field.value}
                        disabled={isLoading}
                      >
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue placeholder="Pilih role" />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          {role?.data?.data?.map((r) => (
                            <SelectItem key={r.id} value={r.id}>
                              {r.name}
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
                  name="branchId"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Cabang</FormLabel>
                      <Select
                        onValueChange={field.onChange}
                        value={field.value}
                        disabled={isLoading}
                      >
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue placeholder="Pilih cabang" />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          {branch?.data?.data?.map((b) => (
                            <SelectItem key={b.id} value={b.id}>
                              {b.name}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <div className="flex gap-4 border-t pt-6">
                <Button
                  type="button"
                  variant="outline"
                  size="lg"
                  disabled={isLoading}
                  loading={isLoading}
                  className="flex-1 sm:flex-none"
                  onClick={onReset}
                >
                  Reset Filter
                </Button>
                <Button
                  type="submit"
                  size="lg"
                  disabled={isLoading}
                  loading={isLoading}
                  className="flex-1 sm:flex-none"
                >
                  Terapkan
                </Button>
              </div>
            </form>
          </Form>
        </div>
      </PopoverContent>
    </Popover>
  )
}
