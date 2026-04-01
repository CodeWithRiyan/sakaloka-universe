import { combineReducers } from "@reduxjs/toolkit"
import { bomApi } from "./bom/api"
import { branchApi } from "./branch/api"
import { brandApi } from "./brand/api"
import { cartReducer } from "./cart/reducer"
import { categoriesApi } from "./categories/api"
import { orderApi } from "./order/api"
import { productApi } from "./product/api"
import { roleApi } from "./role/api"
import { stockApi } from "./stock/api"
import { stockReducer } from "./stock/reducer"
import { userApi } from "./user/api"
import { utilsReducer } from "./utils/reducer"

export const rootReducer = combineReducers({
  utils: utilsReducer,
  cart: cartReducer,
  stock: stockReducer,
  [bomApi.reducerPath]: bomApi.reducer,
  [branchApi.reducerPath]: branchApi.reducer,
  [brandApi.reducerPath]: brandApi.reducer,
  [userApi.reducerPath]: userApi.reducer,
  [roleApi.reducerPath]: roleApi.reducer,
  [categoriesApi.reducerPath]: categoriesApi.reducer,
  [productApi.reducerPath]: productApi.reducer,
  [orderApi.reducerPath]: orderApi.reducer,
  [stockApi.reducerPath]: stockApi.reducer,
})
