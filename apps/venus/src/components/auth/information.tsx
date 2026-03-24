import { Link } from "react-router"

export default function Information({ page }: { page?: string }) {
  const checkPath: (yes: string, no: string) => string = (yes, no) => {
    return page === "login" ? yes : no
  }
  return (
    <div className="flex h-full w-full flex-col justify-end gap-4 text-sm">
      <div className="flex justify-center gap-2">
        <p>{checkPath("Belum", "Sudah")} punya akun?</p>
        <Link
          to={checkPath("/register", "/login") ?? ""}
          className="font-semibold hover:text-blue-600 hover:underline"
        >
          {checkPath("Daftar", "Login")}
        </Link>
      </div>
      <div className="justify flex justify-between">
        <Link
          to="#"
          className="w-32 text-center hover:text-blue-600 hover:underline"
        >
          Kebijakan Privasi
        </Link>
        <Link
          to="#"
          className="w-36 text-center hover:text-blue-600 hover:underline"
        >
          Syarat & Ketentuan
        </Link>
        <Link
          to="#"
          className="w-16 text-center hover:text-blue-600 hover:underline"
        >
          Bantuan
        </Link>
      </div>
    </div>
  )
}
