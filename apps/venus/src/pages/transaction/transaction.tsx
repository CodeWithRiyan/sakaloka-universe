"use client"

import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet"

import { useCallback } from "react"
// import { useCartStore, useCatalogueStore } from "@/lib/store";
// import TransactionMenu from "./menu";
import { cn } from "@/lib/utils"
import {
  PiFunnelSimple,
  PiShoppingCartSimple,
  PiStarFill,
} from "react-icons/pi"
// import ModalLoading from "@/components/dashboard/modal-loading";
import { InputSearch } from "@/components/custom/input-search"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@/components/ui/select"
import { useGetCategoriesListQuery } from "@/store/categories/api"
import { useAppSelector } from "@/store/hooks"
import { useGetMenuListQuery } from "@/store/order/api"
import { useSearchParams } from "react-router"
import Cart from "./cart"
import EmptyCart from "./empty-cart"
import TransactionMenu from "./menu"

export default function Transaction() {
  const cart = useAppSelector((state) => state.cart.cart)
  const [searchParams, setSearchParams] = useSearchParams()
  const search = searchParams.get("search")
  const favorite = searchParams.get("favorite") || "false"
  const category = searchParams.get("category") || "all"
  const allSearchParams = Object.fromEntries(searchParams.entries())
  const { data: categories, isFetching: isLoadingCategories } =
    useGetCategoriesListQuery({
      limit: 100,
    })
  const { data: menu, isFetching: isLoadingMenu } = useGetMenuListQuery({
    page: 1,
    limit: 100,
    categoryId: category === "all" ? undefined : category,
    isFeatured: favorite === "true" ? true : undefined,
    search: search || undefined,
  })
  const isLoading = isLoadingCategories || isLoadingMenu
  void isLoading

  const handleSearch = useCallback(
    (value: string) => {
      setSearchParams(
        {
          ...allSearchParams,
          search: value,
        },
        {
          replace: true,
        }
      )
    },
    [allSearchParams, setSearchParams]
  )

  const handleClear = useCallback(() => {
    setSearchParams(
      {
        ...allSearchParams,
        search: "",
      },
      {
        replace: true,
      }
    )
  }, [allSearchParams, setSearchParams])

  const handleFavorite = () => {
    setSearchParams(
      {
        ...allSearchParams,
        favorite: favorite === "true" ? "false" : "true",
      },
      {
        replace: true,
      }
    )
  }

  const handleCategory = (value: string) => {
    setSearchParams(
      {
        ...allSearchParams,
        category: value,
      },
      {
        replace: true,
      }
    )
  }

  return (
    <div className="flex h-full w-full">
      {/* {isLoading && <ModalLoading />} */}
      <div className="flex h-full w-full flex-col gap-4 p-4">
        <div className="flex gap-1">
          <Sheet>
            <SheetTrigger className="text-blue relative flex size-9 flex-none items-center justify-center bg-white text-xl md:size-10 lg:hidden">
              <PiShoppingCartSimple />
              {cart.length > 0 && (
                <div className="absolute top-0 right-0 flex size-5 items-center justify-center rounded-full border-2 border-white bg-red-500 text-xs text-white">
                  {cart.length}
                </div>
              )}
            </SheetTrigger>
            <SheetContent
              side="left"
              className="border-r-none w-[400px] p-0 pt-6 sm:w-[540px]"
            >
              <SheetHeader className="hidden">
                <SheetTitle>Cart</SheetTitle>
              </SheetHeader>
              {cart.length === 0 ? <EmptyCart /> : <Cart />}
            </SheetContent>
          </Sheet>
          <button
            className={cn(
              "flex size-9 flex-none items-center justify-center bg-white text-2xl text-gray-300 hover:ring-blue-400 md:size-10",
              favorite === "true" &&
                "border-yellow-400 bg-yellow-50 text-yellow-400"
            )}
            onClick={handleFavorite}
          >
            <PiStarFill />
          </button>
          <Select onValueChange={handleCategory} defaultValue={category}>
            <SelectTrigger
              className="flex size-9 flex-none items-center justify-center rounded-none bg-white p-0 text-2xl md:size-10"
              icon={<></>}
            >
              <PiFunnelSimple className="size-6" />
            </SelectTrigger>
            <SelectContent className="rounded-none">
              <SelectItem className="rounded-none" value="all">
                Semua
              </SelectItem>
              {categories?.data.data.map((cat) => (
                <SelectItem className="rounded-none" value={cat.id}>
                  {cat.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <InputSearch
            value={search || ""}
            placeholder="Cari menu..."
            onSearch={handleSearch}
            onClear={handleClear}
            // isLoading={isLoading}
            debounceMs={500}
            containerClassName="flex-1"
            className="h-9 w-full rounded-none border-none bg-white md:h-10"
            debounceOptions={{
              leading: false,
              trailing: true,
            }}
          />
        </div>

        <TransactionMenu product={menu?.data.data || []} />
      </div>
      <div
        className={cn(
          "hidden h-full overflow-hidden bg-white text-center shadow-md transition-all lg:block",
          cart.length === 0 ? "w-0" : "w-[350px] flex-none"
        )}
      >
        {cart.length === 0 ? <EmptyCart /> : <Cart />}
      </div>
    </div>
  )
}
