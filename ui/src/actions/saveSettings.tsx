import { type ActionFunctionArgs } from "react-router-dom";
import z from "zod";
import Cookie from "js-cookie";
import { redirect } from "react-router";
import { jwtDecode } from "jwt-decode";

const User = z.object({
  email: z.string(),
  username: z.string(),
})

export async function saveSettings({ request }: ActionFunctionArgs) {
  const host = import.meta.env.VITE_API_HOST;
  const formData = await request.formData()
  const result = User.safeParse(Object.fromEntries(formData.entries()))
  const token = Cookie.get("token")

  if (!result.success) {
    return {
      error: "Received an error",
    }
  } else {
    let { email, username } = result.data
    username = username.toLowerCase()
    try {
      const response = await fetch(host + "/users/save_settings", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify({ email, username }),
      })

      if (!response.ok) {
        return {
          error: "An unexpected error occured",
        }
      }

      if (token) {
        const old_username = jwtDecode(token).sub
        if (old_username != username) {
          Cookie.remove("token")
          redirect("/")
        }
      }

      return ({
        error: null,
      })
    } catch (e) {
      return {
        error: "Network error or host unreachable."
      }
    }
  }
}

