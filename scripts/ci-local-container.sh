#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMAGE_NAME="${IMAGE_NAME:-tupalang-ci-local:0.8.1-dev}"
STRICT_LINKS="${CI_LOCAL_STRICT_LINKS:-0}"
SKIP_BUILD="${CI_LOCAL_SKIP_BUILD:-0}"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

usage() {
  cat <<EOF
Usage: ./scripts/ci-local-container.sh

Run the local CI flow inside a Docker container that matches the tools
required by GitHub Actions.

Environment:
  IMAGE_NAME            Container image tag (default: $IMAGE_NAME)
  CI_LOCAL_STRICT_LINKS Forward strict links mode to ci-local.sh (default: $STRICT_LINKS)
  CI_LOCAL_SKIP_BUILD   Skip docker build when set to 1 (default: $SKIP_BUILD)

Examples:
  ./scripts/ci-local-container.sh
  CI_LOCAL_STRICT_LINKS=1 ./scripts/ci-local-container.sh
  CI_LOCAL_SKIP_BUILD=1 ./scripts/ci-local-container.sh
EOF
}

require_cmd() {
  local cmd="$1"
  local hint="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo -e "${RED}missing:${NC} '$cmd' (${hint})" >&2
    exit 1
  fi
}

if [[ "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ $# -ne 0 ]]; then
  echo -e "${RED}error:${NC} unexpected arguments" >&2
  usage >&2
  exit 1
fi

require_cmd docker "install Docker Desktop or Docker Engine"

echo -e "${GREEN}Local CI in container${NC}"
echo

if [[ "$SKIP_BUILD" != "1" ]]; then
  echo -e "${YELLOW}==>${NC} docker build ${IMAGE_NAME}"
  docker build -t "$IMAGE_NAME" -f "$ROOT_DIR/docker/ci-local.Dockerfile" "$ROOT_DIR"
  echo -e "${GREEN}ok${NC} docker build ${IMAGE_NAME}"
  echo
fi

echo -e "${YELLOW}==>${NC} docker run ${IMAGE_NAME}"
docker run --rm \
  -v "$ROOT_DIR:/workspace" \
  -w /workspace \
  -e CI_LOCAL_STRICT_LINKS="$STRICT_LINKS" \
  "$IMAGE_NAME" \
  ./scripts/ci-local.sh
echo -e "${GREEN}ok${NC} docker run ${IMAGE_NAME}"
