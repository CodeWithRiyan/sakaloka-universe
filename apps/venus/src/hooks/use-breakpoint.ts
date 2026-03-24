import { useEffect, useState } from "react"
import { useMediaQuery } from "react-responsive"

export default function useBreakpoint() {
  const [isClient, setIsClient] = useState(false)

  // This effect runs only on the client side
  useEffect(() => {
    setIsClient(true)
  }, [])

  // Define media queries for different breakpoints
  // These queries can be adjusted based on your design requirements
  // sm: 640px, md: 768px, lg: 1024px, xl: 1280px, xxl: 2080px
  // You can also use custom breakpoints as per your design system
  const sm = useMediaQuery({ query: "(min-width: 640px)" })
  const md = useMediaQuery({ query: "(min-width: 768px)" })
  const lg = useMediaQuery({ query: "(min-width: 1024px)" })
  const xl = useMediaQuery({ query: "(min-width: 1280px)" })
  const xxl = useMediaQuery({ query: "(min-width: 2080px)" })

  // If not client-side, return default values
  // This prevents hydration errors in Next.js
  // when the media queries are evaluated on the server
  // and the client-side media queries are different.
  if (!isClient) {
    return { sm: false, md: false, lg: false, xl: false, xxl: false }
  }

  return { sm, md, lg, xl, xxl }
}
