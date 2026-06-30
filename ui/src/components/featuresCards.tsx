import { motion } from "motion/react";
import { CardHeader } from "./ui/card";
import { CodeXml, Gauge, HatGlasses, PaintbrushVertical, SaveOff, Zap } from "lucide-react";
import { MagicCard } from "./ui/magic-card";

const features = [
  {
    icon: Gauge,
    title: "Performant",
    description:
      "KubePods desktops have 8 Go of RAM and 4 vCPU, making them performant.",
  },
  {
    icon: Zap,
    title: "Boots in seconds",
    description:
      "The desktops rely on containarization technology, allowing faster boot times.",
  },
  {
    icon: HatGlasses,
    title: "Privacy focused",
    description:
      "KubePods doesn't collect any telemetry datas that you didn't agree for.",
  },
  {
    icon: SaveOff,
    title: "No persistance",
    description:
      "KubePods desktops are designed as throwable units, therefore no data are saved.",
  },
  {
    icon: PaintbrushVertical,
    title: "An intuitive interface",
    description:
      "KubePods interfaces are designed to combine simplicity and aesthetic.",
  },
  {
    icon: CodeXml,
    title: "100% Open source",
    description:
      "KubePods source code is fully open source on Github for a maximum transparency.",
  },
];

export default function FeaturesCards() {
  const containerVariant = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.1,
      }
    }
  }

  const cardVariant = {
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
    <div className="mx-auto w-full max-w-(--breakpoint-lg) px-6 py-20">
      <motion.h2
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
        className="font-medium text-3xl leading-10 tracking-[-0.04em] sm:text-4xl md:text-[40px] md:leading-13"
        id="why-kubepods"
      >
        Why KubePods<br />
        <motion.span
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
              delay: 0.2,
              ease: "easeInOut"
            }
          }}
          className="text-muted-foreground/80"
        >
          KubePods deliver clouds desktops at low prices
        </motion.span>
      </motion.h2>

      <div className="px-6 py-20">
        <div className="mx-auto w-full max-w-(--breakpoint-lg)">
          <motion.div
            variants={containerVariant}
            initial="hidden"
            whileInView="visible"
            className="mx-auto -mt-4 grid w-full gap-x-6 gap-y-8 md:grid-cols-2 lg:grid-cols-3"
          >
            {features.map((feature) => (
              <motion.div variants={cardVariant}>
                <MagicCard
                  className="flex flex-col overflow-hidden rounded-xl pb-5 shadow-none pt-3"
                  key={feature.title}
                >
                  <CardHeader>
                    <feature.icon />
                    <h4 className="mt-3! font-medium text-xl tracking-tight text-left">
                      {feature.title}
                    </h4>
                    <p className="text-[17px] text-muted-foreground text-left">
                      {feature.description}
                    </p>
                  </CardHeader>
                </MagicCard>
              </motion.div>
            ))}
          </motion.div>
        </div>
      </div>

    </div >
  )
}
