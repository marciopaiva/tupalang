ARG RUST_VERSION=1.93
ARG TUPA_RUST_BUILDER_IMAGE=tupalang-base-rust-builder:${RUST_VERSION}
ARG TUPA_RUST_RUNTIME_IMAGE=tupalang-base-rust-runtime:bookworm
FROM ${TUPA_RUST_BUILDER_IMAGE} AS builder

WORKDIR /usr/src/tupalang

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --release --package tupa-cli

FROM ${TUPA_RUST_RUNTIME_IMAGE}

COPY --from=builder /usr/src/tupalang/target/release/tupa /usr/local/bin/tupa

RUN useradd -m -u 1000 -U tupa
USER tupa
ENTRYPOINT ["tupa"]
CMD ["--help"]
