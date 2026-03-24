"use client"

import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { BackButton } from "./back-button"
import { Header } from "./header"
import Information from "./information"
import { Social } from "./social"

interface CardWrapperProps {
  children: React.ReactNode
  headerLabel?: string
  showSocial?: boolean
  backButtonLabel?: string
  backButtonHref?: string
  pageInformation?: string
}

export const CardWrapper = ({
  children,
  headerLabel,
  backButtonLabel,
  backButtonHref,
  showSocial,
  pageInformation,
}: CardWrapperProps) => {
  return (
    <Card className="flex h-screen w-full flex-col justify-between overflow-y-auto rounded-none border-none py-5 shadow-none transition-all duration-300 md:py-10 md:pr-10 md:pl-5 lg:pr-20 lg:pl-10">
      <div>
        <CardHeader>
          <Header label={headerLabel ?? ""} />
        </CardHeader>
        <CardContent>{children}</CardContent>
        {showSocial && (
          <CardFooter>
            <Social />
          </CardFooter>
        )}
      </div>
      <CardFooter>
        {backButtonHref && backButtonLabel ? (
          <BackButton label={backButtonLabel} href={backButtonHref} />
        ) : (
          <Information page={pageInformation} />
        )}
      </CardFooter>
    </Card>
  )
}
