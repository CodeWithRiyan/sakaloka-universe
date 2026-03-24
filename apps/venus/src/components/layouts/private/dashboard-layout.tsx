import Loader from "@/components/loader"
import { cn } from "@/lib/utils"
import { useAppSelector } from "@/store/hooks"
import { Suspense } from "react"
import { Outlet } from "react-router"
import Menubar from "./menubar"
import DashboardNavbar from "./navbar"
// import Footer from "./footer"

export default function DashboardLayout() {
  const isCollapse = useAppSelector((state) => state.utils.isCollapse)
  return (
    <>
      <title>SakaPOS | Dashboard</title>
      <meta
        name="description"
        content="Kelola bisnis Anda dengan mudah dan efisien menggunakan SakaPOS"
      />
      <main className="min-h-screen w-full">
        <DashboardNavbar />
        <div
          className={cn(
            "min-h-screen w-screen bg-white p-4 pt-20 pb-24 transition-all md:pb-4 md:pl-28",
            isCollapse && "md:pl-4"
          )}
        >
          <Menubar />
          <Suspense fallback={<Loader className="bg-white text-blue-500" />}>
            <Outlet />
          </Suspense>
        </div>

        {/* <Footer /> */}
      </main>
    </>
  )
}
