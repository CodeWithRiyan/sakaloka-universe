# ADR-001: Monorepo with Cargo Workspace

**Status:** ✅ Accepted
**Date:** 2025-02
**Author:** PT Riyan Solusi Teknologi

## Context

Sakaloka-Universe has multiple components (API server, desktop app)
that share common types and business logic. We needed to decide between:

1. **Separate repositories** — one repo per service
2. **Monorepo with Cargo workspace** — single repo, multiple crates

## Decision

We chose a **Cargo workspace monorepo** with strict `apps/` and `libs/` separation:

```
sakaloka-universe/
├── apps/
│   ├── api/       ← 🌍 Earth — Axum REST API
│   └── venus/     ← 🌸 Venus — React + Tauri desktop
└── libs/
    ├── core/      ← domain types, traits, constants
    ├── secure/    ← IAM (ONLY place for auth logic)
    └── data/      ← PostgreSQL client via SQLx
```

## Consequences

**Positive:**
- Single `cargo build --workspace` compiles everything
- Shared dependency versions pinned in root `Cargo.toml`
- Iron Curtain rules enforced across all crates in one CI run
- Refactoring across `libs/` is atomic (no cross-repo coordination)

**Negative:**
- Larger `target/` directory (mitigated by sccache in CI)
- All team members clone the full monorepo (acceptable at current size)
