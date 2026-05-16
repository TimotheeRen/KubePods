"use server"

import z from "zod";

const User = z.object({
  username: z.string(),
  password: z.string(),
})

export async function login(prevState: any, form: FormData) {
  const host = process.env.API_HOST || "http://localhost:3001"
  const result = User.safeParse(Object.fromEntries(form.entries()))

  if (!result.success) {
    return {
      error: "Validation",
      message: result.error.issues[0].message
    }
  } else {
    const { username, password } = result.data
    try {
      const response = await fetch(host + "/users/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ username, password }),
      })

      if (!response.ok) {
        if (response.status == 500) {
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
