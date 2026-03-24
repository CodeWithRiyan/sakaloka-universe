import dayjs from "dayjs"
import id from "dayjs/locale/id"
import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import { HelmetProvider } from "react-helmet-async"
import { Provider } from "react-redux"
import { BrowserRouter } from "react-router"
import { ThemeProvider } from "./components/ui/theme-provider.tsx"
import "./index.css"
import AppRoutes from "./routes/app-routes.tsx"
import { store } from "./store"

dayjs.locale(id)

const context = {}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Provider store={store}>
      <HelmetProvider context={context}>
        <ThemeProvider>
          <BrowserRouter>
            <AppRoutes />
          </BrowserRouter>
        </ThemeProvider>
      </HelmetProvider>
    </Provider>
  </StrictMode>
)
