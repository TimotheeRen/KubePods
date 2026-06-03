import { createBrowserRouter, RouterProvider } from "react-router-dom"
import Root from "@/routes/index"
import { Toaster } from "sonner"
import { login } from "./actions/login"
import { register } from "./actions/register"
import Auth from "./middlewares/auth"
import Dashboard from "./routes/dashboard.tsx"
import { ThemeProvider } from "./components/theme-provider.tsx"

const router = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
  },
  {
    path: '/dashboard',
    loader: Auth,
    element: <Dashboard />
  },
  {
    path: '/action',
    children: [
      {
        path: 'login',
        action: login
      },
      {
        path: 'register',
        action: register
      },
    ]
  },
])

function App() {
  return (
    <>
      <ThemeProvider defaultTheme="dark" storageKey="theme">
        <RouterProvider router={router} />
      </ThemeProvider>
      <Toaster richColors />
    </>
  )
}

export default App
