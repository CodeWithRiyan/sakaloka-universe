/**
 * Quarantined landing page content.
 *
 * This file intentionally remains in the repository as source material for a
 * future centralized marketing site managed through Strapi. It must not be
 * wired back into Venus routes or imported into the POS application bundle.
 */
import { Accordion } from "@/components/custom/accordion"
import { OrbitingCircles } from "@/components/custom/orbiting-circles"
import Pricing from "@/components/custom/pricing"
import { StarRating } from "@/components/custom/star-rating"
import WordRotate from "@/components/custom/word-rotate"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { motionProps } from "@/lib/reveal"
import { DynamicIcon } from "lucide-react/dynamic"
import { motion } from "motion/react"
import { Helmet } from "react-helmet-async"
import { IoMdMail } from "react-icons/io"
import { Link } from "react-router"
import {
  faqSchema,
  landingData,
  linkTags,
  metaTags,
  organizationSchema,
  seoMetadata,
  softwareSchema,
  webPageSchema,
} from "./content"

export default function LandingPage() {
  return (
    <>
      <Helmet>
        <title>{seoMetadata.title}</title>
      </Helmet>
      {metaTags.map((tag, index) => (
        <meta key={index} {...tag} />
      ))}
      {linkTags.map((link, index) => (
        <link key={index} {...link} />
      ))}
      <script type="application/ld+json">
        {JSON.stringify(organizationSchema)}
      </script>
      <script type="application/ld+json">
        {JSON.stringify(softwareSchema)}
      </script>
      <script type="application/ld+json">{JSON.stringify(faqSchema)}</script>
      <script type="application/ld+json">
        {JSON.stringify(webPageSchema)}
      </script>

      <main className="flex flex-col items-stretch justify-center overflow-hidden">
        <section
          id="home"
          className="bg-primary relative overflow-hidden pt-32 md:pt-40"
        >
          <div className="container mx-auto px-3 sm:max-w-[540px] md:max-w-[720px] md:px-20 lg:max-w-[1140px] xl:max-w-[1690px]">
            <div className="mb-10 grid grid-cols-1 gap-8 xl:grid-cols-2">
              <div
                className="relative mx-auto space-y-8 text-center sm:max-w-[570px] md:space-y-10 xl:mx-0 xl:text-start"
                data-cue="slideInUp"
                data-show="true"
                style={{
                  animation: "slideInUp 600ms ease 0ms normal forwards",
                }}
              >
                <div className="text-primary-foreground">
                  <motion.h1
                    className="mb-5 text-2xl font-bold text-balance sm:text-3xl md:mb-7 md:text-4xl lg:mb-5 lg:text-5xl lg:leading-[1.2] xl:mb-6"
                    {...motionProps({ reveal: "bottom" })}
                  >
                    <span>{landingData.hero.titlePrefix}</span>
                    <WordRotate
                      words={[...landingData.hero.rotatingWords]}
                      className="pl-2 md:pl-3"
                    />
                  </motion.h1>
                  <motion.p
                    className="text-sm md:lg:text-base xl:text-lg"
                    {...motionProps({ reveal: "bottom", delay: 0.3 })}
                  >
                    {landingData.hero.description}
                  </motion.p>
                </div>
                <motion.div
                  className="flex items-center justify-center space-x-4 xl:justify-start"
                  {...motionProps({ reveal: "bottom", delay: 0.6 })}
                >
                  <Button
                    variant="white"
                    href={landingData.hero.primaryCta.href}
                    className="font-semibold"
                  >
                    {landingData.hero.primaryCta.label}
                  </Button>
                  <Button
                    variant="success"
                    href={landingData.hero.secondaryCta.href}
                    className="font-semibold"
                  >
                    {landingData.hero.secondaryCta.label}
                  </Button>
                </motion.div>
                <motion.div
                  className="flex items-center justify-center space-x-[15px] xl:justify-start ltr:ml-[2px] rtl:mr-[2px]"
                  {...motionProps({ reveal: "left", delay: 0.9 })}
                >
                  <div className="flex -space-x-2">
                    {landingData.hero.previewAssets.map((asset, index) => (
                      <img
                        key={index}
                        src={asset}
                        className="inline-block size-12 rounded-full object-cover object-top ring-[3px] ring-white"
                        alt="customer review"
                      />
                    ))}
                  </div>
                  <div>
                    <StarRating showRating={false} delay={0.9} />
                    <p className="text-primary-foreground">
                      {landingData.hero.reviewLabel}
                    </p>
                  </div>
                </motion.div>
              </div>
              <motion.div
                className="text-center xl:text-end"
                data-cue="slideInUp"
                data-show="true"
                {...motionProps({ reveal: "right", delay: 0.9 })}
              >
                <img
                  src={landingData.hero.heroImage}
                  className="inline-block"
                  alt="banner5"
                />
              </motion.div>
            </div>
          </div>
          <div className="bg-background overflow-hidden">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 1440 320"
              className="text-primary"
            >
              <path
                fill="currentColor"
                fillOpacity="1"
                d="M0,224L40,208C80,192,160,160,240,160C320,160,400,192,480,181.3C560,171,640,117,720,128C800,139,880,213,960,240C1040,267,1120,245,1200,224C1280,203,1360,181,1400,170.7L1440,160L1440,0L1400,0C1360,0,1280,0,1200,0C1120,0,1040,0,960,0C880,0,800,0,720,0C640,0,560,0,480,0C400,0,320,0,240,0C160,0,80,0,40,0L0,0Z"
              ></path>
            </svg>
          </div>
        </section>

        <section
          id="features"
          className="space-y-8 px-10 py-20 md:space-y-16 md:py-40 lg:px-16"
        >
          <div className="flex flex-col items-center gap-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.features.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              <span>{landingData.features.title[1]}</span>
              <span className="text-primary underline decoration-wavy md:decoration-4">
                {landingData.features.title[2]}
              </span>
              <span>{landingData.features.title[3]}</span>
            </motion.h1>
            <motion.p
              className="max-w-3xl text-sm text-balance md:text-base"
              {...motionProps({ reveal: "bottom", delay: 0.6 })}
            >
              {landingData.features.description}
            </motion.p>
          </div>
          <div className="flex flex-col items-center gap-4 md:flex-row">
            {/* <div className="flex-none w-full md:w-1/2 flex justify-center">
              <img src="/laptop-phone.png" alt="" className="object-contain" />
            </div> */}
            <div className="grid grid-cols-1 gap-6 md:grid-cols-2 2xl:grid-cols-3">
              {landingData.features.feature_list.map((item, index) => (
                <motion.div
                  key={index}
                  className="flex items-center gap-8 rounded-xl border p-4 shadow-md"
                  {...motionProps({
                    reveal: "zoomIn",
                    delay: index * 0.3 + 1.2,
                  })}
                >
                  <div className="bg-primary rounded-full p-4">
                    <DynamicIcon
                      // eslint-disable-next-line @typescript-eslint/no-explicit-any
                      name={item.icon as any}
                      size={32}
                      strokeWidth={1.6}
                      className="text-primary-foreground"
                    />
                  </div>
                  <div className="flex flex-col gap-1">
                    <h4 className="text-lg font-semibold md:text-xl">
                      {item.title}
                    </h4>
                    <span>{item.description}</span>
                  </div>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        <section id="benefits">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1440 320"
            className="text-sky-100"
          >
            <path
              fill="currentColor"
              fillOpacity="1"
              d="M0,192L40,170.7C80,149,160,107,240,96C320,85,400,107,480,138.7C560,171,640,213,720,202.7C800,192,880,128,960,117.3C1040,107,1120,149,1200,176C1280,203,1360,213,1400,218.7L1440,224L1440,320L1400,320C1360,320,1280,320,1200,320C1120,320,1040,320,960,320C880,320,800,320,720,320C640,320,560,320,480,320C400,320,320,320,240,320C160,320,80,320,40,320L0,320Z"
            ></path>
          </svg>
          <div className="flex w-full flex-col bg-sky-100">
            <div className="flex flex-col items-center gap-4 text-center">
              <motion.div {...motionProps({ reveal: "bottom" })}>
                <Badge className="rounded-full px-4">
                  {landingData.benefits.tagline}
                </Badge>
              </motion.div>
              <motion.h1
                className="text-2xl font-semibold md:text-4xl"
                {...motionProps({ reveal: "bottom", delay: 0.3 })}
              >
                {landingData.benefits.title}
              </motion.h1>
              <motion.p
                className="max-w-3xl text-sm text-balance md:text-base"
                {...motionProps({ reveal: "bottom", delay: 0.6 })}
              >
                {landingData.benefits.description}
              </motion.p>
            </div>
            {landingData.benefits.benefit_list.map((item, index) => (
              <div
                key={index}
                className="flex flex-col-reverse items-center gap-10 px-10 py-20 md:px-20 lg:px-40 odd:lg:flex-row even:lg:flex-row-reverse"
              >
                <div className="w-full space-y-8 lg:w-1/2">
                  <div className="space-y-1">
                    <motion.h2
                      className="text-xl font-bold sm:text-4xl"
                      {...motionProps({ reveal: "bottom" })}
                    >
                      <span>{item.title}</span>
                      {/* <span className="inline-block bg-gradient-to-br from-primary to-sky-300 bg-clip-text text-transparent">
                        Kontrol Penuh
                      </span>
                      <span>{` atas Penjualan & Stok`}</span> */}
                    </motion.h2>
                    <motion.p
                      {...motionProps({ reveal: "bottom", delay: 0.3 })}
                    >
                      {item.description}
                    </motion.p>
                  </div>
                  <motion.ul
                    className="space-y-3"
                    {...motionProps({ reveal: "bottom", delay: 0.6 })}
                  >
                    {item.points.map((point, index) => (
                      <motion.li
                        key={index}
                        className="flex gap-4"
                        {...motionProps({
                          reveal: "bottom",
                          delay: index * 0.3 + 0.9,
                        })}
                      >
                        <div className="bg-primary size-10 rounded-xl p-2">
                          <DynamicIcon
                            // eslint-disable-next-line @typescript-eslint/no-explicit-any
                            name={point.icon as any}
                            className="text-primary-foreground flex-none text-2xl"
                          />
                        </div>
                        <span>{point.text}</span>
                      </motion.li>
                    ))}
                  </motion.ul>
                </div>
                <motion.img
                  src={item.image}
                  alt="supermarket-workers"
                  className="w-full object-contain lg:w-1/2"
                  {...motionProps({
                    reveal: "zoomIn",
                    viewportMargin: "0px 0px -500px 0px",
                  })}
                />
              </div>
            ))}
          </div>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1440 320"
            className="text-sky-100"
          >
            <path
              fill="currentColor"
              fillOpacity="1"
              d="M0,32L48,53.3C96,75,192,117,288,128C384,139,480,117,576,101.3C672,85,768,75,864,90.7C960,107,1056,149,1152,154.7C1248,160,1344,128,1392,112L1440,96L1440,0L1392,0C1344,0,1248,0,1152,0C1056,0,960,0,864,0C768,0,672,0,576,0C480,0,384,0,288,0C192,0,96,0,48,0L0,0Z"
            ></path>
          </svg>
        </section>

        <section
          id="payment-integration"
          className="flex flex-col items-center gap-16 py-20"
        >
          <div className="flex max-w-4xl flex-col items-center gap-4 px-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.integrations.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              {landingData.integrations.title}
            </motion.h1>
            <motion.p {...motionProps({ reveal: "bottom", delay: 0.6 })}>
              {landingData.integrations.description}
            </motion.p>
          </div>
          <motion.div
            className="relative flex h-[300px] w-full items-end overflow-hidden"
            {...motionProps({ reveal: "bottom", delay: 0.9 })}
          >
            <div className="relative flex h-[700px] w-full flex-col items-center justify-center overflow-hidden">
              <OrbitingCircles iconSize={52} radius={300}>
                <img
                  src="/svg/payment/dana.svg"
                  alt="dana"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/gopay.svg"
                  alt="gopay"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/shopeepay.svg"
                  alt="shopeepay"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/qris.svg"
                  alt="qris"
                  className="h-full w-full object-contain"
                />
              </OrbitingCircles>
              <OrbitingCircles iconSize={52} radius={200} reverse>
                <img
                  src="/svg/payment/ovo.svg"
                  alt="ovo"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/linkaja.svg"
                  alt="linkaja"
                  className="h-full w-full object-contain"
                />
                <img
                  src="/svg/payment/doku.svg"
                  alt="doku"
                  className="h-full w-full object-contain"
                />
              </OrbitingCircles>
            </div>
            <div className="ground absolute inset-x-0 top-0 h-1/2 bg-gradient-to-b from-white via-white/0 to-white/0" />
          </motion.div>
        </section>

        <section
          id="pricing"
          className="flex flex-col items-center gap-20 px-20 py-20 md:py-40"
        >
          <div className="flex max-w-4xl flex-col items-center gap-4 px-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.pricing.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              {landingData.pricing.title}
            </motion.h1>
            <motion.p {...motionProps({ reveal: "bottom", delay: 0.6 })}>
              {landingData.pricing.description}
            </motion.p>
          </div>
          <motion.div {...motionProps({ reveal: "bottom", delay: 0.9 })}>
            <Pricing plans={landingData.pricing.price_list} />
          </motion.div>
        </section>

        <section
          id="faq"
          className="flex flex-col items-center gap-20 px-4 py-20 md:px-20 md:py-40"
        >
          <div className="flex flex-col items-center gap-4 text-center">
            <motion.div {...motionProps({ reveal: "bottom" })}>
              <Badge className="rounded-full px-4">
                {landingData.faqs.tagline}
              </Badge>
            </motion.div>
            <motion.h1
              className="text-2xl font-semibold md:text-4xl"
              {...motionProps({ reveal: "bottom", delay: 0.3 })}
            >
              {landingData.faqs.title}
            </motion.h1>
          </div>
          <div className="w-full max-w-3xl space-y-2 px-4 md:space-y-4">
            {landingData.faqs.faq_list.map((item, index) => (
              <Accordion
                key={index}
                index={index}
                question={item.question}
                answer={item.answer}
              />
            ))}
          </div>
        </section>

        {/* <section
          id="blog"
          className="flex w-full flex-col gap-10 px-10 py-20 md:px-20"
        >
          <h1 className="text-center text-2xl font-bold md:text-3xl">
            Mungkin Kamu Juga Membutuhkan Ini
          </h1>
          <div className="flex w-full">
            <Swiper
              rewind={true}
              grabCursor={true}
              slidesPerView={1}
              slidesPerGroup={1}
              autoplay={{
                delay: 2500,
                disableOnInteraction: false,
              }}
              breakpoints={{
                768: {
                  slidesPerView: 3,
                  slidesPerGroup: 3,
                },
              }}
              navigation={{
                enabled: true,
              }}
              modules={[EffectCoverflow, Navigation, Autoplay]}
              className="swiper_container"
            >
              {blogs.map((data) => (
                <SwiperSlide key={data.id}>
                  <div className="relative flex w-full justify-center">
                    <Link className="flex w-[95%] flex-col gap-2" href="##">
                      <div className="flex flex-col gap-1">
                        <span className="font-light">{data.updatedAt}</span>
                        <div className="relative aspect-[3/2] w-full rounded-xl overflow-hidden bg-green-300">
                          <img
                            src={data.thumb}
                            alt="slide_image"
                            fill
                            sizes="(max-width: 768px) 33vw, (max-width: 1200px) 70vw, 100vw"
                            className="object-cover w-full"
                          />
                        </div>
                      </div>
                      <div className="flex flex-col gap-1">
                        <p className="line-clamp-2 md:text-lg font-bold leading-snug text-slate-900">
                          {data.title}
                        </p>
                        <p className="line-clamp-4 leading-tight">
                          {data.content}
                        </p>
                      </div>
                    </Link>
                  </div>
                </SwiperSlide>
              ))}
            </Swiper>
          </div>
        </section> */}
        <section
          id="bantuan"
          className="relative w-full px-0 py-[1px] text-white sm:my-20 sm:py-10 md:px-10 md:py-20 lg:px-32"
        >
          <motion.div
            className="absolute top-0 left-10 hidden aspect-[9/16] h-full sm:block md:left-16 lg:left-56"
            {...motionProps({ reveal: "left" })}
          >
            <img
              src={landingData.supportCta.image}
              alt=""
              className="object-contain"
            />
          </motion.div>
          <div className="bg-primary sm:bg-primary flex min-h-56 w-full flex-col items-end justify-center gap-10 px-10 py-10 pr-10 transition-all sm:items-start sm:pl-56 md:min-h-80 md:rounded-3xl md:pl-80 lg:pr-20 lg:pl-[420px]">
            <div className="flex flex-col items-end gap-1 sm:items-start">
              <motion.span
                className="text-xl font-semibold md:text-2xl lg:text-3xl"
                {...motionProps({ reveal: "bottom", delay: 0.3 })}
              >
                {landingData.supportCta.title}
              </motion.span>
              <motion.span
                className="font-light lg:text-lg"
                {...motionProps({ reveal: "bottom", delay: 0.6 })}
              >
                {landingData.supportCta.description}
              </motion.span>
            </div>
            <motion.div {...motionProps({ reveal: "bottom", delay: 0.9 })}>
              <Button
                href={landingData.supportCta.button.href}
                variant="success"
                className="px-10 font-semibold"
              >
                {landingData.supportCta.button.label}
              </Button>
            </motion.div>
          </div>
        </section>

        <section>
          <div className="item-start bg-primary flex w-full flex-col gap-5 px-10 py-10 text-white md:flex-row md:items-center md:justify-between md:gap-10 md:px-20 md:pb-0 lg:translate-y-10">
            <div className="flex flex-col gap-3">
              <motion.span
                className="text-xl font-semibold md:text-2xl lg:text-3xl"
                {...motionProps({ reveal: "bottom" })}
              >
                {landingData.finalCta.title}
              </motion.span>
              <motion.span
                className="font-light lg:text-xl"
                {...motionProps({ reveal: "bottom", delay: 0.3 })}
              >
                {landingData.finalCta.description}
              </motion.span>
            </div>
            <motion.div
              className="flex items-center"
              {...motionProps({ reveal: "right", delay: 0.6 })}
            >
              <Button
                href={landingData.finalCta.button.href}
                variant="success"
                className="h-9 px-10 font-semibold"
              >
                {landingData.finalCta.button.label}
              </Button>
            </motion.div>
          </div>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 1440 320"
            className="text-primary"
          >
            <path
              fill="currentColor"
              fillOpacity="1"
              d="M0,128L120,117.3C240,107,480,85,720,90.7C960,96,1200,128,1320,144L1440,160L1440,0L1320,0C1200,0,960,0,720,0C480,0,240,0,120,0L0,0Z"
            ></path>
          </svg>
        </section>

        <footer className="flex w-full flex-col gap-10 p-10 pt-0 lg:-translate-y-28 lg:p-20 lg:py-0">
          <motion.a
            href={landingData.footer.brandHref}
            className="text-primary inline-flex flex-none items-center text-xl font-extrabold transition-colors duration-700"
            {...motionProps({ reveal: "bottom" })}
          >
            {landingData.footer.brand}
          </motion.a>
          <div className="flex w-full flex-col justify-between gap-5 lg:flex-row lg:gap-20">
            <div className="flex flex-col-reverse gap-10 lg:flex-row lg:gap-20">
              <div className="flex max-w-sm min-w-80 flex-col gap-4">
                <motion.span {...motionProps({ reveal: "bottom", delay: 0.3 })}>
                  {landingData.footer.address}
                </motion.span>
                <motion.span {...motionProps({ reveal: "bottom", delay: 0.6 })}>
                  {landingData.footer.copyright}
                </motion.span>
              </div>
              <div className="flex w-full justify-between gap-4">
                {landingData.footer.navigationGroups.map((group, groupIndex) => (
                  <motion.div
                    key={groupIndex}
                    className="flex flex-col gap-4"
                    {...motionProps({
                      reveal: "bottom",
                      delay: 0.9 + groupIndex * 0.3,
                    })}
                  >
                    {group.map((item) => (
                      <Link
                        key={item.label}
                        className="hover:text-primary cursor-pointer"
                        to={item.href}
                      >
                        {item.label}
                      </Link>
                    ))}
                  </motion.div>
                ))}
              </div>
            </div>
            <div className="flex flex-none flex-col gap-2">
              <motion.div
                className="group flex items-center gap-2 hover:cursor-pointer"
                {...motionProps({ reveal: "bottom", delay: 1.8 })}
              >
                <IoMdMail className="transition-color group-hover:text-primary text-3xl duration-300 group-hover:drop-shadow-md" />
                <span>{landingData.footer.email}</span>
              </motion.div>
              <motion.div
                className="flex gap-3"
                {...motionProps({ reveal: "bottom", delay: 2.1 })}
              >
                <a href={landingData.footer.socialLinks[0].href}>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="33"
                    viewBox="0 0 32 33"
                    fill="none"
                    className="transition-color hover:text-primary duration-300 hover:drop-shadow-md"
                  >
                    <g clipPath="url(#clip0_162_129)">
                      <path
                        d="M32 16.4443C32 7.60684 24.8375 0.444336 16 0.444336C7.1625 0.444336 0 7.60684 0 16.4443C0 25.2818 7.1625 32.4443 16 32.4443C16.0938 32.4443 16.1875 32.4443 16.2812 32.438V19.9881H12.8438V15.9818H16.2812V13.0318C16.2812 9.61309 18.3688 7.75059 21.4188 7.75059C22.8813 7.75059 24.1375 7.85684 24.5 7.90684V11.4818H22.4C20.7437 11.4818 20.4188 12.2693 20.4188 13.4255V15.9755H24.3875L23.8687 19.9818H20.4188V31.8256C27.1063 29.9068 32 23.7505 32 16.4443Z"
                        fill="currentColor"
                      />
                    </g>
                    <defs>
                      <clipPath id="clip0_162_129">
                        <rect
                          width="32"
                          height="32"
                          fill="white"
                          transform="translate(0 0.444336)"
                        />
                      </clipPath>
                    </defs>
                  </svg>
                </a>
                <a href={landingData.footer.socialLinks[1].href}>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="33"
                    viewBox="0 0 32 33"
                    fill="none"
                    className="transition-color hover:text-primary cursor-pointer duration-300 hover:drop-shadow-md"
                  >
                    <g clipPath="url(#clip0_162_133)">
                      <path
                        d="M16 0.444336C7.16479 0.444336 0 7.60913 0 16.4443C0 25.2795 7.16479 32.4443 16 32.4443C24.8352 32.4443 32 25.2795 32 16.4443C32 7.60913 24.8352 0.444336 16 0.444336ZM23.3054 12.9194C23.3125 13.0769 23.3159 13.2351 23.3159 13.394C23.3159 18.2468 19.6221 23.8427 12.8669 23.843C10.793 23.843 8.86304 23.2351 7.23779 22.1933C7.52515 22.2273 7.81763 22.2441 8.11377 22.2441C9.83447 22.2441 11.418 21.6572 12.675 20.6721C11.0674 20.6423 9.71191 19.5805 9.24414 18.1213C9.46802 18.1643 9.69824 18.1877 9.93433 18.1877C10.2695 18.1877 10.5942 18.1425 10.9028 18.0583C9.22241 17.7219 7.95654 16.2368 7.95654 14.4585C7.95654 14.4419 7.95654 14.427 7.95703 14.4116C8.4519 14.6867 9.01782 14.8523 9.62036 14.8708C8.63428 14.2129 7.98608 13.0881 7.98608 11.8139C7.98608 11.1411 8.16797 10.5107 8.4834 9.96802C10.2944 12.1901 13.001 13.6516 16.0532 13.8051C15.9902 13.5361 15.9578 13.2558 15.9578 12.9677C15.9578 10.9404 17.6025 9.29566 19.6306 9.29566C20.687 9.29566 21.6411 9.74219 22.3113 10.456C23.1479 10.291 23.9336 9.98536 24.6433 9.5647C24.3687 10.4219 23.7866 11.1411 23.0283 11.5959C23.7712 11.507 24.4792 11.31 25.1372 11.0175C24.6458 11.7541 24.0225 12.4011 23.3054 12.9194Z"
                        fill="currentColor"
                      />
                    </g>
                    <defs>
                      <clipPath id="clip0_162_133">
                        <rect
                          width="32"
                          height="32"
                          fill="currentColor"
                          transform="translate(0 0.444336)"
                        />
                      </clipPath>
                    </defs>
                  </svg>
                </a>
                <a href={landingData.footer.socialLinks[2].href}>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="32"
                    height="33"
                    viewBox="0 0 32 33"
                    fill="none"
                    className="transition-color hover:text-primary cursor-pointer duration-300 hover:drop-shadow-md"
                  >
                    <g clipPath="url(#clip0_162_135)">
                      <path
                        d="M16 0.444336C7.16479 0.444336 0 7.60913 0 16.4443C0 25.2795 7.16479 32.4443 16 32.4443C24.8352 32.4443 32 25.2795 32 16.4443C32 7.60913 24.8352 0.444336 16 0.444336ZM11.3506 24.6318H7.45386V12.9084H11.3506V24.6318ZM9.40234 11.3076H9.37695C8.06934 11.3076 7.22363 10.4075 7.22363 9.28247C7.22363 8.13208 8.09521 7.25684 9.42822 7.25684C10.7612 7.25684 11.5815 8.13208 11.6069 9.28247C11.6069 10.4075 10.7612 11.3076 9.40234 11.3076ZM25.4014 24.6318H21.5051V18.3601C21.5051 16.7839 20.9409 15.7089 19.531 15.7089C18.4546 15.7089 17.8135 16.434 17.5317 17.134C17.4287 17.3845 17.4036 17.7346 17.4036 18.0849V24.6318H13.5071C13.5071 24.6318 13.5581 14.0083 13.5071 12.9084H17.4036V14.5683C17.9214 13.7695 18.8479 12.6333 20.9153 12.6333C23.479 12.6333 25.4014 14.3088 25.4014 17.9096V24.6318Z"
                        fill="currentColor"
                      />
                    </g>
                    <defs>
                      <clipPath id="clip0_162_135">
                        <rect
                          width="32"
                          height="32"
                          fill="currentColor"
                          transform="translate(0 0.444336)"
                        />
                      </clipPath>
                    </defs>
                  </svg>
                </a>
              </motion.div>
            </div>
          </div>
        </footer>
      </main>
    </>
  )
}
