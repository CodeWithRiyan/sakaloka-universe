"use client"

import * as React from "react"

import { cn } from "@/lib/utils"
import { PiEye, PiEyeSlash } from "react-icons/pi"

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  suffixIcon?: React.ReactNode
  prefixIcon?: React.ReactNode
  containerClassName?: string
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    { containerClassName, className, type, prefixIcon, suffixIcon, ...props },
    ref
  ) => {
    const [inputType, setInputType] =
      React.useState<React.HTMLInputTypeAttribute>(
        type === "password" ? "password" : "text"
      )

    const toggleInputType = () => {
      setInputType((prevType) =>
        prevType === "password" ? "text" : "password"
      )
    }

    return (
      <div
        className={cn("relative w-full", containerClassName)}
        style={{ marginTop: 0 }}
      >
        {prefixIcon && (
          <div className="absolute top-0 left-0 flex h-full w-11 items-center justify-center text-xl">
            {prefixIcon}
          </div>
        )}
        <input
          type={type === "password" ? inputType : type}
          className={cn(
            "border-input placeholder:text-muted-foreground focus-visible:ring-ring flex h-9 w-full rounded-lg border bg-transparent px-3 py-1 text-sm shadow-2xs transition-colors duration-300 file:border-0 file:bg-transparent file:text-sm file:font-medium hover:ring-1 hover:ring-blue-400 focus-visible:ring-1 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50",
            prefixIcon && "pl-11",
            className
          )}
          ref={ref}
          {...props}
        />
        {type === "password" && (
          <button
            type="button"
            className="absolute top-0 right-0 flex h-full w-11 items-center justify-center text-xl"
            onClick={toggleInputType}
          >
            {inputType === "password" ? <PiEye /> : <PiEyeSlash />}
          </button>
        )}
        {suffixIcon && (
          <div className="absolute top-0 right-0 flex h-full w-11 items-center justify-center text-xl">
            {suffixIcon}
          </div>
        )}
      </div>
    )
  }
)
Input.displayName = "Input"

export { Input }
