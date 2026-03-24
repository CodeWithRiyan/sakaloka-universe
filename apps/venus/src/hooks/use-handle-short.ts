import type { OnShortProps } from "@/components/custom/sort-button"
import { useSearchParams } from "react-router"

export function useHandleShort({
  setSortBy,
  setSortOrder,
}: {
  setSortBy: (val?: string) => void
  setSortOrder: (val?: string) => void
}) {
  const handleSort = ({
    newSortBy,
    currentSortBy,
    currentSortOrder,
  }: OnShortProps) => {
    const isActive = newSortBy === currentSortBy

    if (isActive) {
      if (currentSortOrder === "asc") {
        setSortBy()
        setSortOrder()
      } else if (currentSortOrder === "desc") {
        setSortOrder("asc")
      }
    } else {
      setSortBy(newSortBy)
      setSortOrder("desc")
    }
  }

  return handleSort
}

export function useHandleShortByUrl() {
  const [searchParams, setSearchParams] = useSearchParams()
  const allSearchParams = Object.fromEntries(searchParams.entries())

  const handleSort = ({
    newSortBy,
    currentSortBy,
    currentSortOrder,
  }: OnShortProps) => {
    const isActive = newSortBy === currentSortBy

    if (isActive) {
      const newParams: Record<string, string> = { ...allSearchParams }

      if (currentSortOrder === "asc") {
        delete newParams.sortBy
        delete newParams.sortOrder
      } else if (currentSortOrder === "desc") {
        newParams.sortOrder = "asc"
      }

      setSearchParams(newParams, { replace: true })
    } else {
      setSearchParams(
        {
          ...allSearchParams,
          sortBy: newSortBy,
          sortOrder: "desc",
        },
        {
          replace: true,
        }
      )
    }
  }

  return handleSort
}
