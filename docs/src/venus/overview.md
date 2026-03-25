# Venus Desktop App

Venus is the **Tauri + React** desktop POS/ERP application. It provides the user-facing
interface for Sakaloka-Universe, communicating with the Earth API for all data operations.

## Tech Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Desktop shell | Tauri | 2.x |
| Frontend framework | React | 19.1 |
| Language | TypeScript | 5.8 |
| Build tool | Vite | 7.1 |
| Styling | Tailwind CSS | 4.1 |
| Components | Radix UI + shadcn/ui | latest |
| State management | Redux Toolkit + RTK Query | 2.8 |
| Routing | React Router | 7.8 |
| Forms | React Hook Form + Zod | 7.62 / 4.1 |
| Data tables | TanStack Table | 8.21 |
| Icons | Lucide React | 0.542 |
| Animations | Motion (Framer Motion) | 12.23 |

## App Identity

| Property | Value |
|----------|-------|
| Product name | Sakaloka POS |
| Identifier | `id.sakaloka.pos` |
| Window size | 1280x800 (resizable) |
| Dev server | `http://localhost:5173` |
| Targets | Windows, macOS, Linux |

## Architecture

Venus uses a **thin Tauri shell** — the Rust side only initializes the webview window and
the `tauri_plugin_opener` plugin. All business logic, API calls, and state management
live in the React/TypeScript frontend.

```
Venus Desktop
├── src-tauri/           # Rust: Tauri shell (minimal)
│   ├── src/lib.rs       # tauri::Builder + plugin_opener
│   └── tauri.conf.json  # Window config, CSP, bundle settings
└── src/                 # TypeScript: React frontend
    ├── pages/           # Route components (lazy-loaded)
    ├── components/      # UI components (shadcn/ui + custom)
    ├── store/           # Redux + RTK Query API slices
    ├── hooks/           # Custom React hooks
    ├── lib/             # Auth, utils, helpers
    ├── types/           # TypeScript types (inc. generated from OpenAPI)
    └── routes/          # React Router config
```

## Quick Start

```bash
cd apps/venus

# Install frontend dependencies
pnpm install

# Start dev server (frontend only)
pnpm dev

# Start with Tauri desktop shell
pnpm tauri:dev

# Build for production
pnpm tauri:build
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_BASE_URL` | `http://localhost:3000` | Frontend base URL |
| `VITE_API_BASE_URL` | `https://dev-api-erp.riyansolusi.com/api` | Earth API URL |
