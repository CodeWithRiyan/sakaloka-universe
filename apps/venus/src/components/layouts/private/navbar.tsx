"use client"

import { Button } from "@/components/ui/button"
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu"
import { Skeleton } from "@/components/ui/skeleton"
import { useAuth } from "@/hooks/use-auth" // Import custom hook
import useBreakpoint from "@/hooks/use-breakpoint"
import { logout } from "@/lib/auth"
import { cn } from "@/lib/utils"
import { PrivateRoute } from "@/routes/routes"
import {
  useGetBranchCurrentQuery,
  useGetBranchListQuery,
  useSelectBranchMutation,
} from "@/store/branch/api"
import { useAppDispatch, useAppSelector } from "@/store/hooks"
import { setCollapse } from "@/store/utils/action"
import {
  IoHelpCircleOutline,
  IoLogOutOutline,
  IoSettingsOutline,
} from "react-icons/io5"
import { PiBell, PiListBold, PiStorefront, PiUser } from "react-icons/pi"
import { Link, useLocation, useNavigate } from "react-router"

export default function DashboardNavbar() {
  const navigate = useNavigate()
  const dispatch = useAppDispatch()
  const isCollapse = useAppSelector((state) => state.utils.isCollapse)

  const { user, isLoggedIn } = useAuth()

  const { data: branch, isFetching } = useGetBranchListQuery({})
  const { data: currentBranch } = useGetBranchCurrentQuery()
  const [selectBranch, { isLoading: isSelecting }] = useSelectBranchMutation()

  const handleChangeBranch = async (id: string) => {
    try {
      await selectBranch({ organizationId: id }).unwrap()
    } catch (error) {
      console.error("Gagal memilih branch:", error)
    }
  }

  const sideMenu = [
    {
      label: "pengaturan",
      active: "settings",
      path: "/dashboard/settings",
      icon: <IoSettingsOutline className="text-xl" />,
    },
    {
      label: "bantuan",
      active: "help",
      path: "/dashboard/help",
      icon: <IoHelpCircleOutline className="text-xl" />,
    },
    {
      label: "keluar",
      path: "#",
      icon: <IoLogOutOutline className="text-xl" />,
    },
  ]

  const { pathname } = useLocation()
  const pathActive = PrivateRoute.find((data) =>
    data.children
      ? data.children.find((dataSub) => dataSub.path === pathname)
      : data.path === pathname
  )

  const { md } = useBreakpoint()

  if (!isLoggedIn) {
    navigate("/login")
    return null
  }

  return (
    <nav className="bg-primary fixed top-0 z-40 flex h-16 w-screen items-center justify-between px-4 text-white shadow-sm transition-all">
      <div className="flex items-center gap-4">
        {!pathActive?.path?.includes("/dashboard/settings") && (
          <Button
            variant="ghost"
            size="icon_md"
            className="hidden hover:bg-blue-400 md:flex"
            onClick={() => dispatch(setCollapse(!isCollapse))}
          >
            <PiListBold className="text-2xl text-white" />
          </Button>
        )}
        <NavigationMenu
          classNameViewportContainer="left-0 right-auto"
          classNameViewport="min-w-[300px]"
        >
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuTrigger
                className="right-auto flex items-center gap-3 text-left"
                variant="transparent"
                showChevron={md}
                disabled={isFetching || isSelecting}
              >
                {isFetching || isSelecting ? (
                  <>
                    <Skeleton className="size-10 rounded-full" />
                    <div className="hidden h-8 w-32 flex-col justify-between pr-2 md:flex">
                      <Skeleton className="h-4 w-full" />
                      <Skeleton className="h-3 w-[60%]" />
                    </div>
                  </>
                ) : (
                  <>
                    <div className="flex size-10 items-center justify-center rounded-full bg-white text-xl text-gray-500">
                      <PiStorefront />
                    </div>
                    <div className="hidden w-32 pr-2 md:block">
                      <p className="truncate text-sm font-bold">
                        {currentBranch?.data.name || "Pilih cabang"}
                      </p>
                      {currentBranch?.data.address && (
                        <p className="text-xs font-light">
                          {currentBranch?.data.address}
                        </p>
                      )}
                    </div>
                  </>
                )}
              </NavigationMenuTrigger>
              <NavigationMenuContent className="md:w-full">
                <ul className="grid w-full gap-1 p-2">
                  {branch?.data.data.map((data) => {
                    return (
                      <li key={data.id} className="w-full">
                        <NavigationMenuLink asChild>
                          <button
                            className={cn(
                              "flex w-full items-center gap-2 rounded-md px-2 py-1",
                              currentBranch?.data.id === data.id
                                ? "bg-primary text-white"
                                : "hover:bg-accent"
                            )}
                            onClick={() => handleChangeBranch(data.id)}
                            disabled={isFetching || isSelecting}
                          >
                            <div className="flex size-10 flex-none items-center justify-center rounded-full bg-gray-50 text-xl text-gray-500">
                              <PiStorefront />
                            </div>
                            <div className="w-full pr-2 text-start">
                              <p className="truncate text-sm font-bold">
                                {data?.name || "Pilih cabang"}
                              </p>
                              {data?.address && (
                                <p className="text-xs font-light">
                                  {data.address}
                                </p>
                              )}
                            </div>
                          </button>
                        </NavigationMenuLink>
                      </li>
                    )
                  })}
                </ul>
              </NavigationMenuContent>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
      </div>
      <p className="hidden text-xl font-bold text-white md:block">SakaPOS</p>
      <div className="flex h-full items-center gap-4">
        <Button
          variant={"link"}
          size={"icon_md"}
          className="text-2xl text-white transition-colors hover:text-slate-700"
        >
          <PiBell />
        </Button>
        <hr className="h-6 w-[1px] bg-white" />
        <NavigationMenu>
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuTrigger
                className="flex items-center gap-3 text-left"
                variant="transparent"
                showChevron={md}
              >
                {!user ? (
                  <>
                    <Skeleton className="size-10 rounded-full" />
                    <div className="hidden h-8 w-32 flex-col justify-between pr-2 md:flex">
                      <Skeleton className="h-4 w-full" />
                      <Skeleton className="h-3 w-[60%]" />
                    </div>
                  </>
                ) : (
                  <>
                    <div className="flex size-10 items-center justify-center rounded-full bg-white text-xl text-gray-500">
                      <PiUser />
                    </div>
                    <div className="hidden w-32 pr-2 md:block">
                      <p className="truncate text-sm font-bold">
                        {user?.fullName}
                      </p>
                      <p className="text-xs font-light">
                        {user?.role?.name || "User"}
                      </p>
                    </div>
                  </>
                )}
              </NavigationMenuTrigger>
              <NavigationMenuContent>
                <ul className="grid w-[200px] gap-1 p-2">
                  {sideMenu?.map((data) => (
                    <li key={data.label}>
                      <NavigationMenuLink asChild>
                        {data.path === "#" ? (
                          <button
                            className="hover:bg-accent flex w-full items-center gap-2 rounded-md px-2 py-1"
                            onClick={() => {
                              logout()
                              navigate("/login")
                            }}
                          >
                            {data.icon}
                            {data.label}
                          </button>
                        ) : (
                          <Link
                            to={data.path!}
                            className="hover:bg-accent flex w-full items-center gap-2 rounded-md px-2 py-1"
                          >
                            {data.icon}
                            {data.label}
                          </Link>
                        )}
                      </NavigationMenuLink>
                    </li>
                  ))}
                </ul>
              </NavigationMenuContent>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
      </div>
    </nav>
  )
}
