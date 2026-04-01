# Docker Setup

Sakaloka-Universe uses Docker Compose for both local development and server deployment.

## Local Development

The root `docker-compose.yml` starts all planet containers for local development:

```bash
# Start all services
docker compose up -d

# Start only infrastructure (no API)
docker compose up -d surrealdb zenoh qdrant

# Check status
docker ps --filter "name=sakaloka"
```

### Services

| Service | Image | Port | Purpose |
|---------|-------|------|---------|
| `sakaloka-api` | Built from `apps/api/Dockerfile` | `127.0.0.1:53000` | Earth REST API |
| `mars-ockam` | Built from `apps/ockam/Dockerfile` | `127.0.0.1:54000` | Mars secure transport |
| `surrealdb` | `surrealdb/surrealdb:v3.0.4` | `127.0.0.1:58000` | Jupiter database |
| `zenoh` | `eclipse/zenoh:1.0.0` | `127.0.0.1:57447` | Saturn pub/sub |
| `qdrant` | `qdrant/qdrant:v1.13.6` | `127.0.0.1:56333` | Uranus vector search |

### Port Conflict Check

Before starting, always check for port conflicts:

```bash
bash scripts/check-ports.sh
```

## Server Deployment

The `deploy/` directory contains per-environment Docker Compose configurations:

```
deploy/
├── docker-compose.local.yml   # Local overrides (build from source)
├── docker-compose.dev.yml     # Dev environment (single source of truth)
├── docker-compose.sit.yml     # SIT environment
├── docker-compose.demo.yml    # Demo environment
├── docker-compose.prod.yml    # Production environment
├── deploy.sh                  # Deployment script
├── .env.template              # Environment variable template
└── config/
    ├── zenoh-users.demo.dict  # Zenoh ACL for demo
    └── zenoh-users.prod.dict  # Zenoh ACL for production
```

### Deploy to an Environment

```bash
cd deploy
cp .env.template .env
# Edit .env with environment-specific values

# Deploy to dev
bash deploy.sh dev

# Deploy to production
bash deploy.sh prod
```

### API Dockerfile

The API uses a multi-stage Docker build (`apps/api/Dockerfile`):

1. **Builder stage** — Rust image, compiles the workspace in release mode
2. **Runtime stage** — Minimal Debian image, copies only the binary + migrations
3. Migrations directory is copied to `/app/migrations/` and referenced via `MIGRATIONS_DIR`

### Server Infrastructure

The deployment server (`srv1294158`) runs shared infrastructure alongside Sakaloka:

| Service | Purpose | Notes |
|---------|---------|-------|
| `infra-nginx` | Reverse proxy | Routes to all apps |
| `infra-cloudflared` | Cloudflare tunnel | Public ingress |
| `infra-uptime-kuma` | Monitoring | Health check dashboard |
| `infra-{env}-postgres` | PostgreSQL | For other apps (not Sakaloka) |
| `infra-{env}-redis` | Redis | For other apps |
| `infra-{env}-minio` | MinIO | Object storage |
