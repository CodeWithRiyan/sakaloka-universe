# Deployment Guide

Sakaloka-Universe deploys to a VPS using Docker Compose with per-environment `.env` files.

## Environments

| Environment | Trigger | Image Tag | Purpose |
|-------------|---------|-----------|---------|
| `dev` | Push to `dev` branch | `dev` | Development testing |
| `sit` | Push to `main` branch | `sit` | System integration testing |
| `demo` | Push to `demo` branch | `demo` | Client demonstrations |
| `prod` | Push to `prod` branch | `prod` | Production |

## Deployment Files

```
deploy/
├── docker-compose.yml    # Single compose file (all environments)
├── deploy.sh             # Deployment helper script
├── .env.template         # Environment variable template
└── .env.{env}            # Per-environment config (not committed)
```

## Deploy Steps

### 1. Prepare Environment

```bash
cd deploy
cp .env.template .env.dev
# Edit .env.dev with your values
```

### 2. Run Deployment

```bash
bash deploy.sh <environment>

# Examples:
bash deploy.sh dev
bash deploy.sh prod
```

The script:
1. Loads `.env.{env}` for the target environment
2. Pulls the latest images from `git.sakaloka.id` registry
3. Starts services with `docker compose --env-file .env.{env} -p sakaloka-{env} up -d`
4. Runs health checks

### 3. Verify

```bash
curl -s http://127.0.0.1:<port>/health
docker ps --filter "name=sakaloka"
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `REGISTRY` | Yes | `git.sakaloka.id` | Container registry host |
| `API_IMAGE` | Yes | `riyan/sakaloka-api` | API image name |
| `WEB_IMAGE` | Yes | `riyan/sakaloka-pos` | Web image name |
| `IMAGE_TAG` | Yes | `dev` | Environment tag |
| `API_HOST_PORT` | Yes | `53000` | API host port |
| `WEB_HOST_PORT` | Yes | `53040` | Web host port |
| `NETWORK` | Yes | `net-dev` | Docker network name |
| `SAKALOKA_JWT_SECRET` | Yes | *(none)* | JWT signing key (min 32 chars) |
| `DATABASE_URL` | Yes | *(none)* | PostgreSQL connection URL |
| `CORS_ORIGINS` | No | `http://localhost:5173` | Allowed CORS origins |
| `RUST_LOG` | No | `info` | Log level |

## Server Architecture

The production server runs:

```
Internet
  |
  v
Cloudflare Tunnel (infra-cloudflared)
  |
  v
Nginx (infra-nginx) -- reverse proxy
  |
  +-- sakaloka-api:{env}     (Axum REST API)
  +-- sakaloka-pos:{env}     (Venus POS Web)
  +-- sakaloka-postgres      (PostgreSQL)
```

## CI/CD Integration

Forgejo Actions automatically:
1. Builds Docker images on push to `dev`/`main`/`demo`/`prod`
2. Pushes to `git.sakaloka.id/riyan/sakaloka-api:{env}` and `sakaloka-pos:{env}`
3. Deploys via volume-mounted compose on the VPS

See [CI/CD Pipeline](./ci.md) for workflow details.
