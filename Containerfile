# SPDX-License-Identifier: PMPL-1.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath)

# Proof-of-Work Container Image
# Multi-stage build for Bevy game with optional Z3 verification
#
# Build targets:
#   default (headless)  - Server/test builds without display
#   full               - With Z3 verification + Steam + network
#
# Container runtime: podman (preferred), nerdctl, docker

# =============================================================================
# Stage 1: Build Rust binary
# =============================================================================
FROM docker.io/library/rust:1.85-slim-bookworm AS builder

# Install build dependencies for Bevy (headless mode)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libwayland-dev \
    libxkbcommon-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Bevy 0.18 requires Rust nightly (1.89+)
RUN rustup toolchain install nightly && rustup default nightly

WORKDIR /app

# Copy manifests first (for caching)
COPY Cargo.toml Cargo.lock ./

# Create dummy source for dependency caching
RUN mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    echo 'pub fn lib() {}' > src/lib.rs

# Build dependencies only (cached layer)
# Use --no-default-features to exclude z3-verify (default feature)
RUN cargo build --release --no-default-features --features headless && \
    rm -rf src target/release/deps/proof_of_work* target/release/proof-of-work*

# Copy actual source
COPY src ./src

# Build the real binary (headless by default, no z3-verify)
ARG FEATURES="headless"
RUN cargo build --release --no-default-features --features "$FEATURES"

# =============================================================================
# Stage 2: Runtime image
# =============================================================================
FROM cgr.dev/chainguard/wolfi-base:latest AS runtime

LABEL org.opencontainers.image.source="https://github.com/hyperpolymath/proof-of-work"
LABEL org.opencontainers.image.description="Proof-of-Work puzzle game with cryptographic verification"
LABEL org.opencontainers.image.licenses="PMPL-1.0-or-later"
LABEL org.opencontainers.image.authors="Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>"

# Install runtime dependencies (headless mode)
RUN apk add --no-cache \
    alsa-lib \
    ca-certificates \
    && adduser -D -u 1000 -s /sbin/nologin pow

USER pow

WORKDIR /app

# Copy binary from builder
COPY --from=builder --chown=pow:pow /app/target/release/proof-of-work ./

# Default: headless mode for testing
ENV BEVY_HEADLESS=1
ENV RUST_LOG=info

ENTRYPOINT ["./proof-of-work"]
