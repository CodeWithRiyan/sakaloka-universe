# ADR-003: Dual JWT Strategy (User + Service)

**Status:** ✅ Accepted
**Date:** 2025-02
**Author:** PT Riyan Solusi Teknologi

## Context

Sakaloka-Universe has two distinct token consumers:
1. **End users** (via Venus desktop) — authenticate against Earth's HTTP API
2. **Internal planets** — Earth must call Jupiter/Uranus/Saturn with credentials

A single JWT type would either over-grant service tokens to users or under-restrict
inter-planet calls.

## Decision

We use **two JWT types**, both signed with the same `SAKALOKA_JWT_SECRET` (HS256):

### User JWT (`UserClaims`)
- Issued **by Earth** after successful login
- Stored in **React state only** (memory — never localStorage)
- `aud: ["sakaloka:earth"]` — only valid on Earth
- TTL: **15 minutes**
- Contains: `sub` (user ID), `role`, `scopes`, `session_id`, `jti`

### Service JWT (`ServiceClaims`)
- Auto-issued by `ServiceTokenStore` (proactive cache + rotation)
- Valid for **one target planet only** (`aud: ["sakaloka:jupiter"]` etc.)
- TTL: **5 minutes** (refreshed 60 seconds before expiry)
- Contains: `sub` (planet identity), `service_scopes`, `planet`

## Consequences

**Positive:**
- A Jupiter token is rejected by Saturn even with a valid signature (`aud` mismatch)
- JTI blocklist prevents replay attacks
- Proactive refresh means no planet ever calls another with an expired token
- Clear separation of user and service identities in logs and audit trails

**Negative:**
- Requires `ServiceTokenStore` to run in every planet that calls another
- Slightly more complex than a single token type (accepted — security justifies it)
