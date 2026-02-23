# Getting Started

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | ≥ 1.93 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Docker | ≥ 28.0 | [docs.docker.com](https://docs.docker.com/get-docker/) |
| Docker Compose | ≥ 2.39 | Bundled with Docker Desktop |

## Setup

```bash
# 1. Clone the repository
git clone https://github.com/riyan-id/sakaloka-universe.git
cd sakaloka-universe

# 2. Copy environment variables
cp .env.example .env
# Edit .env and fill in your secrets (SAKALOKA_JWT_SECRET must be ≥ 32 chars)

# 3. Check for port conflicts
bash scripts/check-ports.sh

# 4. Start planet containers
docker compose up -d surrealdb zenoh qdrant

# 5. Verify all planets are healthy
docker ps --filter "name=sakaloka"

# 6. Build the workspace
cargo build --workspace

# 7. Run the API locally (for development)
SAKALOKA_JWT_SECRET="your-secret" cargo run -p sakaloka-api

# 8. Test the health endpoint
curl -s http://127.0.0.1:3000/health | python3 -m json.tool
```

## Running CI Checks Locally

```bash
# Iron Curtain — run before every commit
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps
```
