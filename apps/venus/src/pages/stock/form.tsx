import { zodResolver } from "@hookform/resolvers/zod"
import { z } from "zod"

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
import { Input } from "@/components/ui/input"
import InputNumber from "@/components/ui/input-number"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Textarea } from "@/components/ui/textarea"
import { CLOSE_INPUT_FORM_WARNING } from "@/constants"
import { cn } from "@/lib/utils"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import { useGetProductListQuery } from "@/store/product/api"
import { setOpenStockForm } from "@/store/stock/action"
import { useCreateStockMutation } from "@/store/stock/api"
import { useState } from "react"
import { useForm } from "react-hook-form"

const stockSchema = z.object({
  productId: z.string().min(1, {
    message: "Product is required",
  }),
  type: z.string().min(1, {
    message: "Type is required",
  }),
  quantity: z.number().min(1, {
    message: "Quantity is required",
  }),
  reason: z.string().min(1, {
    message: "Reason is required",
  }),
  notes: z.string().optional(),
})

export type StockFormData = z.infer<typeof stockSchema>

const typeOptions = [
  { value: "IN", label: "Masuk" },
  { value: "OUT", label: "Keluar" },
]

export default function StockManagementForm() {
  const [closeWarning, setCloseWarning] = useState(false)
  const dispatch = useAppDispatch()
  const {
    isOpenStockForm: isOpen,
    data: { productId },
  } = useAppSelector((state) => state.stock)
  const [createStock, { isLoading: isLoadingCreate }] = useCreateStockMutation()
  const { data: product } = useGetProductListQuery({
    limit: 100,
    page: 1,
  })

  const isLoading = isLoadingCreate

  const form = useForm<StockFormData>({
    resolver: zodResolver(stockSchema),
    defaultValues: {
      productId: productId || "",
      quantity: 0,
      reason: "",
      notes: "",
      type: "IN",
    },
  })

  const formDirty = form.formState.dirtyFields
  const hasFieldsDirty = Object.keys(formDirty).length > 0

  function onCancel() {
    if (hasFieldsDirty) {
      setCloseWarning(true)
    } else {
      dispatch(setOpenStockForm(false))
      form.reset()
    }
  }

  async function onSubmit(value: StockFormData) {
    const { productId: id, ...payload } = value
    const { type, ...rest } = payload
    const quantity = type === "IN" ? payload.quantity : -payload.quantity

    try {
      await createStock({
        id,
        payload: {
          ...rest,
          quantity,
        },
      })
    } catch (error) {
      console.error("Gagal menyimpan stok:", error)
    } finally {
      dispatch(setOpenStockForm(false))
      form.reset()
    }
  }

  return (
    <>
      <Modal
        title={"Manajemen Stok"}
        description={
          "Ubah stok dengan mengisi form berikut. Field yang bertanda (*) wajib diisi."
        }
        open={isOpen}
        onCancel={onCancel}
        footer={<></>}
        className="w-full sm:max-w-4xl"
      >
        <div className="flex flex-col items-stretch gap-4">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <FormField
                control={form.control}
                name="productId"
                required
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Produk</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                      disabled={isLoading}
                    >
                      <FormControl>
                        <SelectTrigger className={cn(field.value && "h-fit")}>
                          <SelectValue placeholder="Pilih produk" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {product?.data?.data?.map((product) => (
                          <SelectItem key={product.id} value={product.id}>
                            <img
                              src={product.imageUrl}
                              alt={product.name}
                              className="aspect-square w-16 rounded-lg"
                            />
                            <div className="flex flex-col items-start gap-1">
                              <p>{product.name}</p>
                              <p className="text-muted-foreground text-xs">
                                {product.description}
                              </p>
                            </div>
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
                name="quantity"
                required
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Jumlah Stok</FormLabel>
                    <FormControl>
                      <InputNumber
                        {...field}
                        placeholder="Masukkan jumlah stok"
                        disabled={isLoading}
                        onChange={(e) => field.onChange(Number(e.target.value))}
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
                    <FormLabel>Tipe Transaksi</FormLabel>
                    <Select
                      onValueChange={field.onChange}
                      value={field.value}
                      disabled={isLoading}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Pilih tipe transaksi" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {typeOptions.map((type) => (
                          <SelectItem key={type.value} value={type.value}>
                            {type.label}
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
                name="reason"
                required
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Alasan</FormLabel>
                    <Input
                      {...field}
                      placeholder="Masukkan alasan"
                      disabled={isLoading}
                    />
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="notes"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Catatan</FormLabel>
                    <FormControl>
                      <Textarea
                        className="resize-none"
                        placeholder="Masukkan catatan tambahan"
                        disabled={isLoading}
                        rows={3}
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />

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
                  {isLoading ? "Menyimpan..." : "Simpan"}
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
          dispatch(setOpenStockForm(false))
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
