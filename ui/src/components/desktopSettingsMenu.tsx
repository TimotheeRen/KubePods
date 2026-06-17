import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "./ui/dropdown-menu";
import { Button } from "./ui/button";
import { ExternalLink, Settings, Trash2 } from "lucide-react";
import { Link, useRevalidator } from "react-router";
import DestructiveAlert from "./destructiveAlert"
import { useState } from "react";
import DeleteDesktop from "@/actions/deleteDesktop"

interface DesktopSettingsMenuProps {
  link: string,
  resourceName: string,
}

export default function DesktopSettingsMenu({ link, resourceName }: DesktopSettingsMenuProps) {
  const revalidator = useRevalidator()
  const [open, setOpen] = useState(false)

  const handleDelete = async () => {
    await DeleteDesktop(resourceName)
    revalidator.revalidate()
  }
  return (
    <div>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button><Settings /></Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem>
            <ExternalLink />
            <Link target="_blank" to={link}>
              Open the desktop
            </Link>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem variant="destructive" onClick={(e) => {
            e.preventDefault()
            setOpen(true)
          }}>
            <Trash2 />
            Delete
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <DestructiveAlert title="Delete desktop ?" description="This will permanatly delete the current desktop." isOpen={open} onOpenChange={setOpen} execute={handleDelete} resourceName={resourceName} />
    </div>
  )
}
