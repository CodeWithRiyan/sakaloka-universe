import { cn } from "@/lib/utils"
import { cva, type VariantProps } from "class-variance-authority"
import { capitalize } from "lodash"

const badgeVariants = cva("mt-1 flex-none size-2 rounded-full", {
  variants: {
    variant: {
      default: "bg-blue-500",
      success: "bg-green-500",
      warning: "bg-yellow-500",
      error: "bg-red-500",
    },
  },
  defaultVariants: {
    variant: "default",
  },
})

export interface ToastDescriptionListProps
  extends VariantProps<typeof badgeVariants> {
  list: string[]
}
export default function ToastDescriptionList({
  list,
  variant,
}: ToastDescriptionListProps) {
  return (
    <ul>
      {list.map((e, i) => (
        <li key={i} className="flex gap-2">
          <span className={cn(badgeVariants({ variant }))} />
          <span>{capitalize(e)}</span>
        </li>
      ))}
    </ul>
  )
}
