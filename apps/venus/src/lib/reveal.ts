import { type MotionProps } from "motion/react"

export type TReveal = "top" | "right" | "bottom" | "left" | "zoomIn" | "zoomOut"
/**
 * @prop `reveal` value is `top` , `right` , `bottom` , `left`, `zoomIn`, `zoomOut`. Default is `right`
 * @prop `delay` number that start from `0` to `100`. Default is `0`
 * @prop `viewportOnce` if the value is `true`, the animation will only be triggered when the element first appears within the viewport. Default is `false`.
 *
 * @example
 * ```jsx
 * <motion.button
 *  {...motionProps({
 *     reveal: "bottom",
 *     delay: 1.5
 *  })}
 *  className="fixed left-0 top-0 size-10 flex justify-center
 *    items-center rounded-md"
 *  onClick={handleClick}
 * >
 *   +
 * </motion.button>
 * ```
 */
export const motionProps: ({
  reveal,
  delay,
  viewportOnce,
  viewportMargin,
}: {
  reveal?: TReveal
  delay?: number
  viewportOnce?: boolean
  viewportMargin?: string
}) => MotionProps = ({
  reveal = "right",
  delay = 0,
  viewportOnce = false,
  viewportMargin = "0px 0px -100px 0px",
}) => {
  const translateVariants = {
    top: { translateY: -100 },
    right: { translateX: 100 },
    bottom: { translateY: 100 },
    left: { translateX: -100 },
    zoomIn: { scale: 0.8 },
    zoomOut: { scale: 1.2 },
  }

  return {
    variants: {
      hidden: {
        opacity: 0,
        ...translateVariants[reveal],
      },
      show: {
        opacity: 1,
        translateX: 0,
        translateY: 0,
        scale: 1,
        transition: {
          type: "spring",
          stiffness: 60,
          bounce: 0.25,
          duration: 0.8,
          delay: Math.min(delay, 100) * 0.3,
        },
      },
    },
    initial: "hidden",
    whileInView: "show",
    viewport: {
      once: viewportOnce,
      margin: viewportMargin,
    },
  }
}
