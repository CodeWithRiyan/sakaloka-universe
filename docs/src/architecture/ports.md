# Port Allocation

All Sakaloka-Universe services bind to `127.0.0.1` on ports in the `5xxxx` range.

**Never use `0.0.0.0`** — this is enforced in `docker-compose.yml`.

## Port Table

| Planet | Service | External Port | Internal Port |
|--------|---------|--------------|--------------|
| 🌍 Earth | Axum REST API | `127.0.0.1:53000` | `3000` |

## Port Conflict Check

Before `docker compose up`, run:

```bash
bash scripts/check-ports.sh
```

This checks each Sakaloka port and distinguishes between:
- 🐳 **SAKALOKA** — port held by our own running container (safe)
- ❌ **CONFLICT** — port held by a foreign process (abort!)
- ✅ **FREE** — port available (safe to start)

## Why `127.0.0.1`?

Binding to `127.0.0.1` (loopback) means the service is only reachable from the same machine.
This is a critical security boundary. Production containers sit behind a reverse proxy,
never exposed to raw internet traffic.
