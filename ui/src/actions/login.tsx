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
        if (response.status == 401) {
          return {
            error: "Unauthenticated",
            message: "Wrong password"
          }
        } else {
          return {
            error: "Fetching",
            message: "An unexpected error occured"
          }
        }
      }

      let token = await response.text(); // TODO: put it in a cookie on the client
      return ({
        error: null,
        message: "Successfully logged in.",
        token: token,
      })
    } catch (e) {
      return {
        error: "Server",
        message: "Network error or host unreachable."
      }
    }
  }
}
