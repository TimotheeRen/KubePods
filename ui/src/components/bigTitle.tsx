import { motion } from "motion/react";

export default function BigTitle() {
  const containerVariant = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.1,
      }
    }
  }

  const wordVariants = {
    hidden: {
      opacity: 0,
      y: 10,
      rotate: 5,
      filter: "blur(16px)"
    },
    visible: {
      opacity: 1,
      y: 0,
      rotate: 0,
      filter: "blur(0px)",
      transition: {
        type: "spring",
        damping: 15,
        stiffness: 100,
        filter: { duration: 1 }
      }
    }
  } as const

  return (
    <motion.h1
      className="text-6xl font-extrabold mb-2 flex flex-wrap gap-4 text-center justify-center"
      variants={containerVariant}
      initial="hidden"
      animate="visible"
      viewport={{ once: true }}
    >
      <motion.span variants={wordVariants}>Your</motion.span>
      <motion.span variants={wordVariants}>own</motion.span>
      <motion.span variants={wordVariants} className="text-primary">desktops</motion.span>
      <motion.span variants={wordVariants}>at</motion.span>
      <motion.span variants={wordVariants}>your</motion.span>
      <motion.span variants={wordVariants} className="text-primary">reach</motion.span>
      <motion.span variants={wordVariants}>wherever</motion.span>
      <motion.span variants={wordVariants}>you</motion.span>
      <motion.span variants={wordVariants}>are.</motion.span>
    </motion.h1>
  )
}
