import { Plus } from "lucide-react";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import { SidebarMenuButton } from "./ui/sidebar";
import { Field, FieldGroup } from "./ui/field";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Button } from "./ui/button";
import { useFetcher } from "react-router";

interface ActionData {
  error: string | null,
}

export default function CreateDesktopDialog() {
  const fetcher = useFetcher<ActionData>()
  return (
    <Dialog>
      <DialogTrigger asChild>
        <SidebarMenuButton
          className="min-w-8 bg-primary text-primary-foreground duration-200 ease-linear hover:bg-primary/90 hover:text-primary-foreground active:bg-primary/90 active:text-primary-foreground cursor-pointer"
        >
          <div className="flex gap-1 items-center">
            <Plus />
            <span>Create a desktop</span>
          </div>
        </SidebarMenuButton>
      </DialogTrigger>
      <DialogContent>
        <fetcher.Form method="post" action="/action/createDesktop">
          <DialogHeader>
            <DialogTitle>Create a desktop</DialogTitle>
            <DialogDescription>Add a desktop to your workspace</DialogDescription>
          </DialogHeader>
          <FieldGroup className="mt-3 mb-5">
            <Field>
              <Label>Desktop name</Label>
              <Input placeholder="Your desktop name" />
            </Field>
          </FieldGroup>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit">Create</Button>
          </DialogFooter>
        </fetcher.Form>
      </DialogContent>
    </Dialog>
  )
}
