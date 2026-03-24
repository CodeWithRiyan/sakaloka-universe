import type { OrderItem } from "@/types/order"
import {
  ADD_ITEM,
  REMOVE_ITEM,
  RESET_CHART,
  SET_IS_UPDATE,
  SET_OPEN_CHECKOUT,
  SET_OPEN_DRAFT,
  SET_OPEN_RESULT,
  SET_TOTAL,
  UPDATE_ITEMS,
} from "./constants"

export const total = (payload: number) => ({ type: SET_TOTAL, data: payload })
export const addItem = (payload: OrderItem) => ({
  type: ADD_ITEM,
  data: payload,
})
export const removeItem = (payload: OrderItem) => ({
  type: REMOVE_ITEM,
  data: payload,
})
export const resetChart = () => ({ type: RESET_CHART })
export const updateItems = (payload: {
  transactionId?: string
  prevItems: OrderItem[]
}) => {
  console.log("updateItems payload:", payload)
  return {
    type: UPDATE_ITEMS,
    data: payload,
  }
}
export const setIsUpdate = (payload: boolean) => ({
  type: SET_IS_UPDATE,
  data: payload,
})

export const setOpenCheckout = (payload: boolean) => ({
  type: SET_OPEN_CHECKOUT,
  data: payload,
})

export const setOpenDraft = (payload: boolean) => ({
  type: SET_OPEN_DRAFT,
  data: payload,
})

export const setOpenResult = (payload: boolean) => ({
  type: SET_OPEN_RESULT,
  data: payload,
})
