import { Timer } from "lucide-react";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { ChartContainer, type ChartConfig } from "./ui/chart";
import { PolarGrid, PolarRadiusAxis, RadialBar, RadialBarChart, Label } from "recharts";
import { useLoaderData } from "react-router";

const chartConfig = {
  desktops: {
    label: "Hours",
  },
} satisfies ChartConfig

interface timeRemainingInterface {
  utilization: number,
  remaining: number,
}

export default function DesktopsUtilization() {
  const { timeRemaining } = useLoaderData() as { timeRemaining: timeRemainingInterface };
  const chartData = [
    { label: String(timeRemaining.utilization) + " Hours", count: 1, fill: "var(--color-primary)" },
  ]
  let angle = timeRemaining.utilization / timeRemaining.remaining * 360
  return (
    <Card className="w-full h-full">
      <CardHeader>
        <CardTitle>Time spent</CardTitle>
        <CardDescription>Explore your time available</CardDescription>
        <CardAction>
          <Timer />
        </CardAction>
      </CardHeader>
      <CardContent>
        <ChartContainer
          config={chartConfig}
          className="mx-auto aspect-square max-h-62.5"
        >
          <RadialBarChart
            data={chartData}
            endAngle={angle}
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
                          className="fill-foreground text-2xl font-bold"
                        >
                          {chartData[0].label.toLocaleString()}
                        </tspan>
                        <tspan
                          x={viewBox.cx}
                          y={(viewBox.cy || 0) + 24}
                          className="fill-muted-foreground"
                        >
                          Running desktops
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

