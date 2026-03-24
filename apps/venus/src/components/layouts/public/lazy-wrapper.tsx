import { Suspense, type ReactNode } from "react"

// Loading component
const PageLoader = () => (
  <div className="flex min-h-screen items-center justify-center">
    <div className="border-primary h-8 w-8 animate-spin rounded-full border-b-2"></div>
  </div>
)

// Wrapper component for lazy loading
export const LazyWrapper = ({ children }: { children: ReactNode }) => (
  <Suspense fallback={<PageLoader />}>{children}</Suspense>
)

export default LazyWrapper
