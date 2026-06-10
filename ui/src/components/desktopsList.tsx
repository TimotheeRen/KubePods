import { MonitorPlay } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";

export default function DesktopList() {
  return (
    <Card className="w-full h-full">
      <CardHeader>
        <CardTitle>Your desktops</CardTitle>
        <CardDescription>Manage your desktops</CardDescription>
        <CardAction>
          <MonitorPlay />
        </CardAction>
      </CardHeader>
      <CardContent>

      </CardContent>
    </Card>
  )
}
