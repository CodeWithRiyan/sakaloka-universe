import Loader from "@/components/loader"
import { ArrowLeft } from "lucide-react"
import { Suspense } from "react"
import { LuPrinter } from "react-icons/lu"
import {
  PiLockKeyOpen,
  PiShieldCheck,
  PiStorefront,
  PiUser,
  PiUsersThree,
} from "react-icons/pi"
import { Link, Outlet, useLocation } from "react-router"
import DashboardNavbar from "./navbar"
// import Footer from "./footer"

const menuSettings: {
  label: string
  path: string
  icon: React.ReactNode
  sub?: {
    label: string
    path: string
  }[]
}[] = [
  {
    label: "profil",
    path: "/dashboard/settings",
    icon: <PiUser className="text-xl" />,
  },
  {
    label: "ubah password",
    path: "/dashboard/settings/change-password",
    icon: <PiLockKeyOpen className="text-xl" />,
  },
  {
    label: "manajemen toko",
    path: "/dashboard/settings/branch",
    icon: <PiStorefront className="text-xl" />,
  },
  {
    label: "manajemen role",
    path: "/dashboard/settings/role",
    icon: <PiShieldCheck className="text-xl" />,
  },
  {
    label: "manajemen karyawan",
    path: "/dashboard/settings/employee",
    icon: <PiUsersThree className="text-xl" />,
  },
  {
    label: "manajemen perangkat",
    path: "/dashboard/settings/printer",
    icon: <LuPrinter className="text-xl" />,
  },
]

export default function SettingsLayout() {
  const { pathname } = useLocation()

  return (
    <>
      <title>SakaPOS | Settings</title>
      <meta
        name="description"
        content="Atur pengaturan aplikasi SakaPOS disini"
      />
      <main className="min-h-screen w-full">
        <DashboardNavbar />
        <div className="min-h-screen w-screen space-y-4 bg-white px-4 pt-20 pb-4 md:px-8 md:pb-8">
          <Link
            to="/dashboard"
            className="inline-flex h-9 items-center gap-2 self-start rounded-lg bg-blue-500 px-4 text-sm text-white shadow-sm transition-colors hover:bg-blue-400 hover:text-white"
          >
            <ArrowLeft />
            <span>Kembali</span>
          </Link>
          <div className="flex min-h-[calc(100vh-172px)] gap-10">
            <ul className="flex flex-none flex-col self-start text-slate-600 transition-all md:w-60 lg:w-80">
              {menuSettings.map((data) => (
                <li
                  key={data.label}
                  className="flex h-9 w-full items-center gap-1"
                >
                  <div
                    className={`h-[70%] w-1 flex-none rounded-full ${
                      data.path === pathname ||
                      data.sub?.some((d) => d.path === pathname)
                        ? "bg-blue-500"
                        : "bg-transparent"
                    }`}
                  />
                  <Link
                    to={data.path}
                    className={`inline-flex w-full items-center gap-4 rounded-md px-4 py-2 text-sm capitalize md:px-5 ${
                      (data.path === pathname ||
                        data.sub?.some((d) => d.path === pathname)) &&
                      "bg-gray-50 font-semibold text-slate-800"
                    }`}
                  >
                    {data.icon}
                    <span className="hidden md:block">{data.label}</span>
                  </Link>
                </li>
              ))}
            </ul>
            <Suspense
              fallback={<Loader className="bg-transparent text-blue-500" />}
            >
              <Outlet />
            </Suspense>
          </div>
        </div>

        {/* <Footer /> */}
      </main>
    </>
  )
}
