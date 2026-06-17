import Cookie from "js-cookie";

export default async function DeleteDesktop(name: string) {
  const host = import.meta.env.VITE_API_HOST;
  const token = Cookie.get("token")

  await fetch(host + "/desktops/delete_desktop", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({ name }),
  })
}
