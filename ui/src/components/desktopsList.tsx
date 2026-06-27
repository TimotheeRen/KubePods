import { MonitorPlay, ScreenShareOff } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle } from "./ui/empty";
import CreateDesktopDialog from "./createDesktopDialog";
import { useLoaderData } from "react-router";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table";
import Cookie from "js-cookie";
import { jwtDecode } from "jwt-decode";
import DesktopSettingsMenu from "./desktopSettingsMenu";

type Desktop = {
  name: string;
  distribution: string;
  desktop_environment: string;
};

export default function DesktopList() {
  const { desktops, utilization, timeRemaining } = useLoaderData() as { desktops: any, utilization: any, timeRemaining: any };
  console.log(utilization)
  console.log(timeRemaining)
  const isPresent = desktops.length === 0
  const token = Cookie.get("token")
  if (!token) { return null }
  const username = jwtDecode(token).sub

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

        {isPresent ? (
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
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Distribution</TableHead>
                <TableHead>Desktop environment</TableHead>
                <TableHead>State</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {desktops.map((d: Desktop) => (
                <TableRow key={d.name}>
                  <TableCell>{d.name}</TableCell>
                  <TableCell>{d.distribution}</TableCell>
                  <TableCell>{d.desktop_environment}</TableCell>
                  <TableCell>
                    <span>Running</span>
                  </TableCell>
                  <TableCell>
                    <DesktopSettingsMenu link={"http://" + username + "-" + d.name + ".kubepods.com:8080"} resourceName={d.name} /> </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}

      </CardContent>
    </Card>
  )
}
