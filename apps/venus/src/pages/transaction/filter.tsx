import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
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
import { cn } from "@/lib/utils"
import { zodResolver } from "@hookform/resolvers/zod"
import dayjs from "dayjs"
import { CalendarIcon, Filter } from "lucide-react"
import queryString from "query-string"
import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import { useLocation, useNavigate } from "react-router"
import z from "zod"

const filterOrderSchema = z.object({
  date: z
    .object({
      from: z.date(),
      to: z.date().optional(),
    })
    .optional()
    .refine(
      (data) => {
        if (!data) return true
        if (data.to) {
          return data.to >= data.from
        }
        return true
      },
      {
        message: "End date must be after start date",
      }
    ),
})

export default function OrderFilter() {
  const [open, setOpen] = useState(false)

  const isLoading = false
  const location = useLocation()
  const navigate = useNavigate()
  const searchParams = queryString.parse(location.search)
  const fromDate = searchParams.fromDate as string
  const toDate = searchParams.toDate as string

  const filterLengthActive = [fromDate].filter((item) => Boolean(item)).length

  const form = useForm<z.infer<typeof filterOrderSchema>>({
    resolver: zodResolver(filterOrderSchema),
    defaultValues: {
      date: fromDate
        ? {
            from: new Date(fromDate),
            to: toDate ? new Date(toDate) : undefined,
          }
        : undefined,
    },
  })

  useEffect(() => {
    form.setValue(
      "date",
      fromDate
        ? {
            from: new Date(fromDate),
            to: toDate ? new Date(toDate) : undefined,
          }
        : undefined
    )
  }, [form, fromDate, toDate])

  function onReset() {
    form.reset({
      date: undefined,
    })

    navigate(
      `${location.pathname}?${queryString.stringify({
        ...searchParams,
        fromDate: undefined,
        toDate: undefined,
      })}`
    )

    setTimeout(() => {
      setOpen(false)
    }, 300)
  }

  function onSubmit(value: z.infer<typeof filterOrderSchema>) {
    if (!value.date?.from) return

    const params = {
      ...searchParams,
      fromDate: dayjs(value.date.from).format("YYYY-MM-DD"),
      toDate: value.date.to
        ? dayjs(value.date.to).format("YYYY-MM-DD")
        : undefined,
    }

    navigate(`${location.pathname}?${queryString.stringify(params)}`, {
      replace: true,
    })

    setTimeout(() => {
      setOpen(false)
    }, 300)
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
            <p className="text-muted-foreground text-sm">
              Filter data transaksi
            </p>
          </div>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
              <div className="grid gap-2">
                <FormField
                  control={form.control}
                  name="date"
                  render={({ field }) => (
                    <FormItem className="flex flex-col">
                      <FormLabel>Tanggal Transaksi</FormLabel>
                      <Popover>
                        <PopoverTrigger asChild>
                          <FormControl>
                            <Button
                              variant={"outline"}
                              className={cn(
                                "w-full pl-3 text-left font-normal",
                                !field.value && "text-muted-foreground"
                              )}
                            >
                              {field.value?.from ? (
                                field.value.to ? (
                                  <>
                                    {dayjs(field.value.from)
                                      .locale("id")
                                      .format("DD MMM YYYY")}{" "}
                                    -{" "}
                                    {dayjs(field.value.to)
                                      .locale("id")
                                      .format("DD MMM YYYY")}
                                  </>
                                ) : (
                                  dayjs(field.value.from).format("DD MMM YYYY")
                                )
                              ) : (
                                <span>Pilih tanggal</span>
                              )}
                              <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                            </Button>
                          </FormControl>
                        </PopoverTrigger>
                        <PopoverContent className="w-auto p-0" align="start">
                          <Calendar
                            mode="range"
                            defaultMonth={field.value?.from}
                            selected={field.value}
                            onSelect={field.onChange}
                            disabled={(date) => date > new Date()}
                            className="rounded-lg"
                          />
                        </PopoverContent>
                      </Popover>
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
                  className="flex-1"
                  onClick={onReset}
                >
                  Reset
                </Button>
                <Button
                  type="submit"
                  size="lg"
                  disabled={isLoading}
                  className="flex-1"
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
