import { redirect } from "react-router";
import Cookie from "js-cookie";

export default async function Auth() {
  const token = Cookie.get("token")
  if (!token) {
    throw redirect("/")
  }

  const host = import.meta.env.VITE_API_HOST;
  const response = await fetch(host + "/ping", {
    headers: {
      Authorization: `Bearer ${token}`,
    }
  })
  if (response.status != 200) {
    throw redirect("/")
  }

  return null
}
