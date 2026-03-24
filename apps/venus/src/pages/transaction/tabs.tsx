"use client"

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { cn } from "@/lib/utils"
import { resetChart } from "@/store/cart/actions"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import { type OrderTabs } from "@/types/order"
import { useSearchParams } from "react-router"
import TransactionDraft from "./draft"
import FinishedTransaction from "./finished"
import Transaction from "./transaction"

export default function TransactionTabs() {
  const dispatch = useAppDispatch()
  const [searchParams, setSearchParams] = useSearchParams()
  const allParams = Object.fromEntries(searchParams.entries())
  const active = searchParams.get("tab") || "new-transaction"
  const isUpdate = useAppSelector((state) => state.cart.isUpdate)

  const handleCancel = (act: OrderTabs) => {
    setSearchParams({ ...allParams, tab: act })
    dispatch(resetChart())
  }

  return (
    <div
      className={cn(
        "bg-primary/20 absolute inset-0 flex gap-4 pt-16 pb-20 md:pb-0 md:pl-24"
        // collapse && "md:pl-4"
      )}
    >
      <Tabs value={active} className="w-full gap-0">
        <TabsList className="h-12 w-full flex-none justify-evenly overflow-x-auto p-0 transition-all duration-300">
          <TabsTrigger
            className="bg-primary/80 hover:bg-primary/90 data-[state=active]:bg-primary h-full w-full px-5 py-2 text-sm text-white data-[state=active]:text-white sm:text-base"
            value="new-transaction"
            onClick={() => handleCancel("new-transaction")}
          >
            Transaksi Baru
          </TabsTrigger>
          {isUpdate && (
            <TabsTrigger
              className="bg-primary/80 hover:bg-primary/90 data-[state=active]:bg-primary h-full w-full px-5 py-2 text-sm text-white data-[state=active]:text-white sm:text-base"
              value="edit-transaction"
              onClick={() => handleCancel("edit-transaction")}
            >
              Ubah Transaksi
            </TabsTrigger>
          )}
          <TabsTrigger
            className="bg-primary/80 hover:bg-primary/90 data-[state=active]:bg-primary h-full w-full px-5 py-2 text-sm text-white data-[state=active]:text-white sm:text-base"
            value="draft"
            onClick={() => handleCancel("draft")}
          >
            Open Bill
          </TabsTrigger>
          <TabsTrigger
            className="bg-primary/80 hover:bg-primary/90 data-[state=active]:bg-primary h-full w-full px-5 py-2 text-sm text-white data-[state=active]:text-white sm:text-base"
            value="finished"
            onClick={() => handleCancel("finished")}
          >
            Selesai
          </TabsTrigger>
        </TabsList>
        <TabsContent
          value="new-transaction"
          className="mt-0 h-[calc(100%-48px)]"
        >
          <Transaction />
        </TabsContent>
        <TabsContent
          value="edit-transaction"
          className="mt-0 h-[calc(100%-48px)]"
        >
          <Transaction />
        </TabsContent>
        <TabsContent
          value="draft"
          className="mt-0 h-[calc(100%-48px)] bg-white p-4"
        >
          <TransactionDraft />
        </TabsContent>
        <TabsContent
          value="finished"
          className="mt-0 h-[calc(100%-48px)] bg-white p-4"
        >
          <FinishedTransaction />
        </TabsContent>
      </Tabs>
    </div>
  )
}
