import { MonitorPlay, ScreenShareOff } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "./ui/empty";
import CreateDesktopDialog from "./createDesktopDialog";

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

        <Empty>
          <EmptyHeader>
            <EmptyMedia variant="icon">
              <ScreenShareOff />
            </EmptyMedia>
            <EmptyTitle>No desktops created</EmptyTitle>
            <EmptyDescription>
              Add a virtual desktop, accessible from anywhere
            </EmptyDescription>
          </EmptyHeader>
          <EmptyContent>
            <CreateDesktopDialog type="button-plus" />
          </EmptyContent>
        </Empty>

      </CardContent>
    </Card>
  )
}
