#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  echo "Usage: ./scripts/release-verify.sh <version>"
  echo "Example: ./scripts/release-verify.sh 0.8.0"
  exit 1
fi

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([.-]rc[.-]?[0-9]+)?$ ]]; then
  echo "Invalid version format: $VERSION"
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "Working tree is not clean. Commit/stash changes before release verification."
  git status --short
  exit 1
fi

TAG="v$VERSION"
if git rev-parse -q --verify "refs/tags/$TAG" >/dev/null; then
  echo "Tag already exists: $TAG"
  exit 1
fi

for lang in en es pt-br; do
  changelog="docs/$lang/releases/changelog.md"
  if ! grep -q "^## $VERSION" "$changelog"; then
    echo "Missing changelog heading in $changelog: ## $VERSION"
    exit 1
  fi
done

./scripts/docs-parity-check.sh
./scripts/ci-local.sh
./scripts/vipertrade-smoke.sh

echo "release verify: ok for $VERSION"
