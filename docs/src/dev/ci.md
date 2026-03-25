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
| `build-api` | Multi-stage Docker build for the API, push to GHCR |
| `deploy-api` | SSH deploy to server, pull image, restart containers |

#### Environment Resolution

| Trigger | Environment |
|---------|-------------|
| Push to `dev` | `dev` |
| Push to `main` | `sit` |
| Tag `v*-demo` | `demo` |
| Tag `v*` (no suffix) | `prod` |
| Manual dispatch | User-selected |

#### Docker Image Tags

Images are pushed to `ghcr.io/codewithriyan/sakaloka-api:{env}`:

```
ghcr.io/codewithriyan/sakaloka-api:dev
ghcr.io/codewithriyan/sakaloka-api:sit
ghcr.io/codewithriyan/sakaloka-api:demo
ghcr.io/codewithriyan/sakaloka-api:prod
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

| Secret | Used By | Description |
|--------|---------|-------------|
| `SAKALOKA_JWT_SECRET` | CI tests | JWT signing key (min 32 chars) |
| `GHCR_TOKEN` | CD build | GitHub Container Registry push token |
| `DEPLOY_SSH_KEY` | CD deploy | SSH key for server access |
| `DEPLOY_HOST` | CD deploy | Server hostname |
| `DEPLOY_USER` | CD deploy | SSH username |
