# CI/CD Pipeline

Sakaloka-Universe uses GitHub Actions for continuous integration and deployment.

## Workflows

### CI: Iron Curtain (`ci.yml`)

**Trigger:** Push to `main`/`dev`, pull requests

Runs all Iron Curtain quality gates:

| Step | Command | Purpose |
|------|---------|---------|
| Format | `cargo fmt --all -- --check` | Enforce consistent formatting |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | Deny all Clippy warnings |
| Build | `cargo build --workspace` | Verify compilation |
| Test | `cargo test --workspace` | Run all tests (unit + integration + doc) |
| Docs | `RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps` | 100% doc coverage |

The CI job starts a **SurrealDB container** for integration tests that need a database.

Environment variables set in CI:
- `SAKALOKA_JWT_SECRET` — from GitHub secrets

### CD: Build & Release (`cd.yml`)

**Trigger:** Push to `dev`/`main`, version tags (`v*`), manual dispatch

| Job | Description |
|-----|-------------|
| `resolve-env` | Determines target environment from trigger context |
| `build-desktop` | Cross-platform Tauri build (Linux, macOS Apple Silicon, macOS Intel) |
| `build-web` | POS Web (Venus) Docker build with per-env config, push to GHCR |
| `build-api` | Multi-stage Docker build for the API, push to GHCR |
| `deploy-api` | SSH deploy API + infra to server, update Nginx, restart containers |
| `deploy-web` | SSH deploy POS Web container to server |

#### Environment Resolution

| Trigger | Environment |
|---------|-------------|
| Push to `dev` | `dev` |
| Push to `main` | `sit` |
| Tag `v*-demo` | `demo` |
| Tag `v*` (no suffix) | `prod` |
| Manual dispatch | User-selected |

#### Docker Image Tags

Images are pushed to GHCR:

```
ghcr.io/codewithriyan/sakaloka-api:{env}     # Rust API
ghcr.io/codewithriyan/sakaloka-pos:{env}     # POS Web (Venus)
```

## Running CI Locally

Before pushing, run the Iron Curtain checks locally:

```bash
# All checks in order
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps
```

## Required Secrets

### Repository-Level Secrets

| Secret | Used By | Description |
|--------|---------|-------------|
| `VPS_HOST` | CD deploy | VPS hostname or IP address |
| `VPS_SSH_USER` | CD deploy | SSH username on VPS |
| `VPS_SSH_KEY` | CD deploy | SSH private key (Ed25519, full PEM content) |

### Environment-Level Secrets

Create these for each GitHub Environment (`dev`, `sit`, `demo`, `prod`):

| Secret | Used By | Description |
|--------|---------|-------------|
| `SAKALOKA_JWT_SECRET` | CI tests, CD deploy | JWT signing key (min 32 chars, unique per env) |
| `SURREALDB_USER` | CD deploy | SurrealDB root username (default: `root`) |
| `SURREALDB_PASS` | CD deploy | SurrealDB root password (unique per env) |
| `QDRANT_MASTER_KEY` | CD deploy | Qdrant API key (unique per env) |
| `ZENOH_USER` | CD deploy | Zenoh auth username (default: `earth`) |
| `ZENOH_PASS` | CD deploy | Zenoh auth password (must match argon2id hash in `zenoh-users.{env}.dict`) |

### VPS Deployment Layout

```
/srv/apps/sakaloka/{env}/
  .env                         # Auto-generated on first deploy from secrets
  deploy.sh                    # Deploy helper script
  docker-compose.{env}.yml     # Environment-specific compose (single source of truth)
  config/
    nginx-sakaloka.conf        # Nginx reverse proxy (copied to /srv/apps/infra/config/)
    zenoh.{env}.json5           # Zenoh router config
    zenoh-users.{env}.dict      # Zenoh auth dict

/srv/apps/infra/config/
  sakaloka.conf                # Symlinked/copied from deploy, auto-reloaded by CD
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
