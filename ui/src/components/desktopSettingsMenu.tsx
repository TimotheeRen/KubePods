import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "./ui/dropdown-menu";
import { Button } from "./ui/button";
import { ExternalLink, Settings, Trash2 } from "lucide-react";
import { Link } from "react-router";
import DestructiveAlert from "./destructiveAlert"
import { useState } from "react";

interface DesktopSettingsMenuProps {
  link: string
}

export default function DesktopSettingsMenu({ link }: DesktopSettingsMenuProps) {
  const [open, setOpen] = useState(false);
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
      <DestructiveAlert title="Delete desktop ?" description="This will permanatly delete the current desktop." isOpen={open} onOpenChange={setOpen} />
    </div>
  )
}
