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

latest_release_heading() {
  local file="$1"
  grep -m1 -E '^## [0-9]+\.[0-9]+\.[0-9]+' "$file" || true
}

release_body() {
  local file="$1"
  local ver="$2"
  awk -v ver="$ver" '
    $0 ~ ("^## " ver "(\\s|$)") {in_block=1; next}
    in_block && $0 ~ /^## [0-9]+\.[0-9]+\.[0-9]+/ {exit}
    in_block {print}
  ' "$file"
}

contains_or_fail() {
  local haystack="$1"
  local needle="$2"
  local msg="$3"
  if ! grep -Fq "$needle" <<<"$haystack"; then
    echo "$msg"
    exit 1
  fi
}

en_latest="$(latest_release_heading docs/en/releases/changelog.md)"
es_latest="$(latest_release_heading docs/es/releases/changelog.md)"
pt_latest="$(latest_release_heading docs/pt-br/releases/changelog.md)"

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

release_ver="$(echo "$en_latest" | sed -E 's/^## ([0-9]+\.[0-9]+\.[0-9]+).*/\1/')"

en_body="$(release_body docs/en/releases/changelog.md "$release_ver")"
es_body="$(release_body docs/es/releases/changelog.md "$release_ver")"
pt_body="$(release_body docs/pt-br/releases/changelog.md "$release_ver")"

contains_or_fail "$en_body" "### Delivered Scope" "Missing required changelog section in EN: Delivered Scope"
contains_or_fail "$en_body" "### Engineering and CI Completed" "Missing required changelog section in EN: Engineering and CI Completed"
contains_or_fail "$en_body" "### Validation Snapshot (workspace)" "Missing required changelog section in EN: Validation Snapshot"
contains_or_fail "$en_body" "### Technical Debt" "Missing required changelog section in EN: Technical Debt"

contains_or_fail "$es_body" "### Alcance Entregado" "Missing required changelog section in ES: Alcance Entregado"
contains_or_fail "$es_body" "### Ingenier" "Missing required changelog section in ES: Ingenieria y CI"
contains_or_fail "$es_body" "### Snapshot de Validaci" "Missing required changelog section in ES: Snapshot de Validacion"
contains_or_fail "$es_body" "### Deuda T" "Missing required changelog section in ES: Deuda Tecnica"

contains_or_fail "$pt_body" "### Escopo Entregue" "Missing required changelog section in PT-BR: Escopo Entregue"
contains_or_fail "$pt_body" "### Engenharia e CI Entregues" "Missing required changelog section in PT-BR: Engenharia e CI"
contains_or_fail "$pt_body" "### Snapshot de Valida" "Missing required changelog section in PT-BR: Snapshot de Validacao"
contains_or_fail "$pt_body" "### D" "Missing required changelog section in PT-BR: Debito Tecnico"

for lang in en es pt-br; do
  for file in \
    "docs/$lang/reference/spec.md" \
    "docs/$lang/releases/changelog.md" \
    "docs/$lang/releases/release_guide.md" \
    "docs/$lang/releases/release_cut_checklist.md"
  do
    if [[ ! -f "$file" ]]; then
      echo "Missing critical doc file: $file"
      exit 1
    fi
  done

  if ! grep -q 'changelog.md' "docs/$lang/releases/release_guide.md"; then
    echo "Missing changelog link in docs/$lang/releases/release_guide.md"
    exit 1
  fi

  if ! grep -q 'release_cut_checklist.md' "docs/$lang/releases/release_guide.md"; then
    echo "Missing release_cut_checklist link in docs/$lang/releases/release_guide.md"
    exit 1
  fi
done

echo "docs parity check: ok"
