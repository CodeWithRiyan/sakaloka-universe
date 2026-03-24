import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogPortal,
  DialogTitle,
} from "@/components/ui/dialog"
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
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { cn, formatIDR } from "@/lib/utils"
import { orderSchema, type OrderSchema } from "@/schema/order"
import {
  resetChart,
  setOpenCheckout,
  setOpenResult,
} from "@/store/cart/actions"
import { useAppSelector } from "@/store/hooks"
import {
  useCreateOrderMutation,
  useGetOrderListQuery,
  useUpdateOrderMutation,
} from "@/store/order/api"
import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect } from "react"
import { useForm, useWatch } from "react-hook-form"
import { useDispatch } from "react-redux"

export default function Checkout() {
  const dispatch = useDispatch()
  const [createOrder, { isLoading: isLoadingCreate }] = useCreateOrderMutation()
  const [updateOrder, { isLoading: isLoadingUpdate }] = useUpdateOrderMutation()
  const { data: transactions } = useGetOrderListQuery({})
  const { cart, transactionId, openCheckout, totalAmount } = useAppSelector(
    (state) => state.cart
  )
  const transactionTarget = transactions?.data.data.find(
    (trx) => trx.id === transactionId
  )
  const isLoading = isLoadingCreate || isLoadingUpdate
  const tax = 0

  const form = useForm<OrderSchema>({
    resolver: zodResolver(orderSchema),
    defaultValues: {
      type: "RETAIL",
      paymentMethod: "CASH",
      notes: "",
      totalPayment: 0,
      customerName: transactionId ? transactionTarget?.customerName : "",
      tableNumber: 1,
      totalTax: tax,
      items: [],
    },
  })

  const { control } = form

  // Update items when cart changes
  useEffect(() => {
    const cartItems = cart.map((item) => ({
      productId: item.productId,
      quantity: item.quantity,
      unitPrice: item.price,
    }))
    form.setValue("items", cartItems)
  }, [cart, form])

  console.log("Cart:", cart)
  console.log({
    data: form.getValues(),
    errors: form.formState.errors,
  })
  const quickChoice = ["UANG PAS", 50000, 100000, 150000, 200000, 250000]
  const handleShortcut = (value: string | number) => {
    if (value === "UANG PAS") {
      form.setValue("totalPayment", totalAmount + tax)
    } else {
      form.setValue("totalPayment", Number(value))
    }
  }

  const onFinish = async (values: OrderSchema) => {
    const payload = {
      ...values,
      customerName: transactionId
        ? transactionTarget?.customerName
        : values.customerName,
    }

    if (!transactionId) {
      await createOrder(payload)
    } else {
      await updateOrder({
        id: transactionId,
        payload,
      })
    }

    dispatch(setOpenResult(true))
    dispatch(setOpenCheckout(false))
    dispatch(resetChart())
    form.reset()
  }

  const onCancel = () => {
    dispatch(setOpenCheckout(false))
    form.reset({
      items: cart.map((item) => ({
        productId: item.productId,
        quantity: item.quantity,
        unitPrice: item.price,
      })),
    })
  }

  const type = [
    {
      label: "DINE-IN",
      value: "DINEIN",
    },
    {
      label: "TAKEAWAY",
      value: "TAKEAWAY",
    },
  ]

  const totalPayment = useWatch({
    control,
    name: "totalPayment",
  })

  const customerName = useWatch({
    control,
    name: "customerName",
  })

  console.log({
    data: form.getValues(),
    errors: form.formState.errors,
  })

  return (
    <Dialog open={openCheckout}>
      <DialogPortal>
        <DialogContent
          containerClassName="lg:items-center"
          className="w-full sm:w-[600px] lg:w-[800px]"
          onClickClose={onCancel}
          onClickOutside={onCancel}
        >
          <DialogHeader>
            <DialogTitle className="text-xl font-bold">Checkout</DialogTitle>
          </DialogHeader>
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(onFinish)}
              className="grid w-full grid-cols-1 gap-4 lg:grid-cols-2"
            >
              <div className="flex flex-col items-stretch gap-3">
                <div
                  className={cn(
                    "flex h-12 w-full items-center justify-between rounded-lg bg-gray-300 px-4 text-base font-bold text-white",
                    (totalPayment ?? 0) >= totalAmount + tax && "bg-blue-500"
                  )}
                >
                  <span>Total Tagihan</span>
                  <span>{formatIDR(totalAmount + tax)}</span>
                </div>
                <FormField
                  control={form.control}
                  name="totalPayment"
                  defaultValue={form.getValues("totalPayment")}
                  render={({ field }) => (
                    <FormItem className="flex flex-row items-center justify-between rounded-lg border p-4">
                      <div className="space-y-0.5">
                        <FormLabel className="w-40 flex-none text-base">
                          Pembayaran
                        </FormLabel>
                        <FormDescription>
                          Jumlah uang yang diberikan oleh pelanggan kepada
                          kasir.
                        </FormDescription>
                      </div>
                      <FormControl>
                        <Input
                          {...field}
                          type="number"
                          autoComplete="off"
                          min={0}
                          step={500}
                          className="h-10 w-full py-1"
                          onChange={(e) =>
                            field.onChange(e.target.valueAsNumber)
                          }
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <div className="flex w-full flex-col items-center justify-center gap-4 rounded-lg border border-gray-300 p-4 text-lg font-bold lg:hidden">
                  <span>Pilihan Cepat</span>
                  <div className="grid w-full grid-cols-2 justify-center gap-3">
                    {quickChoice.map((value, index) => (
                      <button
                        key={index}
                        type="button"
                        className={cn(
                          "flex h-8 w-full items-center justify-center rounded-lg border border-gray-300 text-base",
                          value === "UANG PAS" &&
                            "border-none bg-[rgb(0,190,0)] text-white"
                        )}
                        onClick={() => handleShortcut(value)}
                      >
                        {typeof value === "string" ? value : formatIDR(value)}
                      </button>
                    ))}
                  </div>
                </div>
                <FormField
                  control={form.control}
                  name="type"
                  render={({ field }) => (
                    <FormItem className="flex flex-col rounded-lg border p-4">
                      <FormLabel className="text-base">
                        Tipe Transaksi
                      </FormLabel>
                      <FormControl>
                        <RadioGroup
                          onValueChange={field.onChange}
                          defaultValue={field.value}
                          className="flex gap-2"
                        >
                          {type.map((item) => {
                            if (
                              !transactionId ||
                              (transactionId && item.value !== "TAKEAWAY")
                            ) {
                              return (
                                <FormItem
                                  key={item.label}
                                  className={cn(
                                    "group flex items-center space-y-0 space-x-3 rounded-lg border px-5 py-3",
                                    form.getValues("type") === item.value
                                      ? "border-blue-400 bg-blue-50/50"
                                      : "cursor-pointer"
                                  )}
                                  onClick={() =>
                                    form.setValue("type", item.value)
                                  }
                                >
                                  <FormControl>
                                    <RadioGroupItem
                                      value={item.value}
                                      className="hidden"
                                    />
                                  </FormControl>
                                  <label
                                    htmlFor="type"
                                    className={cn(
                                      "cursor-pointer text-sm font-medium",
                                      form.getValues("type") === item.value &&
                                        "text-blue-400"
                                    )}
                                    style={{ marginLeft: 0 }}
                                  >
                                    {item.label}
                                  </label>
                                </FormItem>
                              )
                            }
                          })}
                        </RadioGroup>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <div className="flex flex-col items-stretch gap-3">
                <FormField
                  control={form.control}
                  name="customerName"
                  disabled={transactionId ? true : false}
                  render={({ field }) => (
                    <FormItem className="flex flex-row items-center justify-between gap-3 rounded-lg border p-4">
                      <FormLabel className="w-40 flex-none text-base">
                        Nama Pelanggan
                      </FormLabel>
                      <FormControl>
                        <Input className="h-10" autoComplete="off" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <div className="hidden w-full flex-col items-center justify-center gap-4 rounded-lg border border-gray-300 p-4 text-lg font-bold lg:flex">
                  <span>Pilihan Cepat</span>
                  <div className="grid w-full grid-cols-2 justify-center gap-3">
                    {quickChoice.map((value, index) => (
                      <button
                        key={index}
                        type="button"
                        className={cn(
                          "flex h-8 w-full items-center justify-center rounded-lg border border-gray-300 text-base",
                          value === "UANG PAS" &&
                            "border-none bg-[rgb(0,190,0)] text-white"
                        )}
                        onClick={() => handleShortcut(value)}
                      >
                        {typeof value === "string" ? value : formatIDR(value)}
                      </button>
                    ))}
                  </div>
                </div>
              </div>
              <div className="flex justify-end gap-4 lg:col-span-2">
                <Button
                  type="button"
                  variant="destructive"
                  size="lg"
                  className="w-full px-10 font-bold lg:w-fit"
                  onClick={onCancel}
                >
                  BATAL
                </Button>
                <Button
                  type="submit"
                  size="lg"
                  className="w-full px-10 font-bold lg:w-fit"
                  disabled={
                    !((totalPayment ?? 0) >= totalAmount) ||
                    customerName === "" ||
                    isLoading
                  }
                  loading={isLoading}
                >
                  SIMPAN
                </Button>
              </div>
            </form>
          </Form>
        </DialogContent>
      </DialogPortal>
    </Dialog>
  )
}
