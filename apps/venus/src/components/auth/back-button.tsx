"use client"

import { Button } from "@/components/ui/button"
import { Link } from "react-router"

interface BackButtonProps {
  href: string
  label: string
}

export const BackButton = ({ href, label }: BackButtonProps) => {
  return (
    <Button variant="link" className="w-full font-normal" size="sm" asChild>
      <Link to={href}>{label}</Link>
    </Button>
  )
}
