import BigTitle from "@/components/bigTitle";
import Features from "@/components/features";
import FeaturesCards from "@/components/featuresCards";
import HeroButton from "@/components/heroButton";
import Navbar from "@/components/navbar";
import PreviewFrame from "@/components/previewFrame";
import Pricing from "@/components/pricing";
import RegisterDialog from "@/components/registerDialog";
import SubTitle from "@/components/subTitle";
import { LightRays } from "@/components/ui/light-rays";
import CTA from "@/components/cta";
import Footer from "@/components/footer";

export default function Root() {
  return (
    <div className="text-center relative min-h-screen w-full overflow-hidden" id="home">
      <LightRays color="rgba(160, 210, 255, 0.5)" className="absolute -z-10" />
      <Navbar />
      <div className="mt-32 mb-14 text-center">
        <BigTitle />
        <SubTitle />
        <div className="flex justify-center gap-2">
          <RegisterDialog>
            <HeroButton />
          </RegisterDialog>
        </div>
      </div>
      <PreviewFrame />
      <Features />
      <FeaturesCards />
      <Pricing />
      <CTA />
      <Footer />
    </div>
  )
}
