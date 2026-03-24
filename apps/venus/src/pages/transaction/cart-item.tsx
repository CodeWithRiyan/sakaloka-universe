import { cn, formatIDR } from "@/lib/utils"
import { addItem, removeItem } from "@/store/cart/actions"
import { useAppDispatch } from "@/store/hooks"
import { useGetMenuListQuery } from "@/store/order/api"
import type { OrderItem } from "@/types/order"
import { PiMinusBold, PiPlusBold } from "react-icons/pi"
import { useSearchParams } from "react-router"

export default function CartItem(item: OrderItem) {
  const dispatch = useAppDispatch()
  const [searchParams] = useSearchParams()
  // const search = searchParams.get("search")
  // const favorite = searchParams.get("favorite") || "false"
  // const category = searchParams.get("category") || "all"
  const active = searchParams.get("tab") || "new-transaction"
  const { data: menu } = useGetMenuListQuery(
    {},
    {
      selectFromResult: ({ data }) => ({ data }),
    }
  )

  const itemPrice = formatIDR(item.price)

  const stock =
    menu?.data.data?.find((m) => m.id === item.productId)?.availableStock || 0

  const onAddItem = () => {
    dispatch(addItem(item))
  }

  const onRemoveItem = () => {
    dispatch(removeItem(item))
  }

  const edit = active === "edit-transaction"
  const currentQty = item.quantity
  const prevQty = item.prevQty ?? 0

  return (
    <div className="flex items-center justify-between border-b border-dashed py-2">
      <div className="flex w-full flex-col text-left">
        <span className="w-full truncate font-bold" title={item.name}>
          {item.name}
        </span>
        <span>
          {itemPrice} x {item.quantity}
        </span>
      </div>
      <div className="flex flex-none">
        <div className="flex gap-1 pr-[1px]">
          <button
            className={cn(
              "bg-destructive flex size-9 items-center justify-center text-lg text-white transition-all hover:scale-105 active:scale-90",
              edit && "invisible",
              currentQty > prevQty && "visible"
            )}
            onClick={onRemoveItem}
          >
            <PiMinusBold />
          </button>
          <button
            className={cn(
              "bg-primary flex size-9 items-center justify-center text-lg text-white transition-all hover:scale-105 active:scale-90",
              currentQty >= stock + prevQty && "invisible"
            )}
            onClick={onAddItem}
          >
            <PiPlusBold />
          </button>
        </div>
      </div>
    </div>
  )
}
