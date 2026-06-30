import { OrbitingCircles } from "./ui/orbiting-circles"
import alpine from "@/assets/alpine-logo.svg"
import arch from "@/assets/arch-logo.svg"
import fedora from "@/assets/fedora-logo.svg"
import ubuntu from "@/assets/ubuntu-logo.svg"
import debian from "@/assets/debian-logo.svg"
import kde from "@/assets/kde-logo.svg"
import xfce from "@/assets/xfce-logo.svg"
import i3 from "@/assets/i3-logo.svg"

export function HeroOrbitingCircles() {
  return (
    <div className="relative flex h-125 w-full flex-col items-center justify-center overflow-hidden">
      <OrbitingCircles iconSize={40}>
        <img src={alpine} alt="alpine linux" />
        <img src={arch} alt="alpine linux" />
        <img src={fedora} alt="alpine linux" />
        <img src={ubuntu} alt="alpine linux" />
        <img src={debian} alt="alpine linux" />
      </OrbitingCircles>
      <OrbitingCircles iconSize={30} radius={100} reverse speed={2}>
        <img src={kde} alt="alpine linux" />
        <img src={xfce} alt="alpine linux" />
        <img src={i3} alt="alpine linux" />
      </OrbitingCircles>
    </div>
  )
}
