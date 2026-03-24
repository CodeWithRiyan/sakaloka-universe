import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { motionProps } from "@/lib/reveal"
import { motion } from "motion/react"
import { useNavigate } from "react-router"

export default function RegistrationSuccessPage() {
  const navigate = useNavigate()

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-b from-slate-50 to-slate-100 p-6">
      <Card className="animate-fade-in w-full max-w-md border-t-4 border-t-green-500 shadow-lg">
        <CardHeader className="flex flex-col items-center space-y-4">
          <div className="w-full border-b border-green-500 pb-1">
            <h1 className="mb-0 text-xl font-bold text-green-500">
              Registrasi Akun Berhasil!
            </h1>
          </div>
          <div className="relative">
            <motion.img
              src="/svg/ok-amico.svg"
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
              Anda berhasil dibuat membuat akun baru.
            </strong>
            <span className="text-balance">
              Silahkan cek email Anda untuk lakukan aktivasi akun.
            </span>
          </p>
        </CardContent>

        <CardFooter className="flex justify-center">
          <Button
            variant="success"
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
