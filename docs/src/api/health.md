# `GET /health`

Earth's only unauthenticated endpoint. Used by Uptime Kuma and Docker health checks.

## Request

```http
GET /health HTTP/1.1
Host: localhost:3000
```

No authentication required. No request body.

## Response

**`200 OK`**

```json
{
  "status": "ok",
  "planet": "earth",
  "service": "sakaloka-api",
  "version": "0.1.0"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | `string` | Always `"ok"` when the service is running |
| `planet` | `string` | Always `"earth"` — identifies this API instance |
| `service` | `string` | Cargo package name |
| `version` | `string` | Cargo package version |

## Docker Health Check

```yaml
healthcheck:
  test: ["CMD", "curl", "-sf", "http://localhost:3000/health"]
  interval: 10s
  timeout: 5s
  retries: 5
  start_period: 15s
```

## Uptime Kuma Configuration

- **Type:** HTTP(s)
- **URL:** `http://127.0.0.1:53000/health`
- **Heartbeat interval:** 60 seconds
- **Expected status:** `200`
