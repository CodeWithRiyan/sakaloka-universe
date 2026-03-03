# =============================================================
# Sakaloka-Universe -- Dockerfile for Mars (apps/ockam)
# Multi-stage build: compile -> minimal runtime image
# =============================================================

# -- Stage 1: Build -------------------------------------------
FROM rust:1.93-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for layer caching
COPY Cargo.toml Cargo.lock* ./
COPY apps/ockam/Cargo.toml apps/ockam/
COPY libs/core/Cargo.toml libs/core/
COPY libs/secure/Cargo.toml libs/secure/
COPY libs/data/Cargo.toml libs/data/
COPY libs/ai/Cargo.toml libs/ai/
COPY libs/hal/Cargo.toml libs/hal/

# Stub source files to cache dependencies
RUN mkdir -p apps/ockam/src libs/core/src libs/secure/src libs/data/src libs/ai/src libs/hal/src \
    && echo 'fn main() {}' > apps/ockam/src/main.rs \
    && echo '' > libs/core/src/lib.rs \
    && echo '' > libs/secure/src/lib.rs \
    && echo '' > libs/data/src/lib.rs \
    && echo '' > libs/ai/src/lib.rs \
    && echo '' > libs/hal/src/lib.rs \
    && cargo build --release --package sakaloka-ockam 2>/dev/null || true \
    && rm -rf apps/ockam/src libs/core/src libs/secure/src libs/data/src libs/ai/src libs/hal/src

# Copy real source
COPY apps/ apps/
COPY libs/ libs/

RUN cargo build --release --package sakaloka-ockam

# -- Stage 2: Runtime -----------------------------------------
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/sakaloka-ockam .

ENV RUST_LOG=info
ENV OCKAM_PORT=4000

EXPOSE 4000

CMD ["./sakaloka-ockam"]
