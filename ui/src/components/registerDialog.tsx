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
import { useFetcher, useNavigate } from "react-router"
import { toast } from "sonner"
import { ShimmerButton } from "./ui/shimmer-button"
import Cookie from "js-cookie";

interface ActionData {
  error: string | null
  message: string
  token?: string
}

export default function RegisterDialog() {
  const navigate = useNavigate()
  const fetcher = useFetcher<ActionData>()
  const state = fetcher.data
  const pending = fetcher.state === "submitting"

  useEffect(() => {
    if (state?.error) {
      toast.error(state.message)
    } else if (state?.token) {
      toast.success(state.message)
      Cookie.set("token", state.token, {
        expires: 1,
        secure: true,
        sameSite: "strict",
      })
      navigate("/dashboard")
    }
  }, [state])

  return (
    <Dialog>
      <DialogTrigger asChild>
        <ShimmerButton background="rgba(99, 102, 241, 1)" className="h-10 w-20">Register</ShimmerButton>
      </DialogTrigger>
      <DialogContent className="sm:max-w-sm">
        <fetcher.Form method="post" action="/actions/register">
          <DialogHeader>
            <DialogTitle>Register</DialogTitle>
            <DialogDescription>
              Create your account
            </DialogDescription>
          </DialogHeader>
          <FieldGroup className="my-5">
            <Field>
              <Label>Email</Label>
              <Input id="email" name="email" type="email" placeholder="m@example.com" required />
            </Field>
            <Field>
              <Label>Username</Label>
              <Input id="username" name="username" type="text" placeholder="A great username" className="lowercase" required />
            </Field>
            <Field>
              <Label>Password</Label>
              <Input id="password" name="password" type="password" placeholder="A strong password" required />
            </Field>
          </FieldGroup>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit" disabled={pending}>Register</Button>
          </DialogFooter>
        </fetcher.Form>
      </DialogContent>
    </Dialog>
  )
}
