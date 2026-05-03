"use server"

import z  from "zod";

const User = z.object({
  email: z.email(),
  username: z.string().min(3).max(24).regex(/^[a-zA-Z0-9]+$/, {
    message: "The username can only contain letters and numbers."
  }),
  password: z.string().min(8).max(128),
})

export async function register(prevState: any, form: FormData) {
 const host = process.env.API_HOST || "http://localhost:3001"
 const result = User.safeParse(Object.fromEntries(form.entries()))

  if (!result.success) {
    return {
      error: "Validation",
      message: result.error.issues[0].message
    }
  } else {
    const { email, username, password } = result.data
    try {
      const response = await fetch(host+"/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({email, username, password}),
      })

      if (!response.ok) {
        return {
          error: "Fetching",
          message: "Couldn't join host."
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
