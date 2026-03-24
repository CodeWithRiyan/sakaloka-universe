import z from "zod"

export const orderSchema = z.object({
  items: z
    .array(
      z.object({
        productId: z.string(),
        quantity: z.number().int().positive(),
        unitPrice: z.number().nonnegative(),
      })
    )
    .nonempty(),
  paymentMethod: z.string(),
  totalPayment: z.number().nonnegative(),
  totalTax: z.number().nonnegative().optional(),
  type: z.string(),
  tableNumber: z.number().int().positive().optional(),
  customerName: z.string().optional(),
  notes: z.string().optional(),
})

export type OrderSchema = z.infer<typeof orderSchema>
