# ADR-004: 5xxxx Port Allocation Scheme

**Status:** ✅ Accepted
**Date:** 2025-02
**Author:** PT Riyan Solusi Teknologi

## Context

The development machine runs multiple projects simultaneously:
- **Odoo** (ERP) — uses ports 8069, 8072
- **POS Damai** — uses ports 3000, 5432, 9000
- **Residence** — uses ports 3001, 3002, 5433

All standard service ports (3000, 8000, 6333, 7447) are potentially occupied.

## Decision

All Sakaloka-Universe services bind to **`127.0.0.1:5xxxx`** using Docker port mapping:

```
127.0.0.1:53000  → Earth API        (5 prefix + 3000)
127.0.0.1:54000  → Mars / Ockam     (5 prefix + 4000)
127.0.0.1:57447  → Saturn / Zenoh   (5 prefix + 7447)
127.0.0.1:56333  → Uranus / Qdrant  (5 prefix + 6333)
127.0.0.1:58000  → Jupiter / SurrealDB (5 prefix + 8000)
```

Rules:
1. **Always `127.0.0.1`** — never `0.0.0.0` (loopback only)
2. **Always 5xxxx** — easy to recognize as Sakaloka ports
3. **`scripts/check-ports.sh`** — verifies no foreign conflicts before startup

## Consequences

**Positive:**
- Zero conflicts with existing projects on the development machine
- Port numbers are self-documenting (5 + standard port)
- Loopback-only binding prevents accidental external exposure

**Negative:**
- Port numbers differ from standard — developers must remember the `5xxxx` scheme
  (mitigated by the port table in documentation and the check-ports script)
