# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 hyperpolymath

# Proof-of-Work Container Image
# Multi-stage build for Bevy game with optional Z3 verification
#
# Build targets:
#   default (headless)  - Server/test builds without display
#   full               - With Z3 verification + Steam + network
#
# Container runtime support: nerdctl, podman, docker

# =============================================================================
# Stage 1: Build Rust binary
# =============================================================================
FROM rust:1.88-slim-bookworm AS builder

# Install build dependencies (g++/cmake/make required for z3-sys build from source)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    g++ \
    cmake \
    make \
    libz3-dev \
    libasound2-dev \
    libudev-dev \
    libwayland-dev \
    libxkbcommon-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first (for caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy source for dependency caching
RUN mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    echo 'pub fn lib() {}' > src/lib.rs

# Build dependencies only (cached layer)
RUN cargo build --release --features headless && \
    rm -rf src target/release/deps/proof_of_work* target/release/proof-of-work*

# Copy actual source
COPY src ./src

# Build the real binary
ARG FEATURES="headless"
RUN cargo build --release --features "$FEATURES"

# =============================================================================
# Stage 2: Runtime image
# =============================================================================
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.source="https://github.com/hyperpolymath/proof-of-work"
LABEL org.opencontainers.image.description="Proof-of-Work puzzle game library"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"

# Install runtime dependencies (libz3-4 is runtime lib for z3)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libz3-4 \
    libasound2 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 pow
USER pow

WORKDIR /app

# Copy binary from builder
COPY --from=builder --chown=pow:pow /app/target/release/proof-of-work ./

# Default: headless mode for testing
ENV BEVY_HEADLESS=1
ENV RUST_LOG=info

ENTRYPOINT ["./proof-of-work"]
