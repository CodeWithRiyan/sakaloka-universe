"use client"

import { motion } from "motion/react"
import { useEffect, useState } from "react"

export function StarRating({
  rating = 4.3,
  maxStars = 5,
  size = "size-5",
  showRating = true,
  delay = 0,
}) {
  const [isClient, setIsClient] = useState(false)
  const [animationKey, setAnimationKey] = useState(0)

  useEffect(() => {
    setIsClient(true)
    // Trigger animation when rating changes
    setAnimationKey((prev) => prev + 1)
  }, [rating])

  const getStarType = (starIndex: number) => {
    const starValue = starIndex + 1

    if (rating >= starValue) {
      return { type: "full", percentage: 100 }
    } else if (rating > starIndex && rating < starValue) {
      const percentage = (rating - starIndex) * 100
      return {
        type: "partial",
        percentage: Math.max(0, Math.min(100, percentage)),
      }
    } else {
      return { type: "empty", percentage: 0 }
    }
  }

  const renderStar = (index: number) => {
    const starType = getStarType(index)
    const starPath =
      "M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"

    return (
      <motion.div
        key={`${index}-${animationKey}`}
        className="relative inline-block"
        initial={{ scale: 0, rotate: -180 }}
        whileInView={{ scale: 1, rotate: 0 }}
        transition={{
          duration: 0.5,
          delay: index * 0.2 + delay,
          type: "spring",
          stiffness: 200,
          damping: 15,
        }}
      >
        {/* Background star (gray) */}
        <svg
          className={`${size} text-gray-300`}
          fill="currentColor"
          viewBox="0 0 20 20"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path d={starPath} />
        </svg>

        {/* Foreground star (yellow) with clip animation */}
        {starType.percentage > 0 && (
          <motion.div
            className="absolute top-0 left-0 overflow-hidden"
            initial={{ width: "0%" }}
            whileInView={{ width: `${starType.percentage}%` }}
            transition={{
              duration: 0.8,
              delay: index * 0.2 + 0.2 + delay,
              type: "spring",
              stiffness: 100,
              damping: 20,
            }}
          >
            <svg
              className={`${size} text-yellow-400`}
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d={starPath} />
            </svg>
          </motion.div>
        )}

        {/* Sparkle effect for filled stars */}
        {starType.percentage >= 100 && (
          <motion.div
            className="pointer-events-none absolute inset-0"
            initial={{ opacity: 0, scale: 0.5 }}
            whileInView={{
              opacity: [0, 1, 0],
              scale: [0.5, 1.2, 1],
            }}
            transition={{
              duration: 0.6,
              delay: index * 0.2 + 0.8 + delay,
              ease: "easeOut",
            }}
          >
            <svg
              className={`${size} text-yellow-300`}
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path d={starPath} />
            </svg>
          </motion.div>
        )}
      </motion.div>
    )
  }

  if (!isClient) return null

  return (
    <div className="flex items-center space-x-1">
      <div className="flex space-x-1">
        {[...Array(maxStars)].map((_, index) => renderStar(index))}
      </div>
      {showRating && (
        <motion.span
          className="ml-2 text-sm text-gray-500"
          initial={{ opacity: 0, x: -10 }}
          whileInView={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.5, delay: maxStars * 0.1 + 0.5 }}
        >
          {rating.toFixed(1)} / {maxStars}
        </motion.span>
      )}
    </div>
  )
}
