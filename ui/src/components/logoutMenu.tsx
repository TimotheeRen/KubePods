import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogTitle, AlertDialogTrigger } from "./ui/alert-dialog";
import { LogOut } from "lucide-react";
import { SidebarMenuButton } from "@/components/ui/sidebar"
import Cookie from "js-cookie";
import { useNavigate } from "react-router-dom"
import { toast } from "sonner"

export default function LogoutMenu() {
  const navigate = useNavigate()

  const logout = () => {
    Cookie.remove("token")
    toast.success("Successfully logout.")
    navigate("/")
  }

  return (
    <AlertDialog>
      <AlertDialogTrigger>
        <SidebarMenuButton className="text-destructive">
          <LogOut />
          <span>Logout</span>
        </SidebarMenuButton>
      </AlertDialogTrigger>
      <AlertDialogContent size="sm">
        <AlertDialogHeader>
          <AlertDialogMedia className="bg-destructive/10 text-destructive dark:bg-destructive/20 dark:text-destructive">
            <LogOut />
          </AlertDialogMedia>
          <AlertDialogTitle>Logout</AlertDialogTitle>
          <AlertDialogDescription>Are you sure you want to logout ?</AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel variant="outline">Cancel</AlertDialogCancel>
          <AlertDialogAction variant="destructive" onClick={logout}>Logout</AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
