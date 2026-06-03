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
import { useFetcher } from "react-router"
import { toast } from "sonner"
import { ShimmerButton } from "./ui/shimmer-button"

interface ActionData {
  error: string | null
  message: string
  token?: string
}

export default function RegisterDialog() {
  const fetcher = useFetcher<ActionData>()
  const state = fetcher.data
  const pending = fetcher.state === "submitting"

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
        <ShimmerButton background="rgba(99, 102, 241, 1)" className="h-10 w-20">Register</ShimmerButton>
      </DialogTrigger>
      <DialogContent className="sm:max-w-sm">
        <fetcher.Form method="post" action="/action/register">
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
            <Button type="submit" disabled={pending}>Register</Button>
          </DialogFooter>
        </fetcher.Form>
      </DialogContent>
    </Dialog>
  )
}
