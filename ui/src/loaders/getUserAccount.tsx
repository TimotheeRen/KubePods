import Cookie from "js-cookie";

export async function GetUserAccount() {
  const host = import.meta.env.VITE_API_HOST;
  const token = Cookie.get("token")

  const response = await fetch(host + "/users/get_user_account", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  return await response.json()
} 
