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
import { Switch } from "@/components/ui/switch"
import { useGetBrandListQuery } from "@/store/brand/api"
import { useGetCategoriesListQuery } from "@/store/categories/api"
import { useGetProductListQuery } from "@/store/product/api"
import { zodResolver } from "@hookform/resolvers/zod"
import { Filter } from "lucide-react"
import queryString from "query-string"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import { useLocation, useNavigate } from "react-router"
import z from "zod"

const filterProductSchema = z.object({
  categoryId: z.string().optional(),
  brandId: z.string().optional(),
  isFeatured: z.boolean().optional(),
})

export default function ProductFilter() {
  const [open, setOpen] = useState(false)
  const { isFetching: isLoadingProduct } = useGetProductListQuery(
    {},
    {
      selectFromResult: ({ isFetching }) => ({ isFetching }),
    }
  )
  const { data: categories, isFetching: isLoadingCategories } =
    useGetCategoriesListQuery({
      limit: 100,
      page: 1,
    })
  const { data: brand, isFetching: isLoadingBrand } = useGetBrandListQuery({
    limit: 100,
    page: 1,
  })

  const isLoading = isLoadingCategories || isLoadingBrand || isLoadingProduct
  const location = useLocation()
  const navigate = useNavigate()
  const searchParams = queryString.parse(location.search)
  const categoryId = searchParams.categoryId as string
  const brandId = searchParams.brandId as string
  const isFeatured = searchParams.isFeatured === "true"

  const filterLengthActive = [categoryId, brandId, isFeatured].filter((item) =>
    Boolean(item)
  ).length

  const form = useForm<z.infer<typeof filterProductSchema>>({
    resolver: zodResolver(filterProductSchema),
    defaultValues: {
      categoryId: "",
      brandId: "",
      isFeatured: false,
    },
  })

  useEffect(() => {
    form.setValue("categoryId", categoryId || "")
    form.setValue("brandId", brandId || "")
    form.setValue("isFeatured", isFeatured || false)
  }, [categoryId, brandId, isFeatured, open, form])

  function onReset() {
    navigate(
      `${location.pathname}?${queryString.stringify({
        ...searchParams,
        categoryId: undefined,
        brandId: undefined,
        isFeatured: undefined,
      })}`
    )
    setTimeout(() => {
      form.setValue("categoryId", "")
      form.setValue("brandId", "")
      form.setValue("isFeatured", false)

      setOpen(false)
    }, 1000)
  }

  function onSubmit(value: z.infer<typeof filterProductSchema>) {
    navigate(
      `${location.pathname}?${queryString.stringify({
        ...searchParams,
        categoryId: value.categoryId || undefined,
        brandId: value.brandId || undefined,
        isFeatured: value.isFeatured ? true : undefined,
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
                  name="categoryId"
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
                  name="isFeatured"
                  render={({ field }) => (
                    <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                      <div className="space-y-0.5">
                        <FormLabel className="text-sm">
                          Tampilkan hanya produk unggulan
                        </FormLabel>
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
