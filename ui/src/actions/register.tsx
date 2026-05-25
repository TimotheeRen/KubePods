"use server"

import type { ActionFunctionArgs } from "react-router-dom";
import z from "zod";

const User = z.object({
  email: z.email(),
  username: z.string().min(3).max(24).regex(/^[a-zA-Z0-9]+$/, {
    message: "The username can only contain letters and numbers."
  }),
  password: z.string().min(8).max(128),
})

export async function register({ request }: ActionFunctionArgs) {
  const host = "http://localhost:3001"
  const formData = await request.formData()
  const result = User.safeParse(Object.fromEntries(formData.entries()))

  if (!result.success) {
    return {
      error: "Validation",
      message: result.error.issues[0].message
    }
  } else {
    const { email, username, password } = result.data
    try {
      const response = await fetch(host + "/users/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, username, password }),
      })

      if (!response.ok) {
        if (response.status == 409) {
          return {
            error: "Fetching",
            message: "Username already taken"
          }
        } else if (response.status == 500) {
          return {
            error: "Fetching",
            message: "Internal server error"
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
        message: "Successfuly registered!"
      })
    } catch (e) {
      return {
        error: "Server",
        message: "Network error or host unreachable."
      }
    }
  }
}
