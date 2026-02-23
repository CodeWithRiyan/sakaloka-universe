# 🚀 Sprint 1 — Scaffolding Setup Plan
**Sakaloka-Universe | PT Riyan Solusi Teknologi**
**Sprint Goal:** _Setup Docker environment and Cargo workspace with port isolation_
**Duration:** 2 Weeks | **Target Velocity:** 21 Story Points
**Rules Reference:** `.cursorrules` (Iron Curtain) + `README.md` (PRD v1.4)

---

## 📋 Sprint Backlog

| ID | User Story | Points | Priority | Status |
|---|---|---|---|---|
| US-01 | Docker Compose with 5xxxx port bindings | 5 | 🔴 Critical | ☐ Todo |
| US-02 | Cargo workspace with `apps/` + `libs/` crates | 8 | 🔴 Critical | ☐ Todo |
| US-03 | `/health` endpoint on Earth (Loco.rs) | 3 | 🟠 High | ☐ Todo |
| US-04 | Clippy + rustfmt CI checks | 5 | 🟠 High | ☐ Todo |

**Total: 21 pts**

---

## 🗂️ Detailed Task Breakdown

### US-01 — Docker Compose with 5xxxx Port Bindings (5 pts) 🔴

**Goal:** All infrastructure services start without port conflicts.

#### Tasks:

- [ ] **T-01a** Create `docker-compose.yml` in project root
  - `sakaloka-api` (Earth) → `127.0.0.1:53000:3000`
  - `surrealdb` (Jupiter) → `127.0.0.1:58000:8000`
  - `zenoh` (Saturn) → `127.0.0.1:57447:7447`
  - `qdrant` (Uranus) → `127.0.0.1:56333:6333`
  - **Important:** Never bind to `0.0.0.0` — always `127.0.0.1`
  - Mercury (RTIC) and Neptune (Burn) = no containers

- [ ] **T-01b** Add Docker health checks to all services
  - Each service must report `healthy` within 30s
  - `depends_on` must use `condition: service_healthy`

- [ ] **T-01c** Create `.env.example` with required secrets
  ```
  SAKALOKA_JWT_SECRET=<replace-with-secret>
  SAKALOKA_OCKAM_SECRET=<replace-with-secret>
  SURREALDB_USER=root
  SURREALDB_PASS=<replace-with-password>
  ```
  - `.env` must be in `.gitignore` — never commit secrets

- [ ] **T-01d** Write a port conflict pre-check script
  - `scripts/check-ports.sh` — scans for 53000, 58000, 57447, 56333
  - Exits with error code + message if any port is in use

**Acceptance Criteria:**
- `docker compose up -d` starts all 4 containers without errors
- `docker ps` shows all containers as `healthy`
- No port overlap with Residence, POS Damai, or Odoo

---

### US-02 — Cargo Workspace with `apps/` + `libs/` Crates (8 pts) 🔴

**Goal:** `cargo build --workspace` succeeds with all Iron Curtain directives enforced.

#### Directory Structure to Create:

```
sakaloka-universe/
├── Cargo.toml            ← workspace root
├── apps/
│   ├── api/              ← 🌍 Earth — Loco.rs API
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs   ← Iron Curtain + anyhow allowed here
│   └── web/              ← 🌸 Venus — React + Vite + Tauri 2.0
│       ├── package.json
│       └── src/
├── libs/
│   ├── core/             ← shared domain types & traits
│   │   ├── Cargo.toml
│   │   └── src/lib.rs    ← Iron Curtain directives
│   ├── data/             ← SurrealDB + Qdrant + Zenoh clients
│   │   ├── Cargo.toml
│   │   └── src/lib.rs    ← Iron Curtain directives
│   ├── secure/           ← ALL IAM: Argon2, JWT, RBAC, tokens, keys
│   │   ├── Cargo.toml
│   │   └── src/lib.rs    ← Iron Curtain directives
│   └── ai/               ← Burn model loading + embed()
│       ├── Cargo.toml
│       └── src/lib.rs    ← Iron Curtain directives
└── docs/
    ├── book.toml         ← MdBook config
    └── src/
        ├── SUMMARY.md
        └── adr/          ← Architecture Decision Records
```

#### Tasks:

- [ ] **T-02a** Create root `Cargo.toml` workspace manifest
  ```toml
  [workspace]
  resolver = "2"
  members = [
    "apps/api",
    "libs/core",
    "libs/data",
    "libs/secure",
    "libs/ai",
  ]

  [workspace.dependencies]
  # Pin shared dependency versions here
  tokio      = { version = "1", features = ["full"] }
  serde      = { version = "1", features = ["derive"] }
  thiserror  = "1"
  anyhow     = "1"
  tracing    = "0.1"
  ```

- [ ] **T-02b** Scaffold `libs/core` crate
  - `lib.rs` with Iron Curtain directives at top:
    ```rust
    #![deny(clippy::all)]
    #![deny(clippy::unwrap_used)]
    #![deny(clippy::expect_used)]
    #![deny(missing_docs)]
    #![forbid(unsafe_code)]

    //! Sakaloka-Universe shared domain types and traits.
    ```
  - Placeholder modules: `pub mod types;`, `pub mod traits;`, `pub mod constants;`
  - Each module must have a module-level doc comment

- [ ] **T-02c** Scaffold `libs/secure` crate (IAM: all auth lives here)
  - `lib.rs` with Iron Curtain directives
  - Stub module structure: `jwt`, `argon2`, `rbac`, `tokens`, `keys`, `newtypes`
  - No implementation yet — stubs with `todo!()` gated by `#[cfg(test)]` only
  - > **Rule:** `todo!()` is allowed ONLY inside `#[test]` — never in production paths

- [ ] **T-02d** Scaffold `libs/data` crate
  - Iron Curtain directives
  - Stub modules: `surreal`, `qdrant`, `zenoh`

- [ ] **T-02e** Scaffold `libs/ai` crate
  - Iron Curtain directives
  - Stub module: `embed`

- [ ] **T-02f** Scaffold `apps/api` crate (Earth — Loco.rs)
  - `main.rs` — `anyhow` allowed here (binary entry point)
  - Loco.rs scaffold with `cargo loco new` or manual setup
  - Wire `libs/core`, `libs/secure`, `libs/data` as local path dependencies

- [ ] **T-02g** Verify the build
  ```bash
  cargo build --workspace
  cargo clippy --workspace -- -D warnings
  cargo fmt --all -- --check
  ```

**Acceptance Criteria:**
- `cargo build --workspace` exits 0
- All Iron Curtain directives (`#![deny/forbid]`) present in every `lib.rs` / `main.rs`
- `cargo clippy --workspace -- -D warnings` exits 0 (zero warnings)
- `cargo fmt --all -- --check` exits 0

---

### US-03 — `/health` Endpoint on Earth (3 pts) 🟠

**Goal:** Uptime Kuma can monitor Earth's availability.

#### Tasks:

- [ ] **T-03a** Add `/health` route to `apps/api`
  ```rust
  // ✅ Correct — typed response, no unwrap
  pub async fn health() -> impl IntoResponse {
      Json(json!({ "status": "ok", "planet": "earth" }))
  }
  ```

- [ ] **T-03b** Wire route in Loco.rs router:
  ```rust
  router.route("/health", get(health));
  ```

- [ ] **T-03c** Verify with curl:
  ```bash
  curl -s http://127.0.0.1:53000/health
  # Expected: {"status":"ok","planet":"earth"}
  ```

- [ ] **T-03d** Configure Docker health check for `sakaloka-api`:
  ```yaml
  healthcheck:
    test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
    interval: 10s
    timeout: 5s
    retries: 3
  ```

**Acceptance Criteria:**
- `GET http://127.0.0.1:53000/health` returns `200 OK` with JSON body
- Uptime Kuma heartbeat turns green after service start

---

### US-04 — Clippy + rustfmt CI Checks (5 pts) 🟠

**Goal:** Code quality is enforced on every commit via CI pipeline.

#### Tasks:

- [ ] **T-04a** Create `.github/workflows/ci.yml` GitHub Actions workflow
  ```yaml
  name: Sakaloka CI — Iron Curtain

  on:
    push:
      branches: [main, develop]
    pull_request:
      branches: [main]

  env:
    CARGO_TERM_COLOR: always

  jobs:
    iron-curtain:
      name: Iron Curtain Checks
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4

        - name: Install Rust stable
          uses: dtolnay/rust-toolchain@stable
          with:
            components: clippy, rustfmt

        - name: Cache cargo registry
          uses: actions/cache@v3
          with:
            path: ~/.cargo/registry
            key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

        - name: cargo fmt --check
          run: cargo fmt --all -- --check

        - name: cargo clippy (deny warnings)
          run: cargo clippy --workspace --all-targets -- -D warnings

        - name: cargo test
          run: cargo test --workspace

        - name: cargo doc (deny missing_docs)
          run: cargo doc --workspace --no-deps
          env:
            RUSTDOCFLAGS: "-D missing_docs"
  ```

- [ ] **T-04b** Create `rustfmt.toml` in project root
  ```toml
  edition = "2021"
  max_width = 100
  use_field_init_shorthand = true
  use_try_shorthand = true
  ```

- [ ] **T-04c** Create `clippy.toml` in project root
  ```toml
  # Enforce Iron Curtain at clippy config level too
  msrv = "1.75.0"
  ```

- [ ] **T-04d** Add `.gitignore`
  ```
  /target
  .env
  *.local
  Cargo.lock   # keep this for apps, remove for libs
  ```
  > Note: Keep `Cargo.lock` committed for `apps/api` (binary). Add to `.gitignore` for `libs/` (libraries).

**Acceptance Criteria:**
- CI pipeline runs on every push to `main` and every PR
- Pipeline blocks merge if `cargo clippy -- -D warnings` fails
- Pipeline blocks merge if `cargo fmt --check` fails
- Pipeline blocks merge if `cargo test` fails

---

## 📐 Iron Curtain Checklist — Every File

Before marking any task Done, verify each new Rust file:

```
☐ #![deny(clippy::all)]           — present in lib.rs/main.rs
☐ #![deny(clippy::unwrap_used)]   — present
☐ #![deny(clippy::expect_used)]   — present
☐ #![deny(missing_docs)]          — present (libs only)
☐ #![forbid(unsafe_code)]         — present
☐ No .unwrap() calls
☐ No .expect() calls
☐ No panic!() in non-test code
☐ All public items have /// doc comments with # Errors + # Examples
☐ Error types use thiserror (libs) or anyhow (apps/api main.rs only)
```

---

## ✅ Definition of Done (DoD)

Per the PRD, a story is DONE only when ALL of the following are true:

- [ ] Code merged to `main` with no conflicts
- [ ] `cargo clippy --workspace -- -D warnings` → exits 0
- [ ] `cargo fmt --all -- --check` → exits 0
- [ ] `cargo test --workspace` → exits 0
- [ ] `cargo doc --workspace --no-deps` → exits 0 (no missing docs)
- [ ] Acceptance Criteria verified by Product Owner (PT Riyan)
- [ ] No new `.unwrap()` or `.expect()` introduced (Iron Curtain)
- [ ] Integration test added or updated where applicable
- [ ] Rustdoc updated for any new public APIs

---

## 🚦 Sprint 1 Risks

| Risk | Mitigation |
|---|---|
| Loco.rs scaffolding complexity | Use `cargo install loco-cli` + `loco new` to bootstrap; trim to minimal |
| SurrealDB Docker image config | Use official `surrealdb/surrealdb:latest` with `--log debug` during setup |
| Port conflicts on dev machine | Run `scripts/check-ports.sh` before `docker compose up` |
| Iron Curtain failures on first build | Scaffold with stubs only; add `#[allow]` only during scaffolding, remove before merge |

---

## 🔗 References

- PRD: `README.md` → Section 3 (Iron Curtain), Section 5.1 (Sprint 1 & 2)
- Rules: `.cursorrules` → Iron Curtain, Port Config, Workspace Structure
- Loco.rs docs: https://loco.rs
- SurrealDB docs: https://surrealdb.com/docs
- Zenoh Docker: https://github.com/eclipse-zenoh/zenoh/tree/main/rock

---

*Sakaloka-Universe Sprint 1 | Infrastructure: Ready | Security: Hardened | Code: Strict*
