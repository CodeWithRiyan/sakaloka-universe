"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { useState } from "react"
import { useForm } from "react-hook-form"
import * as z from "zod"

import { FormError } from "@/components/form-error"
import { FormSuccess } from "@/components/form-success"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { login } from "@/lib/auth"
import { useNavigate, useSearchParams } from "react-router"
import { toast } from "sonner"
import { CardWrapper } from "./card-wrapper"

const LoginSchema = z.object({
  email: z.email({
    message: "Email is required",
  }),
  password: z.string().min(1, {
    message: "Password is required",
  }),
  code: z.optional(z.string()),
})

export function LoginForm() {
  const [searchParams] = useSearchParams()
  const callbackUrl = searchParams.get("callbackUrl")
  const navigate = useNavigate()
  const urlError =
    searchParams.get("error") === "OAuthAccountNotLinked"
      ? "Email already in use with different provider!"
      : ""

  const [showTwoFactor] = useState(false)
  const [error, setError] = useState<string | undefined>("")
  const [success, setSuccess] = useState<string | undefined>("")
  const [isLoading, setIsLoading] = useState<boolean>()

  const form = useForm<z.infer<typeof LoginSchema>>({
    resolver: zodResolver(LoginSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  })

  const onSubmit = async (values: z.infer<typeof LoginSchema>) => {
    setError("")
    setSuccess("")
    setIsLoading(true)

    try {
      const result = await login(values)

      if (result?.success) {
        setSuccess(result.message)
        setTimeout(() => navigate(callbackUrl || "/dashboard"), 1000)
      } else {
        throw new Error(result?.error)
      }
    } catch (error) {
      const err = error as Error
      toast.error("Login gagal", {
        description: err.message,
      })
      setError(err.message)
      console.error("API authentication error:", err.message)
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <CardWrapper headerLabel="LOGIN" pageInformation="login">
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <div className="space-y-4">
            {showTwoFactor && (
              <FormField
                control={form.control}
                name="code"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Two Factor Code</FormLabel>
                    <FormControl>
                      <Input
                        {...field}
                        disabled={isLoading}
                        placeholder="123456"
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
            )}
            {!showTwoFactor && (
              <>
                <FormField
                  control={form.control}
                  name="email"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Email</FormLabel>
                      <FormControl>
                        <Input
                          {...field}
                          disabled={isLoading}
                          placeholder="john.doe@example.com"
                          type="email"
                        />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={form.control}
                  name="password"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Password</FormLabel>
                      <FormControl>
                        <Input
                          {...field}
                          disabled={isLoading}
                          placeholder="******"
                          type="password"
                        />
                      </FormControl>
                      <div className="flex items-center justify-between gap-2">
                        <FormMessage />
                        <span></span>
                        <Button
                          size="sm"
                          variant="link"
                          href="reset"
                          asChild
                          className="px-0 font-normal"
                        >
                          Lupa password?
                        </Button>
                      </div>
                    </FormItem>
                  )}
                />
              </>
            )}
          </div>
          <FormError message={error || urlError} />
          <FormSuccess message={success} />
          <Button
            disabled={isLoading}
            type="submit"
            className="w-full bg-blue-500 hover:bg-blue-400"
          >
            {showTwoFactor ? "Konfirmasi" : "Masuk"}
          </Button>
        </form>
      </Form>
    </CardWrapper>
  )
}

export default LoginForm
