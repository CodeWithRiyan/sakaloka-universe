import confetti from "canvas-confetti"

export const handleClickConfetti = () => {
  const end = Date.now() + 3 * 1000 // 3 seconds
  const colors = ["#F0B400", "#f8d97e", "#F8F8F8", "#f8deb1"]

  const frame = () => {
    if (Date.now() > end) return

    confetti({
      particleCount: 2,
      angle: 60,
      spread: 55,
      startVelocity: 60,
      origin: { x: 0, y: 0.5 },
      colors: colors,
    })
    confetti({
      particleCount: 2,
      angle: 120,
      spread: 55,
      startVelocity: 60,
      origin: { x: 1, y: 0.5 },
      colors: colors,
    })

    requestAnimationFrame(frame)
  }

  frame()
}
