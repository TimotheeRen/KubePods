import { ChartPie } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";

export default function DesktopsUsagePieChart() {
  return (
    <Card className="w-full h-full">
      <CardHeader>
        <CardTitle>Most used desktops</CardTitle>
        <CardDescription>Explore your most used desktops</CardDescription>
        <CardAction>
          <ChartPie />
        </CardAction>
      </CardHeader>
      <CardContent>

      </CardContent>
    </Card>
  )
}
