import { createBrowserRouter, RouterProvider } from "react-router-dom"
import Root from "@/routes/index"
import { Toaster } from "sonner"
import { login } from "./actions/login"
import { register } from "./actions/register"

const router = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
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
  }
])

function App() {
  return (
    <>
      <RouterProvider router={router} />
      <Toaster richColors />
    </>
  )
}

export default App
