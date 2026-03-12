ARG RUST_VERSION=1.93
FROM rust:${RUST_VERSION}-slim-bookworm

ENV PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1

RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config \
  libssl-dev \
  python3 \
  python3-dev \
  ca-certificates \
  curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/tupalang
