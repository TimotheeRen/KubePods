import { motion } from "motion/react";

export default function SubTitle() {
  return (
    <motion.h1
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
          duration: 0.7,
          delay: 1,
          ease: "easeInOut"
        }
      }}
      className="text-muted-foreground/80 mb-4"
    >
      A secure cloud native desktops hosting plateform.
    </motion.h1 >
  )
}
