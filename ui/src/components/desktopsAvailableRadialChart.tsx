import { Binoculars } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { ChartContainer, type ChartConfig } from "./ui/chart";
import { PolarGrid, PolarRadiusAxis, RadialBar, RadialBarChart, Label } from "recharts";


const chartData = [
  { label: "3", count: 1, fill: "var(--color-primary)" },
]
const chartConfig = {
  desktops: {
    label: "Desktops",
  },
} satisfies ChartConfig


export default function desktopsavAilableRadialchart() {
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
        <ChartContainer
          config={chartConfig}
          className="mx-auto aspect-square max-h-62.5"
        >
          <RadialBarChart
            data={chartData}
            endAngle={100}
            innerRadius={65}
            outerRadius={95}
          >
            <PolarGrid
              gridType="circle"
              radialLines={false}
              stroke="none"
              className="first:fill-muted last:fill-background"
              polarRadius={[86, 74]}
            />
            <RadialBar dataKey="count" background />
            <PolarRadiusAxis tick={false} tickLine={false} axisLine={false}>
              <Label
                content={({ viewBox }) => {
                  if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                    return (
                      <text
                        x={viewBox.cx}
                        y={viewBox.cy}
                        textAnchor="middle"
                        dominantBaseline="middle"
                        fontSize={10}
                      >
                        <tspan
                          x={viewBox.cx}
                          y={viewBox.cy}
                          className="fill-foreground text-4xl font-bold"
                        >
                          {chartData[0].label.toLocaleString()}
                        </tspan>
                        <tspan
                          x={viewBox.cx}
                          y={(viewBox.cy || 0) + 24}
                          className="fill-muted-foreground"
                        >
                          Desktops created
                        </tspan>
                      </text>
                    )
                  }
                }}
              />
            </PolarRadiusAxis>
          </RadialBarChart>
        </ChartContainer>
      </CardContent>
    </Card>
  )
}
