import { motionProps } from "@/lib/reveal"
import { cn } from "@/lib/utils"
import { ChevronDown } from "lucide-react"
import { AnimatePresence, motion } from "motion/react"
import React from "react"
import { Button } from "../ui/button"

export const Accordion = React.forwardRef<
  HTMLDivElement,
  {
    question: string
    answer: string
    index: number
  }
>((props, ref) => {
  const [isOpen, setIsOpen] = React.useState(false)
  const { question, answer, index } = props

  return (
    <motion.div
      {...motionProps({
        reveal: "bottom",
        delay: index * 0.3,
        viewportMargin: "0px",
      })}
    >
      <motion.div
        ref={ref}
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.2, delay: index * 0.1 }}
        className={cn(
          "group rounded-lg",
          "transition-all duration-200 ease-in-out",
          "border-border/50 border",
          isOpen
            ? "from-background via-muted/50 to-background bg-gradient-to-br"
            : "hover:bg-muted/50"
        )}
      >
        <Button
          variant="ghost"
          onClick={() => setIsOpen(!isOpen)}
          className="h-auto w-full justify-between px-6 py-4 hover:bg-transparent"
        >
          <h3
            className={cn(
              "truncate text-left text-sm font-semibold transition-colors duration-200 md:text-base",
              "text-foreground/70",
              isOpen && "text-foreground"
            )}
          >
            {question}
          </h3>
          <motion.div
            animate={{
              rotate: isOpen ? 180 : 0,
              scale: isOpen ? 1.1 : 1,
            }}
            transition={{ duration: 0.2 }}
            className={cn(
              "flex-shrink-0 rounded-full p-0.5",
              "transition-colors duration-200",
              isOpen ? "text-primary" : "text-muted-foreground"
            )}
          >
            <ChevronDown className="h-4 w-4" />
          </motion.div>
        </Button>
        <AnimatePresence initial={false}>
          {isOpen && (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{
                height: "auto",
                opacity: 1,
                transition: { duration: 0.2, ease: "easeOut" },
              }}
              exit={{
                height: 0,
                opacity: 0,
                transition: { duration: 0.2, ease: "easeIn" },
              }}
            >
              <div className="px-6 pt-2 pb-4">
                <motion.p
                  initial={{ y: -10, opacity: 0 }}
                  animate={{ y: 0, opacity: 1 }}
                  exit={{ y: -10, opacity: 0 }}
                  className="text-muted-foreground text-xs leading-relaxed md:text-sm"
                >
                  {answer}
                </motion.p>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </motion.div>
    </motion.div>
  )
})

Accordion.displayName = "Accordion"
