export const setOpenStockForm = (payload: boolean) => ({
  type: "SET_OPEN_STOCK_FORM",
  data: payload,
})

export const setDataStock = (payload: { productId?: string; id?: string }) => ({
  type: "SET_DATA_STOCK",
  data: payload,
})

export const setOpenStockTransactionDetail = (payload: boolean) => {
  return {
    type: "SET_OPEN_STOCK_TRANSACTION_DETAIL",
    data: payload,
  }
}
