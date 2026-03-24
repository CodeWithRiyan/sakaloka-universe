"use client"

import { Button } from "@/components/ui/button"
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTrigger,
} from "@/components/ui/sheet"
import { cn } from "@/lib/utils"
import { ChevronRight } from "lucide-react"
import { motion } from "motion/react"
import { useEffect, useState } from "react"
import { HiMenu } from "react-icons/hi"
import { Link } from "react-router"

export default function Navbar() {
  const [scrollY, setScrollY] = useState(0)

  useEffect(() => {
    const handleScroll = () => {
      setScrollY(window.scrollY)
    }

    window.addEventListener("scroll", handleScroll)

    return () => {
      window.removeEventListener("scroll", handleScroll)
    }
  }, [])

  const menu = [
    {
      label: "Home",
      href: "#home",
    },
    {
      label: "Fitur",
      href: "#features",
    },
    {
      label: "Benefit",
      href: "#benefits",
    },
    {
      label: "Pricing",
      href: "#pricing",
    },
    {
      label: "FAQ",
      href: "#faq",
    },
  ]

  const menuItems = menu.map((data, i) => ({
    key: data.label,
    children: (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        whileInView={{ opacity: 1, y: 0 }}
        transition={{
          type: "spring",
          stiffness: 90,
          bounce: 0.15,
          duration: 0.2,
          delay: (i + 1) * 0.2,
        }}
      >
        <Link
          to={data.href}
          className="inline-flex w-full justify-between py-2 text-base font-medium capitalize"
        >
          <span>{data.label}</span>
          <ChevronRight size={20} />
        </Link>
      </motion.div>
    ),
  }))

  const items = [
    ...menuItems,
    {
      key: "Login",
      children: (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{
            type: "spring",
            stiffness: 90,
            bounce: 0.15,
            duration: 0.2,
            delay: menuItems.length * 0.2 + 0.2,
          }}
        >
          <Button href="/login" className="w-full rounded-xl">
            Login
          </Button>
        </motion.div>
      ),
    },
    {
      key: "Register",
      children: (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          transition={{
            type: "spring",
            stiffness: 90,
            bounce: 0.15,
            duration: 0.2,
            delay: menuItems.length * 0.2 + 0.4,
          }}
        >
          <Button
            variant="success"
            href="/register"
            className="w-full rounded-xl"
          >
            Registrasi
          </Button>
        </motion.div>
      ),
    },
  ]

  return (
    <nav
      className={cn(
        "fixed top-0 left-0 z-50 flex w-full items-center justify-between px-10 transition-all duration-700",
        scrollY <= 30
          ? "bg-black/0 px-5 pt-10 text-white md:px-20"
          : "bg-white px-5 py-4 text-slate-500 shadow md:px-10"
      )}
    >
      <Link
        to="/"
        className={`inline-flex flex-none items-center p-1 text-xl font-bold transition-colors duration-700 ${
          scrollY >= 30 ? "text-primary" : "text-white"
        }`}
      >
        SAKAPOS
      </Link>
      <ul className="m-0 hidden items-center gap-2 font-semibold lg:flex">
        {menu.map((item) => (
          <li key={item.label}>
            <a href={item.href} className="inline-flex px-4 py-2 font-medium">
              {item.label}
            </a>
          </li>
        ))}
      </ul>
      <div className="hidden items-center gap-4 lg:flex">
        <Button
          href="/login"
          variant={scrollY >= 30 ? "default" : "white"}
          className="font-semibold transition-colors"
        >
          <span className="hidden sm:flex">Login</span>
          <span className="flex sm:hidden">Login/Registrasi</span>
        </Button>
        <Button href="/register" variant="success" className="font-semibold">
          Registrasi
        </Button>
      </div>
      <Sheet>
        <SheetTrigger
          className={cn(
            "flex size-9 items-center justify-center rounded-lg text-2xl hover:bg-white/10 sm:size-11 lg:hidden",
            scrollY <= 30 ? "text-white" : "text-primary"
          )}
        >
          <HiMenu />
        </SheetTrigger>
        <SheetContent
          side="right"
          className="w-[400px] px-6 pt-10 sm:w-[540px]"
        >
          <SheetHeader className="px-0">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{
                type: "spring",
                stiffness: 90,
                bounce: 0.15,
                duration: 0.2,
              }}
            >
              <Link
                to="/"
                className="text-primary inline-flex flex-none items-center text-xl font-bold transition-colors duration-700"
              >
                SAKAPOS
              </Link>
            </motion.div>
          </SheetHeader>
          <ul className="flex flex-col gap-4">
            {items.map((item) => (
              <li key={item?.key}>{item?.children}</li>
            ))}
          </ul>
        </SheetContent>
      </Sheet>
    </nav>
  )
}
