"use client"

import { getAuthData } from "@/lib/auth"
import { cn } from "@/lib/utils"
import { useAppSelector } from "@/store/hooks"
import { FaBoxes, FaChartPie } from "react-icons/fa"
import { IoFastFood } from "react-icons/io5"
import { MdSpaceDashboard } from "react-icons/md"
import { PiCashRegisterFill } from "react-icons/pi"
import { Link, useLocation } from "react-router"

export default function Menubar() {
  // const dispatch = useAppDispatch();
  const isCollapse = useAppSelector((state) => state.utils.isCollapse)
  const auth = getAuthData()
  const { pathname } = useLocation()
  const selectedBranch = auth?.organization.id

  const mainMenu = [
    {
      label: "dashboard",
      active: "dashboard",
      path: "/dashboard",
      icon: <MdSpaceDashboard className="size-8 overflow-hidden md:size-12" />,
    },
    {
      label: "produk",
      active: "product",
      path: "/dashboard/product",
      icon: <IoFastFood className="size-8 overflow-hidden md:size-12" />,
    },
    {
      label: "stok",
      active: "stock",
      path: "/dashboard/stock",
      icon: <FaBoxes className="size-8 overflow-hidden md:size-12" />,
    },
    {
      label: "kasir",
      active: "transaction",
      path: "/dashboard/transaction",
      icon: (
        <PiCashRegisterFill className="size-8 overflow-hidden md:size-12" />
      ),
    },
    {
      label: "laporan",
      active: "report",
      path: "/dashboard/report",
      icon: <FaChartPie className="size-8 overflow-hidden md:size-12" />,
    },
  ]

  return (
    <menu
      className={cn(
        "bg-primary fixed bottom-0 left-0 z-10 flex h-20 w-screen justify-evenly overflow-hidden text-slate-500 shadow-[0_-1px_2px_0px_rgba(0,0,0,0.1)] transition-all md:h-screen md:w-24 md:flex-col md:pt-16 md:shadow-[1px_0_2px_0_rgba(0,0,0,0.1)]",
        isCollapse && "md:w-0"
      )}
    >
      {mainMenu.map((data) => {
        if (!selectedBranch) {
          return (
            <button
              key={data.label}
              className={cn(
                "group md:W-24 flex h-20 w-full flex-col items-center justify-center gap-1 text-white hover:bg-blue-400 md:h-full",
                pathname === data.path &&
                  "bg-white text-blue-500 hover:bg-white"
              )}
              // onClick={() => }
            >
              {data.icon}
              <p
                className={cn(
                  "text-xs font-medium text-white capitalize",
                  pathname === data.path && "text-blue-500"
                )}
              >
                {data.label}
              </p>
            </button>
          )
        }

        return (
          <Link
            key={data.label}
            to={data.path}
            className={cn(
              "group md:W-24 flex h-20 w-full flex-col items-center justify-center gap-1 text-white hover:bg-blue-400 md:h-full",
              pathname === data.path && "bg-white text-blue-500 hover:bg-white"
            )}
          >
            {data.icon}
            <p
              className={cn(
                "text-xs font-medium text-white capitalize",
                pathname === data.path && "text-blue-500"
              )}
            >
              {data.label}
            </p>
          </Link>
        )
      })}
    </menu>
  )
}
