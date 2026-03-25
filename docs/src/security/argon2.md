# Argon2 Hashing

Password hashing and verification uses **Argon2id** via the `argon2` crate, implemented
in `libs/secure::argon2`.

## Why Argon2id?

Argon2id is the recommended password hashing algorithm by OWASP. It combines:
- **Argon2i** — resistant to side-channel attacks
- **Argon2d** — resistant to GPU cracking

The `id` variant provides the best of both.

## API

### Hashing

```rust
use sakaloka_secure::argon2::hash_password;
use sakaloka_secure::newtypes::Password;

let pw = Password::new("correct-horse-battery-staple")?;
let hash = hash_password(&pw)?;
// hash is a PHC-format string like:
// $argon2id$v=19$m=19456,t=2,p=1$salt$hash
```

### Verification

```rust
use sakaloka_secure::argon2::verify_password;
use sakaloka_secure::newtypes::Password;

let pw = Password::new("correct-horse-battery-staple")?;
let valid = verify_password(&pw, &stored_hash)?;
// valid: true if password matches, false otherwise
```

## Security Properties

- **Password newtype** — `Password` struct enforces minimum 12 characters at construction
- **Cleared on drop** — the `Password` struct zeros its memory when dropped
- **No Clone** — `Password` does not implement `Clone`, preventing accidental copies that
  extend the plaintext lifetime in memory
- **PHC format** — hash strings are self-describing (algorithm, parameters, salt, hash)
- **Salt per hash** — each password gets a unique random salt

## Usage in Auth Flow

1. **Registration**: `Password::new(raw)` -> `hash_password(&pw)` -> store hash in `user.password_hash`
2. **Login**: `Password::new(raw)` -> `verify_password(&pw, &stored_hash)` -> `true` / `false`

The `Password` type is the **only** way to access the plaintext. It must only be used
by `libs/secure::argon2`. No other module should call `password.as_str()`.
