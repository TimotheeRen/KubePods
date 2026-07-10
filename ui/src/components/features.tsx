import { AppWindow, Earth, SquareDashedMousePointer, Timer } from "lucide-react";
import { motion } from "motion/react";
import { HeroAnimatedList } from "./heroAnimatedList";
import { HeroOrbitingCircles } from "./heroOrbitingCircles";

const Features = () => {
  const viewVariant = {
    initial: {
      opacity: 0,
      filter: "blur(8px)",
      y: 20
    },
    visible: {
      opacity: 1,
      filter: "blur(0px)",
      y: 0,
      transition: {
        duration: 0.5,
        delay: 0.2,
        ease: "easeInOut"
      }
    }
  } as const
  return (
    <div className="mx-auto w-full max-w-(--breakpoint-lg) px-6 py-10">
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
        viewport={{ once: true }}
        className="font-medium text-3xl leading-10 tracking-[-0.04em] sm:text-4xl md:text-[40px] md:leading-13"
        id="features"
      >
        Stop configuring, start working: <br />
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
          viewport={{ once: true }}
          className="text-muted-foreground/80"
        >
          Spin up instantly customized environment
        </motion.span>
      </motion.h2>
      <div className="mt-14 grid gap-6 sm:grid-cols-2 md:grid-cols-5 lg:grid-cols-3">
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
          viewport={{ once: true }}
          className="col-span-1 rounded-xl bg-background p-6 md:col-span-2 lg:col-span-1"
        >
          <motion.div variants={viewVariant} initial="initial" whileInView="visible" className="mb-6 aspect-video w-full rounded-xl bg-background md:hidden" viewport={{ once: true }}>
            <HeroAnimatedList />
          </motion.div>

          <span className="font-medium text-xl tracking-[-0.01em]">
            Quick setups
          </span>

          <ul className="mt-6 space-y-5">
            <li>
              <div className="flex items-start gap-3">
                <Timer className="shrink-0" />
                <p className="-mt-0.5">
                  Spin up desktops in seconds, not hours.
                </p>
              </div>
            </li>
            <li>
              <div className="flex items-start gap-3">
                <Earth className="shrink-0" />
                <p className="-mt-0.5">
                  Access them from anywhere, whether at home or in vacation.
                </p>
              </div>
            </li>
          </ul>

        </motion.div>
        <motion.div variants={viewVariant} initial="initial" whileInView="visible" className="col-span-1 hidden rounded-xl bg-background md:col-span-3 md:block lg:col-span-2" viewport={{ once: true }}>
          <HeroAnimatedList />
        </motion.div>

        <motion.div variants={viewVariant} initial="initial" whileInView="visible" className="col-span-1 hidden rounded-xl bg-background md:col-span-3 md:block lg:col-span-2" viewport={{ once: true }}>
          <HeroOrbitingCircles />
        </motion.div>

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
          viewport={{ once: true }}
          className="col-span-1 rounded-xl bg-background p-6 md:col-span-2 lg:col-span-1"
        >
          <motion.div variants={viewVariant} initial="initial" whileInView="visible" className="mb-6 aspect-video w-full rounded-xl bg-background md:hidden" viewport={{ once: true }}>
            <HeroOrbitingCircles />
          </motion.div>

          <span className="font-medium text-xl tracking-[-0.01em]">
            Make them yours
          </span>

          <ul className="mt-6 space-y-4">
            <li>
              <div className="flex items-start gap-3">
                <SquareDashedMousePointer className="shrink-0" />
                <p className="-mt-0.5">
                  Choose the linux distribution that bests fits your needs.
                </p>
              </div>
            </li>
            <li>
              <div className="flex items-start gap-3">
                <AppWindow className="shrink-0" />
                <p className="-mt-0.5">
                  Choose your favorite desktop environment
                </p>
              </div>
            </li>
          </ul>

        </motion.div>
      </div>
    </div >
  );
};

export default Features;
