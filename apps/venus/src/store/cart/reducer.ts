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

const initialState: {
  cart: OrderItem[]
  totalAmount: number
  isUpdate: boolean
  transactionId?: string
  openCheckout?: boolean
  openDraft?: boolean
  openResult?: boolean
} = {
  cart: [],
  totalAmount: 0,
  isUpdate: false,
  transactionId: undefined,
  openCheckout: false,
  openDraft: false,
  openResult: false,
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const cartReducer = (state = initialState, action: any) => {
  switch (action.type) {
    case SET_TOTAL: {
      const payload: number = action.data
      return {
        ...state,
        totalAmount: payload,
      }
    }
    case ADD_ITEM: {
      const payload: OrderItem = action.data
      if (
        !state.cart.some(
          (item: OrderItem) => item.productId === payload.productId
        )
      ) {
        const newCartItem = {
          ...payload,
          quantity: 1,
          totalPrice: payload.price,
        }
        return {
          ...state,
          cart: [...state.cart, newCartItem],
          totalAmount: state.totalAmount + newCartItem.price,
        }
      } else {
        const updatedCart = state.cart.map((item) =>
          item.productId === payload.productId
            ? {
                ...item,
                quantity: item.quantity + 1,
                totalPrice: (item.totalPrice ?? 0) + payload.price,
              }
            : item
        )
        return {
          ...state,
          cart: updatedCart,
          totalAmount: updatedCart.reduce(
            (acc, item) => acc + (item.totalPrice ?? 0),
            0
          ),
        }
      }
    }
    case REMOVE_ITEM: {
      const payload: OrderItem = action.data
      const updatedCart = state.cart.map((item) =>
        item.productId === payload.productId
          ? {
              ...item,
              quantity: item.quantity - 1,
              totalPrice: (item.totalPrice ?? 0) - payload.price,
            }
          : item
      )

      const filteredCart = updatedCart.filter((item) => item.quantity > 0)

      const totalAmount = filteredCart.reduce(
        (acc, item) => acc + (item.totalPrice ?? 0),
        0
      )

      return {
        ...state,
        cart: filteredCart,
        totalAmount: totalAmount,
      }
    }
    case RESET_CHART: {
      return {
        ...state,
        cart: [],
        totalAmount: 0,
        isUpdate: false,
        transactionId: undefined,
        openCheckout: false,
        openDraft: false,
        openResult: false,
      }
    }
    case UPDATE_ITEMS: {
      const payload: {
        transactionId: string
        prevItems: OrderItem[]
      } = action.data
      console.log("payload: ", payload)
      return {
        ...state,
        cart: payload.prevItems,
        totalAmount: payload.prevItems.reduce(
          (total, item) => total + (item.totalPrice ?? 0),
          0
        ),
        transactionId: payload.transactionId,
      }
    }
    case SET_IS_UPDATE: {
      const payload: boolean = action.data
      return {
        ...state,
        isUpdate: payload,
      }
    }
    case SET_OPEN_CHECKOUT: {
      const payload: boolean = action.data
      return {
        ...state,
        openCheckout: payload,
      }
    }
    case SET_OPEN_DRAFT: {
      const payload: boolean = action.data
      return {
        ...state,
        openDraft: payload,
      }
    }
    case SET_OPEN_RESULT: {
      const payload: boolean = action.data
      return {
        ...state,
        openResult: payload,
      }
    }
    default:
      return state
  }
}
