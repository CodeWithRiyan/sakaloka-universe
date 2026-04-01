import { bomApi } from "./bom/api"
import { branchApi } from "./branch/api"
import { brandApi } from "./brand/api"
import { categoriesApi } from "./categories/api"
import { orderApi } from "./order/api"
import { productApi } from "./product/api"
import { roleApi } from "./role/api"
import { stockApi } from "./stock/api"
import { userApi } from "./user/api"

export const rootMiddleware = [
  bomApi.middleware,
  branchApi.middleware,
  brandApi.middleware,
  categoriesApi.middleware,
  orderApi.middleware,
  stockApi.middleware,
  productApi.middleware,
  roleApi.middleware,
  userApi.middleware,
]
