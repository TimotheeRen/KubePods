import Header from "@/components/header"
import DesktopsList from "@/components/desktopsList"
import Desktopsavailableradialchart from "@/components/desktopsAvailableRadialChart"
import DesktopsUtilization from "@/components/desktopsUtilization"

export default function Dashboard() {
  return (
    <div className="flex-1 flex flex-col">
      <Header title={"Dashboard"} />
      <div className="p-5 flex flex-col gap-3 justify-between h-full w-full">
        <DesktopsList />
        <div className="flex justify-between gap-3 flex-1 flex-col md:flex-row">
          <Desktopsavailableradialchart />
          <DesktopsUtilization />
        </div>
      </div>
    </div>
  )
}
