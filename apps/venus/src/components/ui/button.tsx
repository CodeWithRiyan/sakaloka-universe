import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"
import * as React from "react"

import { cn } from "@/lib/utils"
import { AiOutlineLoading3Quarters } from "react-icons/ai"
import { Link } from "react-router"

// eslint-disable-next-line react-refresh/only-export-components
export const buttonVariants = cva(
  "inline-flex items-center justify-center gap-3 whitespace-nowrap rounded-md text-sm font-medium transition-colors cursor-pointer focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-primary text-primary-foreground shadow-sm hover:bg-primary/90",
        destructive:
          "bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90",
        outline:
          "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
        secondary:
          "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
        ghost: "hover:bg-accent hover:text-accent-foreground",
        link: "text-primary underline-offset-4 hover:underline",
        white: "bg-white text-primary shadow-sm hover:bg-blue-50",
        success:
          "bg-emerald-500 text-primary-foreground shadow-sm hover:bg-emerald-500/90",
      },
      size: {
        default: "h-9 px-4 py-2",
        xs: "h-7 rounded-md px-3 text-2xs",
        sm: "h-8 rounded-md px-3 text-xs",
        lg: "h-10 rounded-md px-8",
        xl: "h-12 rounded-md px-8",
        icon_sm: "size-8",
        icon_md: "size-9",
        icon_lg: "size-10",
        icon_xl: "size-12",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    Pick<
      React.AnchorHTMLAttributes<HTMLAnchorElement>,
      "href" | "target" | "rel" | "download"
    >,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean
  loading?: boolean
  isExternalLink?: boolean
}

const Button = React.forwardRef<
  HTMLButtonElement | HTMLAnchorElement,
  ButtonProps
>(
  (
    {
      className,
      style,
      variant,
      size,
      loading,
      asChild = false,
      href,
      target,
      rel,
      download,
      isExternalLink,
      ...props
    },
    ref
  ) => {
    if (href && !isExternalLink) {
      const anchorProps = props as React.AnchorHTMLAttributes<HTMLAnchorElement>
      return (
        <Link
          ref={ref as React.ForwardedRef<HTMLAnchorElement>}
          to={href}
          target={target}
          rel={rel}
          download={download}
          className={cn(buttonVariants({ variant, size, className }))}
          style={style}
          {...anchorProps}
        >
          {loading && <AiOutlineLoading3Quarters className="animate-spin" />}
          {props.children}
        </Link>
      )
    }

    if (href && isExternalLink) {
      const anchorProps = props as React.AnchorHTMLAttributes<HTMLAnchorElement>
      return (
        <a
          ref={ref as React.ForwardedRef<HTMLAnchorElement>}
          href={href}
          target={target}
          rel={rel}
          download={download}
          className={cn(buttonVariants({ variant, size, className }))}
          style={style}
          {...anchorProps}
        >
          {loading && <AiOutlineLoading3Quarters className="animate-spin" />}
          {props.children}
        </a>
      )
    }

    if (asChild) {
      return (
        <div
          className={cn(buttonVariants({ variant, size, className }))}
          style={style}
        >
          <Slot ref={ref} {...props}>
            {props.children}
          </Slot>
        </div>
      )
    }

    return (
      <button
        className={cn(buttonVariants({ variant, size, className }))}
        style={style}
        ref={ref as React.ForwardedRef<HTMLButtonElement>}
        {...props}
      >
        {loading && <AiOutlineLoading3Quarters className="animate-spin" />}
        {props.children}
      </button>
    )
  }
)
Button.displayName = "Button"

export { Button }
