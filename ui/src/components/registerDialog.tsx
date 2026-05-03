"use client"

import { register } from "@/actions/register"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Field, FieldGroup } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { useActionState, useEffect, useTransition } from "react"
import { toast } from "sonner"

export default function RegisterDialog() {
  const [state, action, pending] = useActionState(register, null)

  useEffect(() => {
    if (state?.error) {
      toast.error(state.message)
    } else if (state?.message) {
      toast.success(state.message)
    }
  }, [state])

  return (
    <Dialog>
        <DialogTrigger asChild>
          <Button>Register</Button>
        </DialogTrigger>
        <DialogContent className="sm:max-w-sm">
          <form action={action}>
            <DialogHeader>
              <DialogTitle>Register</DialogTitle>
              <DialogDescription>
                Create your account
              </DialogDescription>
            </DialogHeader>
            <FieldGroup>
              <Field>
                <Label>Email</Label>
                <Input id="email" name="email" type="email" placeholder="m@example.com" />
              </Field>
              <Field>
                <Label>Username</Label>
                <Input id="username" name="username" type="text" placeholder="..." />
              </Field>
              <Field>
                <Label>Password</Label>
                <Input id="password" name="password" type="password" placeholder="..." />
              </Field>
            </FieldGroup>
            <DialogFooter>
              <DialogClose asChild>
                <Button variant="outline">Cancel</Button>
              </DialogClose>
              <Button type="submit">Register</Button>
            </DialogFooter>
          </form>
        </DialogContent>
    </Dialog>
  )
}
