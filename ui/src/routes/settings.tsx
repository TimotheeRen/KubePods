import Header from "@/components/header";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Field, FieldLabel } from "@/components/ui/field";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { Mail, User } from "lucide-react";
import { useEffect } from "react";
import { useFetcher, useLoaderData } from "react-router";
import { toast } from "sonner";

interface User {
  email: string,
  username: string
}

interface ActionData {
  error: string,
}

export default function Settings() {
  const { email, username } = useLoaderData() as User;
  const fetcher = useFetcher<ActionData>()
  const state = fetcher.data
  const pending = fetcher.state === "submitting"

  useEffect(() => {
    if (fetcher.state !== "idle" || !state) return
    if (state.error) {
      toast.error(state.error)
    } else {
      toast.success("Settings saved!")
    }
  }, [state, fetcher.state])

  return (
    <div className="flex-1 flex flex-col">
      <Header title="Settings" />
      <div className="p-5">
        <h1 className="text-2xl font-bold">Account</h1>
        <p className="text-muted-foreground">Manage your account informations</p>
        <Card className="mt-5">
          <CardContent>
            <fetcher.Form method="post" action="/actions/saveSettings">
              <Field>
                <FieldLabel>Email address</FieldLabel>
                <InputGroup>
                  <InputGroupInput name="email" defaultValue={email} />
                  <InputGroupAddon align="inline-end">
                    <Mail />
                  </InputGroupAddon>
                </InputGroup>
              </Field>
              <Field className="mt-5">
                <FieldLabel>Username</FieldLabel>
                <InputGroup>
                  <InputGroupInput name="username" defaultValue={username} />
                  <InputGroupAddon align="inline-end">
                    <User />
                  </InputGroupAddon>
                </InputGroup>
              </Field>
              <Button className="mt-5" type="submit" disabled={pending}>Save</Button>
            </fetcher.Form>
          </CardContent>
        </Card>
        <h1 className="text-2xl font-bold mt-10 text-destructive">Danger zone</h1>
        <p className="text-muted-foreground">Here you can find risky actions</p>
        <Card className="mt-5">
          <CardContent className="flex sm:flex-row gap-2 flex-col">
            <Button variant="secondary">Change password</Button>
            <Button variant="destructive">Delete the account</Button>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
