FROM rust:bookworm

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    make \
    nodejs \
    npm \
    pkg-config \
    python3 \
    python3-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy
RUN cargo install lychee
RUN npm install -g markdownlint-cli@0.41.0

WORKDIR /workspace
