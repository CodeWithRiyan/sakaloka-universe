# Landing Page Quarantine

This folder is intentionally kept in the Venus repo as a content source for a
future centralized landing site managed through Strapi.

Current rules:

- Do not import this folder into Venus routes, layouts, or app shell code.
- Do not restore `/` to this page inside the Venus application.
- Keep the app entry experience focused on `/login` and `/register`.
- If content needs to be reused, migrate it to the centralized landing site
  instead of reconnecting it to the POS bundle.

Operational note:

- Venus must treat this folder as dormant content, not as an active app page.
- ESLint blocks alias imports from `@/pages/landing` to prevent accidental
  reintroduction into the compiled bundle.
