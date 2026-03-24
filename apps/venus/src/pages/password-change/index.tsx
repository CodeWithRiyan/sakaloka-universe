import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { motionProps } from "@/lib/reveal"
import { motion } from "motion/react"
import { useEffect } from "react"
import { useNavigate, useSearchParams } from "react-router"

export default function PasswordChangePage() {
  const [searchParams] = useSearchParams()
  const hash = searchParams.get("hash")
  const navigate = useNavigate()

  useEffect(() => {
    console.log("Konfirmasi ubah password: ", hash)
  }, [hash])

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-slate-50 to-slate-100 p-6">
      <Card className="border-t-primary animate-fade-in w-full max-w-md border-t-4 shadow-lg">
        <CardHeader className="flex flex-col items-center space-y-4">
          <div className="border-primary w-full border-b pb-1">
            <h1 className="text-primary mb-0 text-xl font-bold">
              Konfirmasi Password Baru Berhasil!
            </h1>
          </div>
          <div className="relative">
            <motion.img
              src="/svg/GDPR-pana.svg"
              alt="ok-amico"
              className="w-56 object-contain"
              {...motionProps({
                reveal: "zoomIn",
              })}
            />
          </div>
        </CardHeader>

        <CardContent className="space-y-2 text-center">
          <p className="inline-flex flex-col justify-center text-base text-slate-700">
            <strong className="text-lg">
              Password Anda telah berhasil diubah.
            </strong>
            <span className="text-balance">
              Anda sekarang dapat menggunakan password baru untuk login.
            </span>
          </p>
        </CardContent>

        <CardFooter className="flex justify-center">
          <Button onClick={() => navigate("/login")} className="mt-4 w-full">
            Ke Halaman Login
          </Button>
        </CardFooter>
      </Card>
    </div>
  )
}
