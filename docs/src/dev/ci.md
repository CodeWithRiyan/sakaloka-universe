# CI/CD Pipeline

Sakaloka-Universe uses Forgejo Actions for continuous integration and deployment.

## Workflows

### CI: Iron Curtain (`.forgejo/workflows/ci.yaml`)

**Trigger:** Push to `main`/`dev`, pull requests

Runs all Iron Curtain quality gates in parallel jobs:

| Job | Command | Purpose |
|-----|---------|---------|
| `rustfmt` | `cargo fmt --all -- --check` | Enforce consistent formatting |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` | Deny all Clippy warnings |
| `test` | `cargo build --workspace && cargo test --workspace && cargo doc` | Build, test, and verify 100% doc coverage |
| `venus` | `pnpm install && pnpm lint && pnpm build` | Lint and build Venus frontend |

The test job starts a **PostgreSQL container** for integration tests.

Environment variables set in CI:
- `SAKALOKA_JWT_SECRET` — from repository secrets
- `DATABASE_URL` — PostgreSQL connection string

### CD: Build & Deploy (`.forgejo/workflows/cd.yaml`)

**Trigger:** Push to `dev`/`main`/`demo`/`prod` branches

| Job | Description |
|-----|-------------|
| `setup` | Maps branch to environment tag |
| `build-api` | Multi-stage Docker build for the API, push to Forgejo registry |
| `build-web` | Venus Docker build, push to Forgejo registry |
| `deploy` | Pull images and restart services on VPS via volume mount |

#### Environment Resolution

| Trigger | Environment |
|---------|-------------|
| Push to `dev` | `dev` |
| Push to `main` | `sit` |
| Push to `demo` | `demo` |
| Push to `prod` | `prod` |

#### Docker Image Tags

Images are pushed to the Forgejo container registry:

```
git.sakaloka.id/riyan/sakaloka-api:{env}     # Rust API
git.sakaloka.id/riyan/sakaloka-pos:{env}     # POS Web (Venus)
```

## Running CI Locally

Before pushing, run the Iron Curtain checks locally:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps
```

## Required Secrets

### Repository-Level Secrets (Forgejo)

| Secret | Used By | Description |
|--------|---------|-------------|
| `REGISTRY_TOKEN` | CD build | Forgejo container registry token |
| `DEPLOY_PATH` | CD deploy | Path to deploy directory on VPS (e.g. `/srv/apps/sakaloka`) |
| `SAKALOKA_JWT_SECRET` | CI tests | JWT signing key (min 32 chars) |

### Per-Environment Configuration

Each environment uses a `.env.{env}` file on the VPS (not stored in secrets):

| Variable | Description |
|----------|-------------|
| `SAKALOKA_JWT_SECRET` | JWT signing key (unique per env) |
| `DATABASE_URL` | PostgreSQL connection URL |
| `CORS_ORIGINS` | Allowed CORS origins |
| `RUST_LOG` | Log level |

### VPS Deployment Layout

```
/srv/apps/sakaloka/
  docker-compose.yml             # Single compose file (all envs)
  .env.dev                       # Dev environment variables
  .env.sit                       # SIT environment variables
  .env.demo                      # Demo environment variables
  .env.prod                      # Prod environment variables
```

### Nginx Domain Mapping

| Environment | API Domain | API Port | POS Domain | POS Port |
|-------------|-----------|----------|------------|----------|
| Dev | `api-dev.sakaloka.id` | 60000 | `dev-pos.sakaloka.id` | 60040 |
| SIT | `api-sit.sakaloka.id` | 60100 | `sit-pos.sakaloka.id` | 60140 |
| Demo | `api-demo.sakaloka.id` | 60200 | `demo-pos.sakaloka.id` | 60240 |
| Prod | `api.sakaloka.id` | 60300 | `pos.sakaloka.id` | 60340 |

### Cloudflare Tunnel Setup

Add these hostnames to the Cloudflare tunnel configuration for `sakaloka.id`:

| Hostname | Service |
|----------|---------|
| `api-dev.sakaloka.id` | `http://infra-nginx:80` |
| `api-sit.sakaloka.id` | `http://infra-nginx:80` |
| `api-demo.sakaloka.id` | `http://infra-nginx:80` |
| `api.sakaloka.id` | `http://infra-nginx:80` |
| `dev-pos.sakaloka.id` | `http://infra-nginx:80` |
| `sit-pos.sakaloka.id` | `http://infra-nginx:80` |
| `demo-pos.sakaloka.id` | `http://infra-nginx:80` |
| `pos.sakaloka.id` | `http://infra-nginx:80` |
