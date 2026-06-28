import Cookie from "js-cookie";

export async function GetUtilization() {
  const host = import.meta.env.VITE_API_HOST;
  const token = Cookie.get("token")

  const response = await fetch(host + "/desktops/get_remaining_desktops", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  return await response.json()
} 
