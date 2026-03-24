interface State {
  isOpenStockForm: boolean
  isOpenStockTransactionDetail: boolean
  data: {
    id?: string
    productId?: string
  }
}

export const initialState: State = {
  isOpenStockForm: false,
  isOpenStockTransactionDetail: false,
  data: {
    id: "",
    productId: "",
  },
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const stockReducer = (state = initialState, action: any): State => {
  switch (action.type) {
    case "SET_OPEN_STOCK_FORM":
      return {
        ...state,
        isOpenStockForm: action.data,
      }
    case "SET_OPEN_STOCK_TRANSACTION_DETAIL":
      console.log("tes reducer", action.data)
      return {
        ...state,
        isOpenStockTransactionDetail: action.data,
      }
    case "SET_DATA_STOCK":
      return {
        ...state,
        data: action.data,
      }
    default:
      return state
  }
}
