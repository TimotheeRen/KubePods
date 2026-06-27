import { AnimatedThemeToggler } from "./ui/animated-theme-toggler"
import { Separator } from "./ui/separator"
import { SidebarTrigger } from "./ui/sidebar"

interface HeaderProps {
  title: string
}

export default function Header({ title }: HeaderProps) {
  return (
    <header className="py-1 flex items-center border-b justify-between">
      <div className="flex items-center px-1">
        <SidebarTrigger />
        <Separator orientation="vertical" className="mr-2 data-[orientation=vertical]:h-4 my-auto" />
        <h1>{title}</h1>
      </div>
      <AnimatedThemeToggler className="-mt-1 mr-2" />
    </header>
  )
}
