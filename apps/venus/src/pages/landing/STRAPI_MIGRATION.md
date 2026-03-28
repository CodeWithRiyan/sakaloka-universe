# Strapi Migration Guide

This landing folder is an archive. Venus must remain an app-only surface with
entry points at `/login` and `/register`.

## Source of truth

- Content source: `content.ts`
- Legacy layout reference: `index.tsx`
- Archived marketing layout shell: `../../components/layouts/public`

Use `content.ts` as the canonical source when moving content into Strapi. Do
not copy text manually from JSX unless the text is missing from `content.ts`.

## Recommended Strapi content model

### Single type: `landing-page`

Suggested sections:

- `hero`
- `features`
- `benefits`
- `paymentIntegrations`
- `pricing`
- `faq`
- `supportCta`
- `finalCta`
- `seo`
- `jsonLd`

### Collection types

Suggested reusable collections/components:

- `feature-item`
- `benefit-item`
- `benefit-point`
- `integration-item`
- `pricing-plan`
- `faq-item`
- `cta-block`
- `social-link`

## Section mapping

- `home`
  Source: `landingData.hero`
  Assets: `/hero-device.png`, `/user-1.webp`
  CTA: `/register`, consultation CTA placeholder
- `features`
  Source: `landingData.features`
- `benefits`
  Source: `landingData.benefits`
  Assets:
  - `/svg/supermarket-workers.svg`
  - `/svg/time-management.svg`
  - `/svg/analysis.svg`
- `payment-integration`
  Source: `landingData.integrations`
  Assets:
  - `/svg/payment/dana.svg`
  - `/svg/payment/gopay.svg`
  - `/svg/payment/shopeepay.svg`
  - `/svg/payment/qris.svg`
  - `/svg/payment/ovo.svg`
  - `/svg/payment/linkaja.svg`
  - `/svg/payment/doku.svg`
- `pricing`
  Source: `landingData.pricing`
- `faq`
  Source: `landingData.faqs`
- `bantuan`
  Source: `landingData.supportCta`
  Asset: `/phone.png`
- `footer`
  Source: `landingData.footer`

## SEO and structured data

Move these fields from `content.ts` into Strapi-managed SEO fields:

- `seoMetadata`
- `metaTags`
- `linkTags`
- `organizationSchema`
- `softwareSchema`
- `faqSchema`
- `webPageSchema`

## Migration notes

- Some CTA destinations are still placeholders (`href="#"` or `href="##"`).
  Resolve them before publishing the centralized landing site.
- Keep `/register` only if the future marketing site is still allowed to send
  users directly into Venus onboarding.
- Preserve payment/support/media assets in a shared media library before
  deleting this archive.
- Once the centralized landing site is live, this folder can either remain as a
  historical archive or be removed in a dedicated cleanup PR.
