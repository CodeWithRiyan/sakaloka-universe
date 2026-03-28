# Sakaloka-Universe — Setup Guide

This guide is the canonical day-to-day onboarding document for the current
monorepo.

## What you are running

- `apps/api` — Axum REST API
- `apps/venus` — React + Tauri operator app
- `libs/*` — shared Rust crates
- `libs/data/surql/*` — SurrealDB migrations and seed data

Important defaults:

- API local URL: `http://localhost:3000`
- API local base path: `http://localhost:3000/api`
- Venus local dev URL: `http://localhost:5173`
- SurrealDB dev credentials: `root / sakaloka-dev-password`
- Seed superadmin email: `superadmin@sakaloka.local`
- Seed superadmin password: `sakaloka-dev-01`

## Prerequisites

- Rust stable via `rustup`
- Docker and Docker Compose
- Node.js 22+
- `pnpm`
- Tauri system dependencies only if you want to run the desktop shell locally

## 1. Bootstrap local environment

From the repo root:

```bash
make bootstrap
```

This prepares:

- root `.env` from [`.env.example`](./.env.example)
- Venus `.env` from [`apps/venus/.env.example`](./apps/venus/.env.example)

If you prefer to do it manually:

```bash
cp .env.example .env
cp apps/venus/.env.example apps/venus/.env
```

## 2. Start local infrastructure

Check ports first:

```bash
make ports
```

Start the development infrastructure:

```bash
make infra-up
```

Inspect status:

```bash
make infra-ps
```

The main local service ports are:

| Service | URL / Port | Purpose |
|---|---|---|
| API | `http://localhost:3000` | Axum backend |
| SurrealDB | `ws://127.0.0.1:58000` | Primary database |
| Qdrant | `http://127.0.0.1:56333` | Vector store |
| Zenoh | `tcp/127.0.0.1:57447` | Messaging |
| Ockam | `http://127.0.0.1:54000` | Secure transport node |
| Venus | `http://localhost:5173` | Frontend dev server |

## 3. Run the API

```bash
make api-dev
```

What happens on startup:

- `.env` is loaded
- SurrealDB connection is established
- all `libs/data/surql/*.surql` migrations are applied in order
- seed data is ensured, including the global superadmin
- Axum starts on `PORT` or `3000`

Useful checks:

- health check: `GET /health`
- OpenAPI generation:

```bash
make openapi
```

## 4. Run Venus

Install dependencies once:

```bash
make venus-install
```

Run the web app:

```bash
make venus-dev
```

Run the Tauri shell:

```bash
make venus-tauri-dev
```

## 5. Sync API contracts into Venus

Whenever backend request/response contracts change:

```bash
make venus-sync-types
```

This refreshes:

- [`apps/venus/openapi.json`](./apps/venus/openapi.json)
- [`apps/venus/src/types/generated.ts`](./apps/venus/src/types/generated.ts)

## 6. Local quality gate

Backend only:

```bash
make check-backend
```

Venus only:

```bash
make check-venus
```

Full local CI-equivalent flow:

```bash
make ci
```

## 7. Authentication and authorization notes

- Authentication is handled by `apps/api` and `libs/secure`
- Authorization is permission-based from `role.permissions` in the database
- Route enforcement stays scope-based through `RequireScope`
- The `role` claim in JWT is informational; access comes from `scopes`

## 8. Venus application boundary

Venus is now an app-only surface.

- entry flow starts at `/login`
- dashboard lives under `/dashboard`
- the old landing page is quarantined and must not be reconnected to the app

Landing archive references:

- [`apps/venus/src/pages/landing/README.md`](./apps/venus/src/pages/landing/README.md)
- [`apps/venus/src/pages/landing/STRAPI_MIGRATION.md`](./apps/venus/src/pages/landing/STRAPI_MIGRATION.md)

## 9. Handy commands

```bash
make help
make ports
make infra-up
make api-dev
make venus-install
make venus-dev
make venus-sync-types
make ci
```

## 10. Current project shape

```text
sakaloka-universe/
├── apps/
│   ├── api/              # Axum REST API
│   ├── ockam/            # Ockam node
│   ├── venus/            # React + Tauri app
│   └── firmware/         # Excluded from workspace root
├── libs/
│   ├── core/             # Domain models and shared types
│   ├── data/             # Surreal/Qdrant/Zenoh integrations
│   ├── secure/           # JWT, RBAC, password, service auth
│   ├── ai/               # Local AI integrations
│   └── hal/              # Hardware abstraction
├── config/               # Shared runtime config
├── scripts/              # Operational helper scripts
├── docker-compose.yml
├── Makefile
└── .env.example
```
