# =============================================================
# 🌌 Sakaloka-Universe — Dockerfile for Earth (apps/api)
# Multi-stage build: compile → minimal runtime image
# =============================================================

# ── Stage 1: Build ───────────────────────────────────────────
FROM rust:1.93-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for layer caching
COPY Cargo.toml Cargo.lock* ./
COPY apps/api/Cargo.toml apps/api/
COPY libs/core/Cargo.toml libs/core/
COPY libs/secure/Cargo.toml libs/secure/
COPY libs/data/Cargo.toml libs/data/
COPY libs/ai/Cargo.toml libs/ai/

# Stub source files to cache dependencies
RUN mkdir -p apps/api/src libs/core/src libs/secure/src libs/data/src libs/ai/src \
    && echo 'fn main() {}' > apps/api/src/main.rs \
    && echo '' > libs/core/src/lib.rs \
    && echo '' > libs/secure/src/lib.rs \
    && echo '' > libs/data/src/lib.rs \
    && echo '' > libs/ai/src/lib.rs \
    && cargo build --release --package sakaloka-api 2>/dev/null || true \
    && rm -rf apps/api/src libs/core/src libs/secure/src libs/data/src libs/ai/src

# Copy real source
COPY apps/ apps/
COPY libs/ libs/

# Build the real binary
RUN cargo build --release --package sakaloka-api

# ── Stage 2: Runtime ─────────────────────────────────────────
FROM debian:trixie-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/sakaloka-api .
COPY libs/data/surql/ /app/migrations/

ENV RUST_LOG=info
ENV MIGRATIONS_DIR=/app/migrations

EXPOSE 3000

CMD ["./sakaloka-api"]
