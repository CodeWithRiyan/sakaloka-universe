# Sakaloka-Universe — Setup Guide

## Prerequisites

- **Rust** (stable) — [rustup.rs](https://rustup.rs)
- **Docker & Docker Compose** — for Postgres, SurrealDB, Qdrant, Zenoh
- **Node.js 20+** & **pnpm** — only for Venus (Tauri desktop app)

## 1. Clone & Environment

```bash
git clone https://github.com/CodeWithRiyan/sakaloka-universe.git
cd sakaloka-universe

# Create environment file
cp .env.example .env
```

Edit `.env` and set a real JWT secret (min 32 chars):

```bash
# Generate a secret
openssl rand -base64 48
```

## 2. Start Infrastructure

Start **only the databases/services** (no app containers needed for local dev):

```bash
docker compose up -d postgres surrealdb qdrant zenoh
```

Wait for healthy status:

```bash
docker compose ps
```

### Service Ports

| Service    | Planet  | External Port         | Purpose              |
|------------|---------|----------------------|----------------------|
| Postgres   | —       | `127.0.0.1:55432`    | Relational DB (API)  |
| SurrealDB  | Jupiter | `127.0.0.1:58000`    | Graph/document DB    |
| Qdrant     | Uranus  | `127.0.0.1:56333`    | Vector search        |
| Zenoh      | Saturn  | `127.0.0.1:57447`    | Pub/sub messaging    |

## 3. Run the API (Earth)

```bash
# Run with auto-migration (reads config/development.yaml)
cargo run -p sakaloka-api
```

The API starts on `http://localhost:3000` with auto-migration enabled.

### Useful API commands

```bash
# Run with Loco CLI (shows all available commands)
cargo run -p sakaloka-api -- --help

# Run database migrations manually
cargo run -p sakaloka-api -- db migrate

# Reset database (drop + re-migrate)
cargo run -p sakaloka-api -- db reset

# Generate OpenAPI spec
cargo run --bin openapi
```

### API Endpoints

- `GET  /health` — health check
- `POST /api/auth/register` — register user
- `POST /api/auth/login` — login
- `GET  /api/products` — list products (requires auth)
- Swagger UI at `/swagger-ui` (when running)

## 4. Run Venus (Tauri Desktop App)

Venus has **two parts**: a React frontend (from the `sakapos` repo) and a Rust Tauri shell.

### First-time setup

The frontend code lives in a separate git repo inside `apps/venus/`:

```bash
cd apps/venus

# If frontend files are not present, pull them:
git init   # (skip if already initialized)
git remote add origin https://github.com/CodeWithRiyan/sakapos.git
git fetch origin
git checkout main

# Install frontend dependencies
pnpm install
```

### Run in development

```bash
# From the project root — starts both React dev server + Tauri window
cd apps/venus/src-tauri
cargo tauri dev
```

This runs `pnpm dev` (React on `localhost:5173`) and opens the Tauri desktop window.

### Build for production

```bash
cd apps/venus/src-tauri
cargo tauri build
```

## 5. Run Tests

```bash
# Set required env var
export SAKALOKA_JWT_SECRET="test-secret-at-least-32-characters-long!"
export DATABASE_URL="postgres://sakaloka:sakaloka_dev@127.0.0.1:55432/sakaloka_dev"

# All tests
cargo test --workspace

# Single crate
cargo test -p sakaloka-secure

# Single test by name
cargo test -p sakaloka-api -- health
```

## 6. Lint & Format

```bash
# Check formatting
cargo fmt --all -- --check

# Fix formatting
cargo fmt --all

# Clippy (must pass with zero warnings)
cargo clippy --workspace --all-targets -- -D warnings

# Doc coverage (must be 100%)
RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps
```

## 7. Full Docker Stack (optional)

To run **everything** in containers (API + all services):

```bash
docker compose up -d
```

| Container      | Port                  |
|----------------|-----------------------|
| sakaloka-api   | `127.0.0.1:53000`    |
| mars-ockam     | `127.0.0.1:54000`    |
| postgres       | `127.0.0.1:55432`    |
| qdrant         | `127.0.0.1:56333`    |
| zenoh          | `127.0.0.1:57447`    |
| surrealdb      | `127.0.0.1:58000`    |

## Project Structure

```
sakaloka-universe/
├── apps/
│   ├── api/                  # Earth — Loco.rs REST API
│   │   ├── config/           #   YAML configs (dev/test/prod)
│   │   ├── migration/        #   SeaORM database migrations
│   │   └── src/
│   │       ├── controllers/  #   Route handlers
│   │       ├── models/       #   SeaORM entities
│   │       ├── views/        #   Response schemas (utoipa)
│   │       └── middleware/   #   Auth middleware
│   ├── ockam/                # Mars — Ockam secure transport
│   ├── venus/                # Venus — Tauri desktop app
│   │   └── src-tauri/        #   Rust shell (tracked here)
│   │   └── src/              #   React frontend (from sakapos repo)
│   └── firmware/             # Mercury — ARM Cortex-M4 (excluded)
├── libs/
│   ├── core/                 # Domain types, traits
│   ├── secure/               # JWT, RBAC, password hashing
│   ├── data/                 # DB clients (Surreal, Qdrant, Zenoh)
│   ├── ai/                   # Burn inference engine
│   └── hal/                  # Hardware abstraction (no_std)
├── config/                   # Zenoh config files
├── docker-compose.yml
├── .env.example
└── CLAUDE.md                 # AI assistant instructions
```

## Two-Repo Workflow

| Repo | URL | Contains |
|------|-----|----------|
| `sakaloka-universe` | `CodeWithRiyan/sakaloka-universe` | Rust monorepo + Tauri shell |
| `sakapos` | `CodeWithRiyan/sakapos` | React frontend |

`apps/venus/` has its own `.git` pointing to `sakapos`. The monorepo's `.gitignore` ignores all frontend files — only `src-tauri/` is tracked by the monorepo.

```bash
# Pull latest frontend changes
cd apps/venus
git pull

# Push Rust changes (from project root)
cd ../..
git add . && git commit -m "..." && git push
```
