import Header from "@/components/header";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Field, FieldLabel } from "@/components/ui/field";
import { InputGroup, InputGroupAddon, InputGroupInput } from "@/components/ui/input-group";
import { Mail, User } from "lucide-react";

export default function Settings() {
  return (
    <div className="flex-1 flex flex-col">
      <Header title="Settings" />
      <div className="p-5">
        <h1 className="text-2xl font-bold">Account</h1>
        <p className="text-muted-foreground">Manage your account informations</p>
        <Card className="mt-5">
          <CardContent>
            <Field>
              <FieldLabel>Email address</FieldLabel>
              <InputGroup>
                <InputGroupInput defaultValue="xxxx@example.com" />
                <InputGroupAddon align="inline-end">
                  <Mail />
                </InputGroupAddon>
              </InputGroup>
            </Field>
            <Field className="mt-5">
              <FieldLabel>Username</FieldLabel>
              <InputGroup>
                <InputGroupInput defaultValue="example" />
                <InputGroupAddon align="inline-end">
                  <User />
                </InputGroupAddon>
              </InputGroup>
            </Field>
            <Button className="mt-5" type="submit">Save</Button>
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
