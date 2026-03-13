#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

RUST_VERSION="${RUST_VERSION:-1.93}"
TUPA_RUST_BUILDER_IMAGE="${TUPA_RUST_BUILDER_IMAGE:-tupalang-base-rust-builder:${RUST_VERSION}}"
TUPA_RUST_RUNTIME_IMAGE="${TUPA_RUST_RUNTIME_IMAGE:-tupalang-base-rust-runtime:bookworm}"
TUPA_CLI_IMAGE="${TUPA_CLI_IMAGE:-tupalang-cli:local}"

if [[ "${CONTAINER_ENGINE:-}" == "docker" ]]; then
  ENGINE="docker"
elif [[ "${CONTAINER_ENGINE:-}" == "podman" ]]; then
  ENGINE="podman"
elif command -v docker >/dev/null 2>&1; then
  ENGINE="docker"
elif command -v podman >/dev/null 2>&1; then
  ENGINE="podman"
else
  echo "ERROR: docker or podman not found" >&2
  exit 1
fi

"$ENGINE" build \
  -f "$ROOT_DIR/docker/tupa-cli.Dockerfile" \
  --build-arg RUST_VERSION="$RUST_VERSION" \
  --build-arg TUPA_RUST_BUILDER_IMAGE="$TUPA_RUST_BUILDER_IMAGE" \
  --build-arg TUPA_RUST_RUNTIME_IMAGE="$TUPA_RUST_RUNTIME_IMAGE" \
  -t "$TUPA_CLI_IMAGE" \
  "$ROOT_DIR"

echo "Built $TUPA_CLI_IMAGE with $ENGINE"
