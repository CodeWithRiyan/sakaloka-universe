import { Badge } from "@/components/ui/badge"
import { cn, formatIDR } from "@/lib/utils"
import { addItem } from "@/store/cart/actions"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import type { Menu } from "@/types/order"
import { PiImageSquareFill, PiStarFill } from "react-icons/pi"

export default function TransactionMenu({ product }: { product: Menu[] }) {
  const dispatch = useAppDispatch()
  const { cart } = useAppSelector((state) => state.cart)

  const addToCart = (data: Menu) => {
    dispatch(
      addItem({
        productId: data.id,
        name: data.name,
        price: data.basePrice,
        quantity: 1,
      })
    )
  }

  return (
    <div className="h-full w-full overflow-y-auto">
      <div className="flex flex-wrap gap-4">
        {product.map((item) => {
          const currentQty =
            cart.find((f) => f.productId === item.id)?.quantity ?? 0
          const prevQty =
            cart.find((f) => f.productId === item.id)?.prevQty ?? 0
          const stock = item.availableStock
          const liveStock = stock - currentQty

          return (
            <button
              key={item.id}
              className={cn(
                "relative flex h-[260px] w-48 flex-col bg-white p-1 text-start shadow-sm",
                (stock <= 0 || currentQty >= stock + prevQty) &&
                  "cursor-not-allowed"
              )}
              disabled={stock <= 0 || currentQty >= stock + prevQty}
              onClick={() => addToCart(item)}
            >
              <div className="relative mb-1 flex aspect-square w-full items-center justify-center overflow-hidden bg-gray-200">
                {item.imageUrl ? (
                  <img
                    src={item.imageUrl}
                    alt={item.name}
                    className="h-full w-full object-cover"
                  />
                ) : (
                  <PiImageSquareFill className="text-2xl text-gray-500" />
                )}
                <div className="absolute top-0 left-0">
                  {item.isFeatured && (
                    <div className="flex size-8 flex-none items-center justify-center border-yellow-400 bg-yellow-50 text-xl text-yellow-400">
                      <PiStarFill />
                    </div>
                  )}
                </div>
              </div>
              <p className="line-clamp-2 text-sm" title={item.name}>
                {item.name}
              </p>
              <strong className="text-sm">{formatIDR(item.basePrice)}</strong>
              {liveStock > 0 && liveStock <= 10 && (
                <Badge variant="warning" className="absolute right-1 bottom-1">
                  {stock - currentQty} Tersisa
                </Badge>
              )}
              {(stock <= 0 || currentQty >= stock + prevQty) && (
                <Badge
                  variant="destructive"
                  className="absolute right-1 bottom-1"
                >
                  Habis
                </Badge>
              )}
            </button>
          )
        })}
      </div>
    </div>
  )
}
