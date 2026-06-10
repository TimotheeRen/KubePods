import Cookie from "js-cookie";

export async function GetDesktops() {
  const host = import.meta.env.VITE_API_HOST;
  const token = Cookie.get("token")

  const response = await fetch(host + "/desktops/get_desktops", {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })
  return ({
    response: await response.json()
  })
} 
