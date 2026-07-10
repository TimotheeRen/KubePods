import { ArrowUpRight, CircleCheck } from "lucide-react";
import { cn } from "@/lib/utils";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { motion } from "motion/react";
import RegisterDialog from "./registerDialog";

const plans = [
  {
    name: "Starter",
    price: 3.49,
    description:
      "For occasionnal users",
    features: [
      { title: "100h desktops usage per month." },
      { title: "Maximum 3 desktops at a time" },
      { title: "No persistance." },
    ],
  },
  {
    name: "Standard",
    price: 4.99,
    isRecommended: true,
    description:
      "For regular users, who uses more their desktops regularly.",
    features: [
      { title: "200h desktops usage per month" },
      { title: "Maximum 5 desktops at a time" },
      { title: "No persistance." },
    ],
    isPopular: true,
  },
  {
    name: "Premium",
    price: 6.99,
    description:
      "For advanced users, who uses more their desktops very often.",
    features: [
      { title: "300h desktops usage per month" },
      { title: "Maximum 7 desktops at a time" },
      { title: "No persistance." },
    ],
  },
];

const Pricing = () => {
  return (
    <div id="pricing" className="px-6 py-10">
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
      >
        Choose a Plan<br />
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
          Choose the plan that bests fits your needs
        </motion.span>
      </motion.h2>

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
        className="mx-auto mt-12 grid max-w-(--breakpoint-lg) grid-cols-1 items-center gap-8 sm:mt-16 lg:grid-cols-3 lg:gap-0"
      >
        {plans.map((plan) => (
          <div
            className={cn("relative rounded-lg border bg-card/50 p-6 px-8", {
              "z-1 overflow-hidden bg-card px-10 py-14 shadow-[0px_1px_6px_0px_rgba(0,0,0,0.07)] lg:-mx-2":
                plan.isPopular,
            })}
            key={plan.name}
          >
            {plan.isPopular && (
              <Badge className="absolute top-2 right-2 px-2.5 py-1 uppercase">
                Most Popular
              </Badge>
            )}
            <h3 className="font-medium text-lg">{plan.name}</h3>

            <p className="mt-4 font-semibold text-4xl">
              ${plan.price}
              <span className="ml-1.5 font-normal text-muted-foreground text-sm">
                /month
              </span>
            </p>

            <p className="mt-4 text-muted-foreground text-sm">
              {plan.description}
            </p>

            <RegisterDialog>
              <Button
                className="mt-6 w-full rounded-full text-base"
                size="lg"
                variant={plan.isPopular ? "default" : "outline"}
              >
                Get Started <ArrowUpRight className="h-4 w-4" />
              </Button>
            </RegisterDialog>
            <Separator className="my-8" />
            <ul className="space-y-3">
              {plan.features.map((feature) => (
                <li className="flex items-start gap-1.5" key={feature.title}>
                  <CircleCheck className="mt-1 h-4 w-4 text-green-600" />
                  {feature.title}
                </li>
              ))}
            </ul>
          </div>
        ))}
      </motion.div>
    </div>
  );
};

export default Pricing;
