// components/ui/input-search.tsx
"use client"

import { Input, type InputProps } from "@/components/ui/input"
import { cn } from "@/lib/utils"
import { debounce } from "lodash"
import { Loader2, Search, X } from "lucide-react"
import React, { useCallback, useEffect, useMemo, useState } from "react"

interface InputSearchProps extends Omit<InputProps, "onChange" | "value"> {
  value?: string
  onSearch: (value: string) => void
  onClear?: () => void
  debounceMs?: number
  showSearchIcon?: boolean
  showClearButton?: boolean
  isLoading?: boolean
  placeholder?: string
  className?: string
  containerClassName?: string
  // Lodash debounce options
  debounceOptions?: {
    leading?: boolean
    trailing?: boolean
    maxWait?: number
  }
}

export function InputSearch({
  value: controlledValue = "",
  onSearch,
  onClear,
  debounceMs = 500,
  showSearchIcon = true,
  showClearButton = true,
  isLoading = false,
  placeholder = "Cari...",
  className,
  containerClassName,
  debounceOptions = { leading: false, trailing: true },
  ...props
}: InputSearchProps) {
  const [internalValue, setInternalValue] = useState(controlledValue)

  // Sync with controlled value
  useEffect(() => {
    setInternalValue(controlledValue)
  }, [controlledValue])

  // Create debounced search function
  const debouncedSearch = useMemo(
    () =>
      debounce(
        (searchTerm: string) => {
          onSearch(searchTerm.trim())
        },
        debounceMs,
        debounceOptions
      ),
    [onSearch, debounceMs, debounceOptions]
  )

  // Cleanup debounced function on unmount
  useEffect(() => {
    return () => {
      debouncedSearch.cancel()
    }
  }, [debouncedSearch])

  // Handle input change
  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const newValue = e.target.value
      setInternalValue(newValue)
      debouncedSearch(newValue)
    },
    [debouncedSearch]
  )

  // Handle clear
  const handleClear = useCallback(() => {
    setInternalValue("")
    debouncedSearch.cancel() // Cancel any pending debounced calls
    debouncedSearch("") // Immediately trigger search with empty string
    onClear?.()
  }, [debouncedSearch, onClear])

  // Handle key events
  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent<HTMLInputElement>) => {
      if (e.key === "Escape" && showClearButton) {
        handleClear()
      }
      props.onKeyDown?.(e)
    },
    [handleClear, showClearButton, props]
  )

  return (
    <div className={cn("relative", containerClassName)}>
      {/* Search Icon */}
      {showSearchIcon && (
        <Search className="text-muted-foreground absolute top-1/2 left-3 z-10 h-4 w-4 -translate-y-1/2" />
      )}

      {/* Input */}
      <Input
        {...props}
        value={internalValue}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        placeholder={placeholder}
        className={cn(
          showSearchIcon && "pl-9",
          (showClearButton || isLoading) && "pr-12",
          className
        )}
        disabled={isLoading || props.disabled}
      />

      {/* Right side icons */}
      <div className="absolute top-1/2 right-3 flex -translate-y-1/2 items-center gap-1">
        {/* Loading Spinner */}
        {isLoading && (
          <Loader2 className="text-muted-foreground z-10 h-4 w-4 animate-spin" />
        )}

        {/* Clear Button */}
        {!isLoading && showClearButton && internalValue && (
          <button
            type="button"
            onClick={handleClear}
            className="text-muted-foreground hover:bg-muted hover:text-foreground flex h-4 w-4 items-center justify-center rounded-full transition-colors"
            disabled={props.disabled}
            aria-label="Clear search"
          >
            <X className="h-3 w-3" />
          </button>
        )}
      </div>
    </div>
  )
}
