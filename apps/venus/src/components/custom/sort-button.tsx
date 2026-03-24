import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import { BiSort, BiSortDown, BiSortUp } from "react-icons/bi"

export interface OnShortProps {
  newSortBy: string
  currentSortBy: string | null
  currentSortOrder: string | null
}

export interface SortButtonProps {
  title: string
  keyName: string
  sortBy: string | null
  sortOrder: string | null
  onSort: (onShort: OnShortProps) => void
}

export function SortButton({
  title,
  keyName,
  sortBy,
  sortOrder,
  onSort,
}: SortButtonProps) {
  const isActive = keyName === sortBy

  const handleSort = () => {
    onSort({
      newSortBy: keyName,
      currentSortBy: sortBy,
      currentSortOrder: sortOrder,
    })
  }

  return (
    <Button
      variant="ghost"
      onClick={handleSort}
      className={cn(
        "hover:text-foreground w-full justify-between font-bold hover:bg-gray-200"
      )}
    >
      <span>{title}</span>
      {isActive ? (
        <>
          {sortOrder === "asc" ? (
            <BiSortUp className="ml-2 h-4 w-4" />
          ) : (
            <BiSortDown className="ml-2 h-4 w-4" />
          )}
        </>
      ) : (
        <BiSort className="ml-2 h-4 w-4" />
      )}
    </Button>
  )
}
