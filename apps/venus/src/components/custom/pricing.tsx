"use client"

import { buttonVariants } from "@/components/ui/button"
// import { Label } from '@/components/ui/label'
// import { Switch } from '@/components/ui/switch'
import { handleClickConfetti } from "@/lib/confetti"
import NumberFlow from "@number-flow/react"
import { Check, Star } from "lucide-react"
import { motion } from "motion/react"
// import { useRef } from "react"
import useBreakpoint from "@/hooks/use-breakpoint"
import { cn } from "../../lib/utils"

interface PricingPlan {
  name: string
  price: string
  features: string[]
  description: string
  buttonText: string
  href: string
  isPopular: boolean
}

interface PricingProps {
  plans: PricingPlan[]
}

export function Pricing({ plans }: PricingProps) {
  const { xl } = useBreakpoint()
  // const switchRef = useRef<HTMLButtonElement>(null)

  // const handleToggle = (checked: boolean) => {
  //   setIsMonthly(!checked)
  //   if (checked && switchRef.current) {
  //     const rect = switchRef.current.getBoundingClientRect()
  //     const x = rect.left + rect.width / 2
  //     const y = rect.top + rect.height / 2

  //     confetti({
  //       particleCount: 50,
  //       spread: 60,
  //       origin: {
  //         x: x / window.innerWidth,
  //         y: y / window.innerHeight,
  //       },
  //       colors: [
  //         'hsl(var(--primary))',
  //         'hsl(var(--accent))',
  //         'hsl(var(--secondary))',
  //         'hsl(var(--muted))',
  //       ],
  //       ticks: 200,
  //       gravity: 1.2,
  //       decay: 0.94,
  //       startVelocity: 30,
  //       shapes: ['circle'],
  //     })
  //   }
  // }

  return (
    <div className="sm:2 grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4 xl:gap-0">
      {plans.map((plan, index) => (
        <motion.div
          key={index}
          initial={{ y: 50, opacity: 1, marginRight: 0 }}
          whileInView={
            xl
              ? {
                  y: 0,
                  x: !plan.isPopular ? 0 : -8,
                  opacity: 1,
                  marginRight: 16,
                  scale: !plan.isPopular ? 1.0 : 1.05,
                }
              : {}
          }
          viewport={{ once: true }}
          transition={{
            duration: 1.6,
            type: "spring",
            stiffness: 100,
            damping: 30,
            delay: 0.4,
            opacity: { duration: 0.5 },
          }}
          className={cn(
            `bg-background relative rounded-2xl border-[1px] p-6 py-10 text-center lg:flex lg:flex-col lg:justify-center`,
            plan.isPopular ? "border-primary border-2" : "border-border",
            "flex flex-col",
            !plan.isPopular && "mt-5",
            index === 0 || index === 2
              ? "z-0 translate-x-0 translate-y-0 -translate-z-[50px] rotate-y-[10deg] transform"
              : "z-10",
            index === 0 && "origin-right",
            index === 2 && "origin-left"
          )}
        >
          {plan.isPopular && (
            <div className="bg-primary absolute top-0 right-0 flex items-center rounded-tr-xl rounded-bl-xl px-2 py-0.5">
              <Star className="text-primary-foreground h-4 w-4 fill-current" />
              <span className="text-primary-foreground ml-1 font-sans font-semibold">
                Popular
              </span>
            </div>
          )}
          <div className="flex flex-1 flex-col justify-between">
            <div className="flex flex-1 flex-col">
              <p
                className={cn(
                  "text-muted-foreground text-xl font-semibold",
                  plan.isPopular && "text-primary"
                )}
              >
                {plan.name}
              </p>
              <p className="text-muted-foreground mt-2 text-xs leading-5">
                {plan.description}
              </p>
              <div className="mt-6 flex items-center justify-center gap-x-2">
                <span className="text-foreground text-4xl font-bold tracking-tight">
                  {plans.length === index + 1 ? (
                    plan.price
                  ) : (
                    <NumberFlow
                      value={Number(plan.price)}
                      format={{
                        style: "currency",
                        currency: "IDR",
                        minimumFractionDigits: 0,
                        maximumFractionDigits: 0,
                      }}
                      transformTiming={{
                        duration: 500,
                        easing: "ease-out",
                      }}
                      willChange
                      className="font-variant-numeric: tabular-nums"
                    />
                  )}
                </span>
              </div>

              <ul className="mt-5 flex flex-col gap-2">
                {plan.features.map((feature, idx) => (
                  <li key={idx} className="flex items-start gap-2">
                    <Check className="text-primary mt-1 h-4 w-4 flex-shrink-0" />
                    <span className="text-left">{feature}</span>
                  </li>
                ))}
              </ul>

              <hr className="my-4 w-full" />
            </div>

            <a
              href={plan.href}
              target="_blank"
              rel="noopener noreferrer"
              aria-label={plan.name}
              onClick={handleClickConfetti}
              className={cn(
                buttonVariants({
                  variant: "outline",
                }),
                "group relative w-full gap-2 overflow-hidden text-lg font-semibold tracking-tighter",
                "hover:bg-primary hover:text-primary-foreground hover:ring-primary transform-gpu ring-offset-current transition-all duration-300 ease-out hover:ring-2 hover:ring-offset-1",
                plan.isPopular
                  ? "bg-primary text-primary-foreground"
                  : "bg-background text-foreground"
              )}
            >
              {plan.buttonText}
            </a>
          </div>
        </motion.div>
      ))}
    </div>
  )
}

export default Pricing
