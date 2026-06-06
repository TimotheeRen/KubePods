import { redirect } from "react-router";
import Cookie from "js-cookie";

export default async function Auth() {
  const token = Cookie.get("token")
  if (!token) return false

  const host = import.meta.env.VITE_API_HOST;
  try {
    const response = await fetch(host + "/ping", {
      headers: {
        Authorization: `Bearer ${token}`,
      }
    })
    if (response.status != 200) {
      throw redirect("/")
    }
  } catch (error) {
    return false
  }

  return true
}
