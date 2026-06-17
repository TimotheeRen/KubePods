import { type ActionFunctionArgs } from "react-router-dom";
import z from "zod";
import Cookie from "js-cookie";

const Desktop = z.object({
  name: z.string(),
  distribution: z.string(),
  desktop_environment: z.string(),
})

export async function createDesktop({ request }: ActionFunctionArgs) {
  const host = import.meta.env.VITE_API_HOST;
  console.log(host)
  const formData = await request.formData()
  const result = Desktop.safeParse(Object.fromEntries(formData.entries()))
  const token = Cookie.get("token")

  if (!result.success) {
    return {
      error: "Validation",
      message: result.error.issues[0].message
    }
  } else {
    const { name, distribution, desktop_environment } = result.data
    console.log(name, distribution, desktop_environment)
    try {
      const response = await fetch(host + "/desktops/create_desktop", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({ name, distribution, desktop_environment }),
      })

      if (!response.ok) {
        if (response.status == 409) {
          return {
            error: "Conflict",
            message: "Desktop already created"
          }
        } else {
          return {
            error: "Fetching",
            message: "An unexpected error occured"
          }
        }
      }

      return ({
        error: null,
        message: "Desktop successfully created !",
      })
    } catch (e) {
      return {
        error: "Server",
        message: "Network error or host unreachable."
      }
    }
  }
}
