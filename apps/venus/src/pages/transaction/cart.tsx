"use client"

import { Button } from "@/components/ui/button"
import { formatIDR } from "@/lib/utils"
import {
  resetChart,
  setOpenCheckout,
  setOpenDraft,
  updateItems,
} from "@/store/cart/actions"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import { FaSave, FaTrash } from "react-icons/fa"
import { TbShoppingCart } from "react-icons/tb"
import CartItem from "./cart-item"

export default function Cart() {
  const dispatch = useAppDispatch()
  const { cart, totalAmount, transactionId } = useAppSelector(
    (state) => state.cart
  )
  const totalPrice = formatIDR(totalAmount)

  const handleCheckout = () => {
    dispatch(updateItems({ transactionId, prevItems: cart }))
    dispatch(setOpenCheckout(true))
  }

  return (
    <div className="flex h-full flex-col justify-between">
      <div className="header mb-2 flex flex-row justify-between border-b-2 border-gray-500 px-4 pt-4 pb-2 text-lg font-semibold">
        <>
          <div className="flex gap-2">
            <div className="flex items-center justify-center">
              <TbShoppingCart className="text-xl" />
            </div>
            <span>Keranjang</span>
          </div>
          <span>{cart.length} Menu</span>
        </>
      </div>
      <div className="flex h-[calc(100%-55px)] flex-col justify-between">
        <div className="h-full overflow-auto px-4">
          {cart.map((data) => (
            <CartItem
              key={data.productId}
              productId={data.productId}
              name={data.name}
              price={data.price}
              quantity={data.quantity}
              prevQty={data.prevQty}
            />
          ))}
        </div>
        <div className="footer flex h-[150px] flex-none flex-col justify-end border-t-2 border-solid">
          <div>
            <div className="mb-2 flex justify-between px-4 text-base font-semibold">
              <span>Total Tagihan</span>
              <span>{totalPrice}</span>
            </div>
            <div className="flex">
              <Button
                type="button"
                variant="destructive"
                size="icon_xl"
                disabled={cart.length === 0}
                className="flex-none rounded-none text-lg font-bold"
                onClick={() => dispatch(resetChart())}
              >
                <FaTrash />
              </Button>
              <Button
                type="button"
                size="icon_xl"
                disabled={cart.length === 0}
                className="flex-none rounded-none bg-amber-500 text-lg font-bold hover:bg-amber-500/90"
                onClick={() => dispatch(setOpenDraft(true))}
              >
                <FaSave />
              </Button>
              <Button
                type="button"
                disabled={cart.length === 0}
                size="xl"
                className="w-full rounded-none px-8 font-bold"
                onClick={() => handleCheckout()}
              >
                CHECKOUT
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
