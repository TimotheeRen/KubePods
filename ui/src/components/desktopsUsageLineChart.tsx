import { ChartLine } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";

export default function DesktopsUsageLineChart() {
  return (
    <Card className="w-full h-full">
      <CardHeader>
        <CardTitle>Desktops usage</CardTitle>
        <CardDescription>Explore your desktops usage</CardDescription>
        <CardAction>
          <ChartLine />
        </CardAction>
      </CardHeader>
      <CardContent>

      </CardContent>
    </Card>
  )
}
