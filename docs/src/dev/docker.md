# Docker Setup

Sakaloka-Universe uses Docker Compose for both local development and server deployment.

## Local Development

The root `docker-compose.yml` starts services for local development:

```bash
docker compose up -d
docker ps --filter "name=sakaloka"
```

### Services

| Service | Image | Port | Purpose |
|---------|-------|------|---------|
| `sakaloka-api` | Built from `apps/api/Dockerfile` | `127.0.0.1:53000` | Earth REST API |

### Port Conflict Check

Before starting, always check for port conflicts:

```bash
bash scripts/check-ports.sh
```

## Server Deployment

The `deploy/` directory contains a single Docker Compose file with per-environment `.env` files:

```
deploy/
├── docker-compose.yml     # Single compose file (all environments)
├── deploy.sh              # Deployment helper script
└── .env.template          # Environment variable template
```

### Deploy to an Environment

```bash
cd deploy
cp .env.template .env.dev
# Edit .env.dev with environment-specific values

bash deploy.sh dev
bash deploy.sh prod
```

### API Dockerfile

The API uses a multi-stage Docker build (`apps/api/Dockerfile`):

1. **Builder stage** — Rust image, compiles the workspace in release mode
2. **Runtime stage** — Minimal Debian image, copies only the binary + migrations
3. Migrations directory is copied to `/app/migrations/` and referenced via `MIGRATIONS_DIR`

### Venus Dockerfile

The Venus POS Web uses a multi-stage build (`apps/venus/Dockerfile`):

1. **Builder stage** — Node image, builds with pnpm
2. **Runtime stage** — Nginx Alpine, serves the SPA
3. Healthcheck at `/healthz`

### Server Infrastructure

The deployment server runs shared infrastructure alongside Sakaloka:

| Service | Purpose | Notes |
|---------|---------|-------|
| `infra-nginx` | Reverse proxy | Routes to all apps |
| `infra-cloudflared` | Cloudflare tunnel | Public ingress |
| `infra-uptime-kuma` | Monitoring | Health check dashboard |
