import ProtectedRoute from "@/components/auth/protected-route"
import Loader from "@/components/loader"
import { Toaster } from "@/components/ui/sonner"
import { Suspense } from "react"
import { Route, Routes } from "react-router"
import { NotFoundRoute, PrivateRoute, PublicRoute } from "./routes"

export default function AppRoutes() {
  return (
    <Suspense fallback={<Loader />}>
      <Routes>
        {[...PublicRoute, ...PrivateRoute].map(
          ({ path: path1, element: element1, children: children1 }, i1) => (
            <Route
              key={i1}
              path={path1}
              element={<ProtectedRoute>{element1!}</ProtectedRoute>}
            >
              {children1?.map(
                (
                  { path: path2, element: element2, children: children2 },
                  i2
                ) => (
                  <Route key={i2} path={path2} element={element2}>
                    {children2?.map(
                      (
                        { path: path3, element: element3, children: children3 },
                        i3
                      ) => (
                        <Route key={i3} path={path3} element={element3}>
                          {children3?.map(
                            ({ path: path4, element: element4 }, i4) => (
                              <Route key={i4} path={path4} element={element4} />
                            )
                          )}
                        </Route>
                      )
                    )}
                  </Route>
                )
              )}
            </Route>
          )
        )}

        {/* Not Found */}
        <Route path={NotFoundRoute.path} element={NotFoundRoute.element} />
      </Routes>
      <Toaster />
    </Suspense>
  )
}
