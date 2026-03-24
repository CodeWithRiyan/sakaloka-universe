import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

/**
 * Format a number into a string with the locale "id-ID".
 * If the number is zero, return an empty string.
 *
 * @param {string | number | readonly string[] | undefined} num - the number to be formatted
 * @returns {string} the formatted number as a string
 */
export const formatCurrency = (
  num: string | number | readonly string[] | undefined
) => {
  const number = Number(num)
  if (number === 0) return ""
  return number.toLocaleString("id-ID")
}

/**
 * Parse a string into a number, removing all non-numeric characters.
 * If the resulting string is empty, return 0.
 * @param {string} str - The string to parse.
 * @returns {number} - The parsed number.
 */
export const parseNumber = (str: string) => {
  const cleaned = str.replace(/\D/g, "")
  return cleaned === "" ? 0 : Number(cleaned)
}

export const formatIDR = (price = 0) => {
  const formatted = new Intl.NumberFormat("id", {
    style: "currency",
    currency: "IDR",
    maximumFractionDigits: 0,
  }).format(price)

  return formatted
}
