import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { motionProps } from "@/lib/reveal"
import { cn } from "@/lib/utils"
import { motion } from "motion/react"
import { useEffect } from "react"
import { useLocation, useNavigate, useSearchParams } from "react-router"

export default function ConfirmEmailPage() {
  const [searchParams] = useSearchParams()
  const hash = searchParams.get("hash")
  const location = useLocation()
  const navigate = useNavigate()
  const titlePage = location.pathname.split("/")[1] || "confirm-new-email"
  const isRegister = titlePage === "confirm-email"

  useEffect(() => {
    if (isRegister) {
      console.log("Aktivasi Akun: ", hash)
    } else {
      console.log("Change Email Confirmation: ", hash)
    }
  }, [hash, isRegister])

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-slate-50 to-slate-100 p-6">
      <Card
        className={cn(
          "border-t-primary animate-fade-in w-full max-w-md border-t-4 shadow-lg",
          isRegister && "border-t-green-500"
        )}
      >
        <CardHeader className="flex flex-col items-center space-y-4">
          <div
            className={cn(
              "w-full border-b pb-1",
              isRegister ? "border-green-500" : "border-primary"
            )}
          >
            <h1
              className={cn(
                "mb-0 text-xl font-bold",
                isRegister ? "text-green-500" : "text-primary"
              )}
            >
              {isRegister
                ? "Aktivasi Akun Berhasil!"
                : "Konfirmasi Email Baru Berhasil!"}
            </h1>
          </div>
          <div className="relative">
            <motion.img
              src={
                isRegister ? "/svg/group-amico.svg" : "/svg/message-rafiki.svg"
              }
              alt={isRegister ? "group-amico" : "message-rafiki"}
              className="w-56 object-contain"
              {...motionProps({
                reveal: "zoomIn",
              })}
            />
          </div>
        </CardHeader>

        <CardContent className="space-y-2 text-center">
          <p className="inline-flex flex-col justify-center text-base text-slate-700">
            {isRegister ? (
              <>
                <strong className="text-lg">Selamat!</strong>
                <span>
                  Akun Anda telah berhasil diaktivasi dan siap digunakan.
                </span>
              </>
            ) : (
              <>
                <strong className="text-lg">
                  Email Anda telah berhasil dikonfirmasi.
                </strong>
                <span className="text-balance">
                  Anda sekarang dapat menggunakan email baru untuk login.
                </span>
              </>
            )}
          </p>
        </CardContent>

        <CardFooter className="flex justify-center">
          <Button
            variant={isRegister ? "success" : "default"}
            onClick={() => navigate("/login")}
            className="mt-4 w-full"
          >
            Ke Halaman Login
          </Button>
        </CardFooter>
      </Card>
    </div>
  )
}
