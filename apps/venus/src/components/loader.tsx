import { cn } from "@/lib/utils"
import { useMediaQuery } from "react-responsive"
import { ScaleLoader } from "react-spinners"
import type { LoaderHeightWidthRadiusProps } from "react-spinners/helpers/props"

export function Loader({
  className,
  ...props
}: LoaderHeightWidthRadiusProps & { className?: string }) {
  const md = useMediaQuery({ query: "(min-width: 768px)" })

  return (
    <div
      className={cn(
        "flex h-screen w-screen items-center justify-center bg-blue-500 text-white",
        className
      )}
    >
      <ScaleLoader
        height={md ? 100 : 50}
        width={md ? 8 : 4}
        barCount={8}
        color="currentColor"
        {...props}
      />
    </div>
  )
}

export default Loader
