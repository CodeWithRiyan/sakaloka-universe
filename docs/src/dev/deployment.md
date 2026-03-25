# Deployment Guide

Sakaloka-Universe deploys to a VPS using Docker Compose with per-environment configurations.

## Environments

| Environment | Trigger | Image Tag | Purpose |
|-------------|---------|-----------|---------|
| `dev` | Push to `dev` branch | `sakaloka-api:dev` | Development testing |
| `sit` | Push to `main` branch | `sakaloka-api:sit` | System integration testing |
| `demo` | Tag `v*-demo` | `sakaloka-api:demo` | Client demonstrations |
| `prod` | Tag `v*` (no suffix) | `sakaloka-api:prod` | Production |

## Deployment Files

```
deploy/
├── docker-compose.base.yml   # Shared service definitions
├── docker-compose.dev.yml    # Dev overrides (ports, volumes)
├── docker-compose.sit.yml    # SIT overrides
├── docker-compose.demo.yml   # Demo overrides
├── docker-compose.prod.yml   # Prod overrides
├── docker-compose.local.yml  # Local testing
├── deploy.sh                 # Deployment script
├── .env.template             # Environment variable template
└── config/
    ├── zenoh-users.demo.dict # Zenoh ACL for demo
    └── zenoh-users.prod.dict # Zenoh ACL for production
```

## Deploy Steps

### 1. Prepare Environment

```bash
cd deploy
cp .env.template .env
# Edit .env with your values
```

### 2. Run Deployment

```bash
bash deploy.sh <environment>

# Examples:
bash deploy.sh dev
bash deploy.sh prod
```

The script:
1. Loads `.env` for the target environment
2. Pulls the latest image from GHCR
3. Starts services with `docker compose -f base.yml -f {env}.yml up -d`
4. Runs health checks

### 3. Verify

```bash
curl -s http://127.0.0.1:<port>/health
docker ps --filter "name=sakaloka"
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `SAKALOKA_JWT_SECRET` | Yes | *(none)* | JWT signing key (min 32 chars) |
| `SURREALDB_URL` | No | `ws://127.0.0.1:58000` | SurrealDB WebSocket URL |
| `SURREALDB_USER` | Yes | *(none)* | SurrealDB username |
| `SURREALDB_PASS` | Yes | *(none)* | SurrealDB password |
| `PORT` | No | `3000` | API listen port |
| `CORS_ORIGINS` | No | `http://localhost:5173,http://localhost:1420` | Allowed CORS origins (comma-separated) |
| `MIGRATIONS_DIR` | No | *(auto from CARGO_MANIFEST_DIR)* | Path to SurrealQL migration files |
| `RUST_LOG` | No | `info` | Log level |
| `QDRANT_URL` | No | `http://127.0.0.1:56333` | Qdrant connection URL |
| `QDRANT_MASTER_KEY` | No | *(none)* | Qdrant API key |
| `OCKAM_PORT` | No | `4000` | Ockam service port |
| `SAKALOKA_OCKAM_SECRET` | No | *(none)* | Ockam transport secret |

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
  +-- sakaloka-surrealdb     (SurrealDB)
  +-- sakaloka-zenoh         (Pub/sub)
  +-- sakaloka-qdrant        (Vector search)
```

## CI/CD Integration

GitHub Actions automatically:
1. Builds the Docker image on push/tag
2. Pushes to `ghcr.io/codewithriyan/sakaloka-api:{env}`
3. SSHs into the server and runs `deploy.sh {env}`

See [CI/CD Pipeline](./ci.md) for workflow details.
