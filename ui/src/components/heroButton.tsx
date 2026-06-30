import { motion } from "motion/react";
import { ShimmerButton } from "./ui/shimmer-button";

export default function HeroButton() {
  return (
    <motion.div
      initial={{
        opacity: 0,
        filter: "blur(8px)",
        y: 10
      }}
      animate={{
        opacity: 1,
        filter: "blur(0px)",
        y: 0,
        transition: {
          duration: 0.5,
          delay: 1.7,
          ease: "easeInOut"
        }
      }}
    >
      <ShimmerButton background="rgba(99, 102, 241, 1)" className="h-10 w-20 px-15" >Try it now</ShimmerButton>
    </motion.div>
  )
}
