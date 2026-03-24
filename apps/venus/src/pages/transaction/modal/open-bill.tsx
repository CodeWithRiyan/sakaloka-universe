"use client"

import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogPortal } from "@/components/ui/dialog"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group"
import { cn } from "@/lib/utils"
import { orderSchema, type OrderSchema } from "@/schema/order"
import { setOpenDraft } from "@/store/cart/actions"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import {
  useCreateOrderMutation,
  useGetOrderActiveListQuery,
  useUpdateOrderMutation,
} from "@/store/order/api"
import { zodResolver } from "@hookform/resolvers/zod"
import { useEffect } from "react"
import { useForm, useWatch } from "react-hook-form"
import { useSearchParams } from "react-router"

export default function ModalOpenBill() {
  const tax = 0
  const dispatch = useAppDispatch()
  const [createOrder, { isLoading: isLoadingCreate }] = useCreateOrderMutation()
  const [updateOrder, { isLoading: isLoadingUpdate }] = useUpdateOrderMutation()
  const { cart, transactionId, openDraft } = useAppSelector(
    (state) => state.cart
  )
  const { data: order, isFetching: isLoadingOrder } =
    useGetOrderActiveListQuery()
  const isLoading = isLoadingOrder || isLoadingCreate || isLoadingUpdate
  const [searchParams] = useSearchParams()
  const active = searchParams.get("tab") || "new-transaction"

  const name =
    order?.data?.find((f) => f.id === transactionId)?.customerName || ""
  const form = useForm<OrderSchema>({
    resolver: zodResolver(orderSchema),
    defaultValues: {
      type: "DINEIN",
      paymentMethod: "CASH",
      notes: "",
      totalPayment: 0,
      customerName: name,
      tableNumber: 1,
      totalTax: tax,
      items: [],
    },
  })

  const { control } = form

  useEffect(() => {
    if (cart.length > 0) {
      form.setValue(
        "items",
        cart.map((item) => ({
          productId: item.productId,
          quantity: item.quantity,
          unitPrice: item.price,
        }))
      )
      const total = cart.reduce((acc, item) => acc + Number(item.totalPrice), 0)
      form.setValue("totalPayment", total)
    }
  }, [cart, form])
  console.log({
    data: form.getValues(),
    errors: form.formState.errors,
  })

  useEffect(() => {
    form.setValue("customerName", name)
  }, [form, name])

  const onFinish = async (values: OrderSchema) => {
    const items = cart.map((item) => ({
      productId: item.productId,
      quantity: item.quantity,
      unitPrice: item.price,
    }))

    if (!transactionId) {
      await createOrder({
        ...values,
        items,
      })
    } else {
      await updateOrder({
        id: transactionId,
        payload: {
          ...values,
          items,
        },
      })
    }

    form.reset()
    dispatch(setOpenDraft(false))
  }

  const type = [
    {
      label: "DINE-IN",
      value: "DINEIN",
    },
  ]

  const customerName = useWatch({
    control,
    name: "customerName",
  })

  const isButtonDisabled =
    active === "new-transaction" && (customerName === "" || isLoading)
      ? true
      : false

  return (
    <Dialog open={openDraft}>
      <DialogPortal>
        <DialogContent
          centered
          showClose={false}
          onClickOutside={() => dispatch(setOpenDraft(false))}
        >
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(onFinish)}
              className="w-full space-y-3"
            >
              <FormField
                control={form.control}
                name="customerName"
                disabled={active === "edit-transaction"}
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
              <FormField
                control={form.control}
                name="type"
                render={({ field }) => (
                  <FormItem className="flex flex-col rounded-lg border p-4">
                    <FormLabel className="text-base">Tipe Transaksi</FormLabel>
                    <FormControl>
                      <RadioGroup
                        onValueChange={field.onChange}
                        defaultValue={field.value}
                        className="flex gap-2"
                      >
                        {type.map((item) => (
                          <FormItem
                            key={item.label}
                            className={cn(
                              "group flex items-center space-y-0 space-x-3 rounded-lg border px-5 py-3",
                              form.getValues("type") === item.value
                                ? "border-blue-400 bg-blue-50/50"
                                : "cursor-pointer"
                            )}
                            onClick={() => form.setValue("type", item.value)}
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
                                "text-sm font-medium",
                                form.getValues("type") === item.value
                                  ? "text-blue-400"
                                  : "-z-10"
                              )}
                              style={{ marginLeft: 0 }}
                            >
                              {item.label}
                            </label>
                          </FormItem>
                        ))}
                      </RadioGroup>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="flex justify-end gap-4">
                <Button
                  type="button"
                  variant="destructive"
                  size="lg"
                  className="font-bold"
                  onClick={() => dispatch(setOpenDraft(false))}
                >
                  BATAL
                </Button>
                <Button
                  type="submit"
                  size="lg"
                  className="font-bold"
                  disabled={isButtonDisabled}
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
