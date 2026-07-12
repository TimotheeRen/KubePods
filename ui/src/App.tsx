import { createBrowserRouter, RouterProvider } from "react-router-dom"
import { redirect } from "react-router";
import Root from "@/routes/index"
import { Toaster } from "sonner"
import { login } from "./actions/login"
import { register } from "./actions/register"
import Auth from "./middlewares/auth"
import Dashboard from "./routes/dashboard.tsx"
import { ThemeProvider } from "./components/theme-provider.tsx"
import DashboardLayout from "./components/dashboardLayout.tsx"
import { createDesktop } from "./actions/createDesktop.tsx";
import NotFound from "./components/ui/not-found.tsx";
import { GetDesktops } from "./loaders/getDesktops.tsx";
import { GetUtilization } from "./loaders/getUtilization.tsx";
import { GetTimeRemaining } from "./loaders/getTimeRemaining.tsx";
import Settings from "./routes/settings.tsx";
import { GetUserAccount } from "./loaders/getUserAccount.tsx";

const router = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
    loader: async () => {
      if (await Auth()) {
        throw redirect("/dashboard")
      }
      return null
    }
  },
  {
    path: '/dashboard',
    loader: async () => {
      if (!await Auth()) {
        throw redirect("/")
      }
      return null
    },
    element: <DashboardLayout />,
    children: [
      {
        path: '',
        element: <Dashboard />,
        loader: async () => {
          const [desktops, utilization, timeRemaining] = await Promise.all([
            GetDesktops(),
            GetUtilization(),
            GetTimeRemaining()
          ])
          return { desktops, utilization, timeRemaining }
        },
      },
      {
        path: 'settings',
        element: <Settings />,
        loader: GetUserAccount
      }
    ],
  },
  {
    path: '/actions',
    children: [
      {
        path: 'login',
        action: login
      },
      {
        path: 'register',
        action: register
      },
      {
        path: 'createDesktop',
        action: createDesktop
      },
    ]
  },
  {
    path: "*",
    element: <NotFound />
  }
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
