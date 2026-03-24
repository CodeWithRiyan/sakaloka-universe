"use client"

import { Button } from "@/components/ui/button"
import { setIsUpdate, setOpenCheckout, updateItems } from "@/store/cart/actions"
import { useAppDispatch } from "@/store/hooks"
import {
  useGetMenuListQuery,
  useGetOrderActiveListQuery,
} from "@/store/order/api"
import type { OrderItem, OrderList } from "@/types/order"
import dayjs from "dayjs"
import { useSearchParams } from "react-router"

export default function TransactionDraft() {
  const dispatch = useAppDispatch()
  const [searchParams, setSearchParams] = useSearchParams()
  const allParams = Object.fromEntries(searchParams.entries())
  const { data: menu, isFetching: isLoadingMenu } = useGetMenuListQuery(
    {},
    { selectFromResult: ({ data, isFetching }) => ({ data, isFetching }) }
  )
  const { data: order, isFetching: isLoadingOrder } =
    useGetOrderActiveListQuery()

  console.log(menu)
  const isLoading = isLoadingMenu || isLoadingOrder
  void isLoading

  const addNewItems = (data: OrderList) => {
    const prevItems: OrderItem[] = data.items.map((d) => {
      // const catalog = menu?.data.data?.find((cat) => cat.id === d.productId);
      return {
        productId: d.productId,
        quantity: d.quantity,
        name: d.itemName,
        price: Number(d.unitPrice),
        totalPrice: Number(d.totalPrice),
        prevQty: d.quantity,
      }
    })
    dispatch(
      updateItems({
        transactionId: data.id,
        prevItems,
      })
    )
    dispatch(setIsUpdate(true))
    setSearchParams({ ...allParams, tab: "edit-transaction" })
  }

  const handleCheckout = (data: OrderList) => {
    const checkoutItems: OrderItem[] = data.items.map((d) => {
      return {
        productId: d.productId,
        quantity: d.quantity,
        name: d.itemName,
        price: Number(d.unitPrice),
        totalPrice: Number(d.totalPrice),
        prevQty: d.quantity,
      }
    })

    console.log("Checkout data:", checkoutItems)

    updateItems({
      transactionId: data.id,
      prevItems: checkoutItems,
    })
    dispatch(setOpenCheckout(true))
  }

  return (
    <div className="flex flex-wrap gap-5">
      {order?.data.map((data) => {
        return (
          <div
            key={data.id}
            title={data.type}
            className={`flex min-h-[580px] w-[340px] flex-col justify-between rounded-lg border-4 border-blue-400 p-3`}
          >
            <div>
              <div className="rounded-lg bg-blue-500 px-5 py-3 text-start text-white">
                <div className="flex justify-between">
                  <strong>{data.type}</strong>
                  <strong className="text-sm">{data.orderNumber}</strong>
                </div>
                <div className="flex justify-between text-sm">
                  <p>{data.customerName}</p>
                  <p>{dayjs(data.createdAt).format("HH:mm DD-MM-YYYY")}</p>
                </div>
              </div>
              <div className="mb-5 max-h-[290px] overflow-y-auto">
                {data.items.map((item) => (
                  <div
                    key={item.productId}
                    className="flex items-center gap-2 border-b-2 border-dotted border-gray-300 py-3"
                  >
                    <div className="flex size-8 flex-none items-center justify-center rounded-lg bg-blue-500 text-sm font-bold text-white">
                      {`${item.quantity}x`}
                    </div>
                    <p className="truncate" title={item.itemName}>
                      {item.itemName}
                    </p>
                  </div>
                ))}
              </div>
              {data.notes && (
                <div className="mb-5 rounded-lg border border-yellow-400 bg-yellow-50/20 px-3 py-2 text-sm text-yellow-500">
                  <div className="max-h-24 overflow-y-auto text-start">
                    <p>
                      <strong>Catatan: </strong>
                      <span>{data.notes}</span>
                    </p>
                  </div>
                </div>
              )}
            </div>
            <div className="flex gap-2">
              <Button
                type="button"
                variant="secondary"
                className="w-full"
                onClick={() => addNewItems(data)}
              >
                Lanjutkan
              </Button>
              <Button
                type="button"
                className="w-full"
                onClick={() => handleCheckout(data)}
              >
                Checkout
              </Button>
            </div>
          </div>
        )
      })}
    </div>
  )
}
