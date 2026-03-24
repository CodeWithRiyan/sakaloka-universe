"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { useState } from "react"
import { useForm } from "react-hook-form"
import * as z from "zod"

import { CardWrapper } from "@/components/auth/card-wrapper"
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
import { register } from "@/lib/auth"
import { useNavigate, useSearchParams } from "react-router"
import { toast } from "sonner"

const RegisterSchema = z
  .object({
    email: z.string().email({
      message: "Email tidak valid",
    }),
    fullName: z.string().min(1, {
      message: "Nama lengkap harus diisi",
    }),
    organizationName: z.string().min(1, {
      message: "Nama usaha harus diisi",
    }),
    password: z.string().min(6, {
      message: "Password minimal 6 karakter",
    }),
    passwordConfirmation: z.string().min(6, {
      message: "Konfirmasi password minimal 6 karakter",
    }),
  })
  .refine((data) => data.password === data.passwordConfirmation, {
    message: "Password dan konfirmasi password tidak sama",
    path: ["passwordConfirmation"],
  })

export const RegisterForm = () => {
  const [searchParams] = useSearchParams()
  const callbackUrl = searchParams.get("callbackUrl")
  const navigate = useNavigate()

  const [error, setError] = useState<string | undefined>("")
  const [success, setSuccess] = useState<string | undefined>("")
  const [isLoading, setIsLoading] = useState<boolean>()

  const form = useForm<z.infer<typeof RegisterSchema>>({
    resolver: zodResolver(RegisterSchema),
    defaultValues: {
      fullName: "",
      email: "",
      organizationName: "",
      password: "",
      passwordConfirmation: "",
    },
  })

  const onSubmit = async (values: z.infer<typeof RegisterSchema>) => {
    setError("")
    setSuccess("")

    setIsLoading(true)

    try {
      const result = await register(values)

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
    <CardWrapper headerLabel="REGISTRASI" pageInformation="register">
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <div className="space-y-4">
            <FormField
              control={form.control}
              name="fullName"
              required
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Nama Lengkap</FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      disabled={isLoading}
                      placeholder="John Doe"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="email"
              required
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Email</FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      disabled={isLoading}
                      placeholder="user@sakaloka.id"
                      type="email"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="organizationName"
              required
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Nama Usaha</FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      disabled={isLoading}
                      placeholder="Sakaloka"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              required
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
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="passwordConfirmation"
              required
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Konfirmasi Password</FormLabel>
                  <FormControl>
                    <Input
                      {...field}
                      disabled={isLoading}
                      placeholder="******"
                      type="password"
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <FormError message={error} />
          <FormSuccess message={success} />
          <Button
            disabled={isLoading}
            type="submit"
            className="w-full bg-blue-500 hover:bg-blue-400"
          >
            Buat Akun
          </Button>
        </form>
      </Form>
    </CardWrapper>
  )
}
