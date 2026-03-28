import { Link } from "react-router"

interface HeaderProps {
  label: string
}

export const Header = ({ label }: HeaderProps) => {
  return (
    <div className="flex w-full flex-col items-center justify-center">
      <Link
        to="/login"
        className="mb-5 inline-flex h-16 items-center px-10 md:hidden"
      >
        <span className="text-2xl font-bold text-blue-500">SakaPOS</span>
      </Link>
      <div className="w-full border-b border-gray-400 pb-1">
        <h1 className="mb-0 text-2xl font-bold">{label}</h1>
      </div>
    </div>
  )
}
