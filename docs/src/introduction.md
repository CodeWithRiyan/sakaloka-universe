# Sakaloka-Universe

**Sakaloka-Universe** is a cloud-native, AI-powered knowledge management platform built by
**PT Riyan Solusi Teknologi**.

It uses a **planet-model architecture** where every component has a single,
well-defined responsibility. Communication between planets is strictly typed and authenticated.

## Quick Start

```bash
# 1. Check for port conflicts
bash scripts/check-ports.sh

# 2. Start all planet containers
docker compose up -d

# 3. Verify all planets are healthy
docker ps --filter "name=sakaloka"

# 4. Test the Earth API
curl -s http://127.0.0.1:53000/health | python3 -m json.tool
```

## Architecture Principles

1. **Iron Curtain** — no `.unwrap()`, no `unsafe`, no panics in production code
2. **Single Responsibility** — each planet owns exactly one domain
3. **Security First** — all auth lives in `libs/secure`, enforced at the router level
4. **Port Isolation** — all services bind to `127.0.0.1:5xxxx` only

## Planet Map

| Planet | Role | Port |
|--------|------|------|
| 🌍 Earth | REST API (Axum) | `127.0.0.1:53000` |
| 🌸 Venus | React + Tauri desktop | *(localhost:5173)* |
