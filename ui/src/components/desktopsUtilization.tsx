import { Timer } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";

export default function DesktopsUtilization() {
  return (
    <Card className="w-full h-full">
      <CardHeader>
        <CardTitle>Time spent</CardTitle>
        <CardDescription>Explore your remaining time</CardDescription>
        <CardAction>
          <Timer />
        </CardAction>
      </CardHeader>
      <CardContent>

      </CardContent>
    </Card>
  )
}
