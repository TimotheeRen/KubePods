import { Button } from "@/components/ui/button";
import logo from "@/assets/logo.png"
import { motion } from "motion/react";
import RegisterDialog from "./registerDialog";

export default function CTA() {
  return (
    <motion.div
      initial={{
        opacity: 0,
        filter: "blur(8px)",
        y: 20
      }}
      whileInView={{
        opacity: 1,
        filter: "blur(0px)",
        y: 0,
        transition: {
          duration: 0.5,
          ease: "easeInOut"
        }
      }}
      className="px-0 py-16 sm:px-6"
    >
      <div className="relative mx-auto flex w-full max-w-5xl flex-col items-center justify-center rounded-3xl bg-foreground py-16 text-background dark:bg-foreground/7 dark:text-foreground">
        <img src={logo} />
        <h2 className="mt-10 font-medium text-5xl tracking-tighter">
          Ready to elevate your workflow ?
        </h2>
        <p className="mx-auto mt-6 max-w-xl text-center text-muted-foreground text-xl/normal">
          Try KubePods and create your firsts desktops in seconds.
        </p>
        <RegisterDialog>
          <Button className="mt-8">Get Started</Button>
        </RegisterDialog>
      </div>
    </motion.div>
  );
};
