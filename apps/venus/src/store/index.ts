import { configureStore } from "@reduxjs/toolkit"
import { rootMiddleware } from "./rootMiddleware"
import { rootReducer } from "./rootReducer"

export const store = configureStore({
  reducer: rootReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      immutableCheck: { warnAfter: 128 },
      serializableCheck: { warnAfter: 128 },
    }).concat(rootMiddleware),
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
