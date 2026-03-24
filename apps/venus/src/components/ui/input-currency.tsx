import { formatCurrency, parseNumber } from "@/lib/utils"
import { Input, type InputProps } from "./input"

export default function InputCurrency(
  props: Omit<InputProps, "type" | "autoComplete"> &
    React.RefAttributes<HTMLInputElement>
) {
  return (
    <Input
      {...props}
      type="text"
      autoComplete="off"
      value={formatCurrency(props.value) || "0"}
      onChange={(e) => {
        const numericValue = parseNumber(e.target.value)
        e.target.value = formatCurrency(numericValue)
        props.onChange?.({
          ...e,
          target: {
            ...e.target,
            value: numericValue.toString(),
          },
        })
      }}
      onFocus={(e) => {
        if (props.value === 0 || props.value === "0") {
          e.target.value = ""
        }
        props.onFocus?.(e)
      }}
      onBlur={(e) => {
        if (e.target.value === "") {
          e.target.value = "0"
        }
        props.onBlur?.(e)
      }}
    />
  )
}
