import { Backlight } from "./ui/backlight";
import { Safari } from "./ui/safari";
import preview from "@/assets/preview.png"

export default function PreviewFrame() {
  return (
    <div className="w-full max-w-5xl mx-auto mb-10 p-2 rounded-xl bg-background/20 backdrop-blur-2xl transition-all duration-300 ease-in-out hover:scale-[1.02]">
      <Backlight blur={5} className="w-full">
        <Safari url="example.kubepods.com" imageSrc={preview} />
      </Backlight>
    </div>
  )
}
