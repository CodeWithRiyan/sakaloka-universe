# ADR-002: Iron Curtain Compiler Rules

**Status:** ✅ Accepted
**Date:** 2025-02
**Author:** PT Riyan Solusi Teknologi

## Context

In a security-critical platform handling user identity and AI-augmented data, panics and
undefined behavior are unacceptable. We needed a strategy to prevent common Rust footguns
at scale across a growing team.

## Decision

Every `lib.rs` and `main.rs` in the workspace must include:

```rust
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
```

These are **compiler errors**, not warnings. CI blocks any merge that introduces a violation.

Additionally:
- Libraries use `thiserror` for structured error types
- The binary entry point (`apps/api/main.rs`) may use `anyhow`
- `.unwrap()` is only permitted inside `#[test]` blocks

## Consequences

**Positive:**
- Panics are structurally impossible in production paths
- All error cases are explicit — callers know what can fail
- Public API is always documented (enforced by `missing_docs`)
- New contributors are guided by compiler errors, not code review

**Negative:**
- More boilerplate for simple functions (mitigated by `thiserror` macros)
- Slightly steeper learning curve for Rust newcomers
- Doc comments required on every public item (acceptable — good practice)
