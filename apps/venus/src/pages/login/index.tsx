import { Link } from "react-router"
import { LoginForm } from "../../components/auth/login-form"

const LoginPage = () => {
  return (
    <>
      <title>SakaPOS | Login</title>
      <meta
        name="description"
        content="Masuk ke akun SakaPOS Anda dan kelola bisnis Anda dengan mudah dan efisien."
      />
      <div className="relative grid h-screen w-full bg-white md:grid-cols-2">
        <LoginForm />
        <div className="z-10 hidden w-full md:block">
          <img
            src="/bg-login.webp"
            alt="slide_image"
            className="absolute top-0 right-0 bottom-0 h-full w-[calc(50%+40px)] object-cover object-left"
          />
          <Link
            to="/login"
            className="absolute top-0 right-0 hidden h-16 items-center px-10 md:flex"
          >
            <span className="text-xl font-bold text-white">SAKAPOS</span>
          </Link>
        </div>
      </div>
    </>
  )
}

export default LoginPage
