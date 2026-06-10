import Header from "@/components/header"
import DesktopsList from "@/components/desktopsList"
import DesktopsUsageLineChart from "@/components/desktopsUsageLineChart"
import DesktopsUsagePieChart from "@/components/desktopsUsagePieChart"

export default function Dashboard() {
  return (
    <div className="flex-1 flex flex-col">
      <Header title={"Dashboard"} />
      <div className="p-5 flex flex-col gap-3 justify-between h-full w-full">
        <DesktopsList />
        <div className="flex justify-between gap-3 flex-1">
          <DesktopsUsageLineChart />
          <DesktopsUsagePieChart />
        </div>
      </div>
    </div>
  )
}
