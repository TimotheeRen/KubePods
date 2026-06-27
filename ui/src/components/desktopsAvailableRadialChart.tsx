import { Binoculars, ChartLine } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";

export default function desktopsavailableradialchart() {
  return (
    <Card className="w-full h-full">
      <CardHeader>
        <CardTitle>Desktops created</CardTitle>
        <CardDescription>Explore your available desktops</CardDescription>
        <CardAction>
          <Binoculars />
        </CardAction>
      </CardHeader>
      <CardContent>

      </CardContent>
    </Card>
  )
}
