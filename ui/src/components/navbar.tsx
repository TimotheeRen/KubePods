import logo from "@/assets/logo.png"
import { AnimatedThemeToggler } from "./ui/animated-theme-toggler"
import { FaGithub } from "react-icons/fa";
import { FaDiscord } from "react-icons/fa";
import { ChevronDown } from "lucide-react";

export default function Navbar() {
  return (
    <div className="flex text-center justify-between items-center cursor-pointer mt-1">
      <div className="flex cursor-pointer items-center gap-2 ml-3 mt-1">
        <img src={logo} alt="KubePods" className="w-8" />
        <h1 className="text-xl font-semibold">KubePods</h1>
      </div>
      <div className="md:flex gap-10 hidden mt-1">
        <div className="flex items-center gap-1">
          <h1 className="cursor-pointer text-lg">Why KubePods</h1>
          <ChevronDown size={18} />
        </div>
        <div className="flex items-center gap-1">
          <h1 className="cursor-pointer text-lg">Desktops</h1>
          <ChevronDown size={18} />
        </div>
        <div className="flex items-center gap-1">
          <h1 className="cursor-pointer text-lg">Pricing</h1>
          <ChevronDown size={18} />
        </div>
        <h1 className="cursor-pointer text-lg">Documentation</h1>
      </div>
      <div className="flex gap-4 items-center mr-3">
        <a href="#"><FaDiscord size={25} className="cursor-pointer" /></a>
        <a href="https://github.com/TimotheeRen/KubePods"><FaGithub size={21} className="cursor-pointer" /></a>
        <AnimatedThemeToggler className="cursor-pointer" />
      </div>
    </div>
  )
}
