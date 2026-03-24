"use client"

import { Toaster as Sonner, type ToasterProps } from "sonner"
import { useTheme } from "./theme-provider"

const Toaster = ({ ...props }: ToasterProps) => {
  const { theme = "system" } = useTheme()

  return (
    <Sonner
      theme={theme as ToasterProps["theme"]}
      className="toaster group"
      closeButton
      position="top-right"
      toastOptions={{
        unstyled: true,
        classNames: {
          closeButton:
            "absolute -top-2 -left-2 p-1 rounded-full border border-gray-50/10",
          toast:
            "relative flex gap-2 p-4 items-start rounded-lg shadow-lg border border-gray-50/10",
          title: "font-semibold text-sm",
          description: "mt-1 text-xs",
        },
      }}
      richColors
      style={
        {
          "--normal-bg": "var(--popover)",
          "--normal-text": "var(--popover-foreground)",
          "--normal-border": "var(--border)",
        } as React.CSSProperties
      }
      {...props}
    />
  )
}

export { Toaster }
