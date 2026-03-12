#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

RUST_VERSION="${RUST_VERSION:-1.93}"
TUPA_RUST_BUILDER_IMAGE="${TUPA_RUST_BUILDER_IMAGE:-tupalang-base-rust-builder:${RUST_VERSION}}"
TUPA_RUST_RUNTIME_IMAGE="${TUPA_RUST_RUNTIME_IMAGE:-tupalang-base-rust-runtime:bookworm}"
TUPA_CLI_IMAGE="${TUPA_CLI_IMAGE:-tupalang-cli:local}"

podman build \
  -f "$ROOT_DIR/docker/tupa-cli.Dockerfile" \
  --build-arg RUST_VERSION="$RUST_VERSION" \
  --build-arg TUPA_RUST_BUILDER_IMAGE="$TUPA_RUST_BUILDER_IMAGE" \
  --build-arg TUPA_RUST_RUNTIME_IMAGE="$TUPA_RUST_RUNTIME_IMAGE" \
  -t "$TUPA_CLI_IMAGE" \
  "$ROOT_DIR"

echo "Built $TUPA_CLI_IMAGE"
