import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"

import {
  ChartLine,
  Logs,
  MonitorCloud,
  Settings,
} from "lucide-react"

import { FaDiscord, FaGithub } from "react-icons/fa"

import CreateDesktopDialog from "./createDesktopDialog"
import logo from "@/assets/logo.png"
import LogoutMenu from "./logoutMenu"
import { Link } from "react-router"

export default function AppSidebar() {
  return (
    <Sidebar variant="inset">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton>
              <a href="#" className="flex gap-1">
                <img src={logo} alt="Logo" className="w-7" />
                <span className="text-lg">KubePods</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <CreateDesktopDialog type="sidebar" />
              </SidebarMenuItem>
            </SidebarMenu>

            <SidebarMenu className="mt-2">
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <Link to="" className="flex gap-2 w-full h-full">
                    <MonitorCloud />
                    <span>Dashboard</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton>
                  <ChartLine />
                  <span>Analytics</span>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton>
                  <Logs />
                  <span>Logs</span>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton asChild>
                  <Link to="settings" className="flex gap-2 w-full h-full">
                    <Settings />
                    <span>Settings</span>
                  </Link>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup className="mt-auto">
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton>
                  <FaDiscord />
                  <span>Support</span>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton>
                  <a href="https://github.com/TimotheeRen/KubePods" className="flex gap-2 w-full h-full" target="_blank">
                    <FaGithub />
                    <span>Source code</span>
                  </a>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <LogoutMenu />
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar >
  )
}
