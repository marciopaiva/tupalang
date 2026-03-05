#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

EN_LIST="$(mktemp)"
ES_LIST="$(mktemp)"
PT_LIST="$(mktemp)"
trap 'rm -f "$EN_LIST" "$ES_LIST" "$PT_LIST"' EXIT

find docs/en -type f -name "*.md" | sed "s|^docs/en/||" | sort > "$EN_LIST"
find docs/es -type f -name "*.md" | sed "s|^docs/es/||" | sort > "$ES_LIST"
find docs/pt-br -type f -name "*.md" | sed "s|^docs/pt-br/||" | sort > "$PT_LIST"

missing_es="$(comm -23 "$EN_LIST" "$ES_LIST" || true)"
missing_pt="$(comm -23 "$EN_LIST" "$PT_LIST" || true)"

if [[ -n "$missing_es" ]]; then
  echo "Missing files in docs/es (present in docs/en):"
  echo "$missing_es"
  exit 1
fi

if [[ -n "$missing_pt" ]]; then
  echo "Missing files in docs/pt-br (present in docs/en):"
  echo "$missing_pt"
  exit 1
fi

en_latest="$(grep -m1 -E '^## [0-9]+\.[0-9]+\.[0-9]+' docs/en/releases/changelog.md || true)"
es_latest="$(grep -m1 -E '^## [0-9]+\.[0-9]+\.[0-9]+' docs/es/releases/changelog.md || true)"
pt_latest="$(grep -m1 -E '^## [0-9]+\.[0-9]+\.[0-9]+' docs/pt-br/releases/changelog.md || true)"

if [[ -z "$en_latest" || -z "$es_latest" || -z "$pt_latest" ]]; then
  echo "Unable to read latest release heading in one or more changelog files."
  exit 1
fi

if [[ "$en_latest" != "$es_latest" ]]; then
  echo "Changelog version mismatch: EN vs ES"
  echo "EN: $en_latest"
  echo "ES: $es_latest"
  exit 1
fi

if [[ "$en_latest" != "$pt_latest" ]]; then
  echo "Changelog version mismatch: EN vs PT-BR"
  echo "EN: $en_latest"
  echo "PT: $pt_latest"
  exit 1
fi

echo "docs parity check: ok"
