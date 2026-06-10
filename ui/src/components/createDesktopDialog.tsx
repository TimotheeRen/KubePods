import { Plus } from "lucide-react";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import { SidebarMenuButton } from "./ui/sidebar";
import { Field, FieldGroup } from "./ui/field";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Button } from "./ui/button";
import { useFetcher } from "react-router";
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "./ui/select";
import { useEffect, useState } from "react";
import { toast } from "sonner";

interface ActionData {
  error: string | null,
  message: string | null,
}

interface CreateDesktopDialogProps {
  type?: string,
}

export default function CreateDesktopDialog({ type }: CreateDesktopDialogProps) {
  const fetcher = useFetcher<ActionData>()
  const state = fetcher.data
  const pending = fetcher.state === "submitting"
  const [open, setOpen] = useState(false)

  useEffect(() => {
    if (state?.error) {
      toast.error(state.message)
    } else if (state?.message) {
      toast.success(state.message)
      setOpen(false)
    }
  }, [state])


  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        {type === "sidebar" ? (
          <SidebarMenuButton
            className="min-w-8 bg-primary text-primary-foreground duration-200 ease-linear hover:bg-primary/90 hover:text-primary-foreground active:bg-primary/90 active:text-primary-foreground cursor-pointer"
          >
            <div className="flex gap-1 items-center">
              <Plus />
              <span>Create a desktop</span>
            </div>
          </SidebarMenuButton>
        ) : type === "button-plus" ? (
          <Button><Plus />Add a desktop</Button>
        ) : (
          <Button>Add a desktop</Button>
        )}
      </DialogTrigger>
      <DialogContent>
        <fetcher.Form method="post" action="/actions/createDesktop">
          <DialogHeader>
            <DialogTitle>Create a desktop</DialogTitle>
            <DialogDescription>Add a desktop to your workspace</DialogDescription>
          </DialogHeader>
          <FieldGroup className="mt-3 mb-5">
            <Field>
              <Label>Desktop name</Label>
              <Input name="name" placeholder="Your desktop name" />
            </Field>
            <Field>
              <Label>Distribution</Label>
              <Select name="distribution" defaultValue="ubuntu">
                <SelectTrigger>
                  <SelectValue placeholder="Ubuntu" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="ubuntu">Ubuntu</SelectItem>
                    <SelectItem value="debian">Debian</SelectItem>
                    <SelectItem value="fedora">Fedora</SelectItem>
                    <SelectItem value="arch">Arch</SelectItem>
                    <SelectItem value="alpine">Alpine</SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </Field>
            <Field>
              <Label >Desktop environment</Label>
              <Select name="desktop_environment" defaultValue="kde">
                <SelectTrigger>
                  <SelectValue placeholder="KDE Plasma" />
                </SelectTrigger>
                <SelectContent>
                  <SelectGroup>
                    <SelectItem value="kde">KDE Plasma</SelectItem>
                    <SelectItem value="xfce">XFCE</SelectItem>
                    <SelectItem value="i3">i3</SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </Field>
          </FieldGroup>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit" disabled={pending}>Create</Button>
          </DialogFooter>
        </fetcher.Form>
      </DialogContent>
    </Dialog>
  )
}
