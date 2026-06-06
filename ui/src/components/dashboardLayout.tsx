import { Outlet } from "react-router"
import AppSidebar from "./sidebar"
import { SidebarInset, SidebarProvider } from "./ui/sidebar"

export default function DashboardLayout() {
  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <Outlet />
      </SidebarInset>
    </SidebarProvider>
  )
}
