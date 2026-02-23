# Iron Curtain Rules

The **Iron Curtain** is Sakaloka-Universe's mandatory compiler-enforced code quality standard.
Every Rust source file must comply. CI blocks merges on any violation.

## Mandatory Directives

Every `lib.rs` and `main.rs` must start with:

```rust
#![deny(clippy::all)]           // All Clippy lints are errors
#![deny(clippy::unwrap_used)]   // No .unwrap() in production code
#![deny(clippy::expect_used)]   // No .expect() in production code
#![deny(missing_docs)]          // All public items must be documented
#![forbid(unsafe_code)]         // Zero unsafe Rust, forever
```

## Error Handling Rules

| Location | Pattern |
|----------|---------|
| `libs/` crates | `thiserror` — typed, structured errors |
| `apps/api` main.rs | `anyhow` — allowed only at the binary entry point |
| Test code | `.unwrap()` allowed inside `#[test]` blocks only |

## What Is Forbidden

| Pattern | Reason |
|---------|--------|
| `.unwrap()` | Panics. Use `?` operator or pattern match. |
| `.expect("msg")` | Still panics. Use `Result` propagation. |
| `panic!()` | Crashes the process. Use `Result::Err`. |
| `unsafe {}` | Memory unsafety. Forbidden forever. |
| `todo!()` | Panics. Allowed only in `#[cfg(test)]`. |
| Raw `String` inputs | Use newtype wrappers from `libs/secure::newtypes`. |

## Documentation Rules (AC-03)

Every public function must have:
- A `///` doc comment
- A `# Errors` section (if `Result` return)
- A `# Examples` section with a working `doctest`

```rust
/// Hashes a password using Argon2id.
///
/// # Errors
///
/// Returns [`SecureError::HashFailed`] on internal failure.
///
/// # Examples
///
/// ```rust,no_run
/// use sakaloka_secure::{argon2::hash_password, newtypes::Password};
/// let pw = Password::new("correct-horse-battery-staple").unwrap();
/// let hash = hash_password(&pw).unwrap();
/// ```
pub fn hash_password(password: &Password) -> Result<String, SecureError> { ... }
```
