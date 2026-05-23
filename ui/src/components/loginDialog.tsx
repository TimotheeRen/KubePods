"use client"

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
import { useEffect } from "react"
import { toast } from "sonner"
import Cookie from "js-cookie";
import { useFetcher } from "react-router"

interface ActionData {
  error: string | null
  message: string
  token?: string
}

export default function LoginDialog() {
  const fetcher = useFetcher<ActionData>()
  const state = fetcher.data
  const pending = fetcher.state === "submitting"

  useEffect(() => {
    if (state?.error) {
      toast.error(state.message)
    } else if (state?.message && state?.token) {
      toast.success(state.message)
      Cookie.set("token", state.token, {
        expires: 1,
        secure: true,
        sameSite: "strict",
      })
    }
  }, [state])

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Login</Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-sm">
        <fetcher.Form method="post" action="/action/login">
          <DialogHeader>
            <DialogTitle>Login</DialogTitle>
            <DialogDescription>
              Log into your account
            </DialogDescription>
          </DialogHeader>
          <FieldGroup className="my-5">
            <Field>
              <Label>Username</Label>
              <Input id="username" name="username" type="text" placeholder="..." required />
            </Field>
            <Field>
              <Label>Password</Label>
              <Input id="password" name="password" type="password" placeholder="..." required />
            </Field>
          </FieldGroup>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit" disabled={pending}>Login</Button>
          </DialogFooter>
        </fetcher.Form>
      </DialogContent>
    </Dialog >
  )
}
