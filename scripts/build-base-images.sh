#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BASE_DIR="$ROOT_DIR/docker/base"

RUST_VERSION="${RUST_VERSION:-1.93}"
TUPA_RUST_BUILDER_IMAGE="${TUPA_RUST_BUILDER_IMAGE:-tupalang-base-rust-builder:${RUST_VERSION}}"
TUPA_RUST_RUNTIME_IMAGE="${TUPA_RUST_RUNTIME_IMAGE:-tupalang-base-rust-runtime:bookworm}"

echo "Building Tupalang base images..."
echo "  TUPA_RUST_BUILDER_IMAGE=$TUPA_RUST_BUILDER_IMAGE"
echo "  TUPA_RUST_RUNTIME_IMAGE=$TUPA_RUST_RUNTIME_IMAGE"

podman build -f "$BASE_DIR/rust-builder.Dockerfile" --build-arg RUST_VERSION="$RUST_VERSION" -t "$TUPA_RUST_BUILDER_IMAGE" "$ROOT_DIR"
podman build -f "$BASE_DIR/rust-runtime.Dockerfile" -t "$TUPA_RUST_RUNTIME_IMAGE" "$ROOT_DIR"

echo "Done."
