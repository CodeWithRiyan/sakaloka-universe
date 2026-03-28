# Venus

Venus is the operator application for Sakaloka-Universe.

## Scope

- React + Vite frontend
- Tauri desktop shell
- app-only surface for authentication and dashboard workflows
- no marketing landing page in the active bundle

## Local environment

Create the env file if it does not exist:

```bash
cp .env.example .env
```

Default local values:

```env
VITE_BASE_URL=http://localhost:3000
VITE_API_BASE_URL=http://localhost:3000/api
```

## Commands

From the repository root:

```bash
make venus-install
make venus-dev
make venus-tauri-dev
make venus-sync-types
make check-venus
```

Or directly inside this folder:

```bash
pnpm install --frozen-lockfile
pnpm dev
pnpm tauri dev
pnpm run generate:types
pnpm lint
pnpm build
```

## Contract sync

Venus consumes:

- `openapi.json`
- `src/types/generated.ts`

Always regenerate them after backend contract changes:

```bash
pnpm run generate:types
```

## Application boundary

Venus starts from `/login` and `/register`.

The previous landing page is intentionally quarantined:

- [`src/pages/landing/README.md`](./src/pages/landing/README.md)
- [`src/pages/landing/STRAPI_MIGRATION.md`](./src/pages/landing/STRAPI_MIGRATION.md)

Do not reconnect the landing page into the Venus router.
