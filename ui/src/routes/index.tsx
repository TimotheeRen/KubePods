import LoginDialog from "@/components/loginDialog";
import Navbar from "@/components/navbar";
import RegisterDialog from "@/components/registerDialog";
import { LightRays } from "@/components/ui/light-rays";

export default function Root() {
  return (
    <div className="text-center relative min-h-screen w-full overflow-hidden">
      <Navbar />
      <div className="mt-32">
        <h1 className="text-6xl font-extrabold mb-2">Your own <span className="text-primary">desktops</span>, at your <span className="text-primary">reach</span> wherever you are.</h1>
        <h1 className="text-muted-foreground mb-4">A secure cloud-native desktops hosting plateform.</h1>
        <div className="flex justify-center gap-2">
          <LoginDialog />
          <RegisterDialog />
        </div>
      </div>
      <LightRays color="rgba(160, 210, 255, 0.5)" />
    </div>
  )
}
