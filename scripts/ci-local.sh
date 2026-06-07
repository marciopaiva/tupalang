#!/usr/bin/env bash
# ci-local.sh — Executa localmente as mesmas validações do GitHub Actions CI
#
# Variáveis de ambiente (opcionais):
#   CI_LOCAL_STRICT_LINKS=1    — Falha se links quebrados forem encontrados (lychee)
#   CI_LOCAL_CHECK_COMMIT=1    — Valida a mensagem do último commit (Conventional Commits)
#   CI_LOCAL_CHECK_PUBLISH=1   — Executa `cargo publish --dry-run` para crates core
#   CI_LOCAL_SKIP_GOLDEN=1     — Pula a verificação de golden outputs
#
# Exemplos:
#   ./scripts/ci-local.sh                   # Executa todas as etapas (goldens opcional)
#   CI_LOCAL_STRICT_LINKS=1 ./scripts/ci-local.sh
#   CI_LOCAL_CHECK_COMMIT=1 ./scripts/ci-local.sh
#   CI_LOCAL_CHECK_PUBLISH=1 ./scripts/ci-local.sh
#
# Este script executa:
#   - cargo fmt --check
#   - cargo clippy (workspace, all targets, deny warnings)
#   - cargo test (workspace)
#   - markdownlint (todos .md)
#   - docs-parity-check.sh (paridade EN/ES/PT-BR)
#   - vipertrade-smoke.sh (pipeline smoke test)
#   - lychee (link check)
#   - Golden output check (compara gerado vs committed)
#   - Commit message convention (opcional)
#   - Publish dry-run (opcional)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

STRICT_LINKS="${CI_LOCAL_STRICT_LINKS:-0}"
CHECK_COMMIT="${CI_LOCAL_CHECK_COMMIT:-0}"
CHECK_PUBLISH="${CI_LOCAL_CHECK_PUBLISH:-0}"
SKIP_GOLDEN="${CI_LOCAL_SKIP_GOLDEN:-0}"

run_step() {
  local title="$1"
  shift
  echo -e "${YELLOW}==>${NC} ${title}"
  "$@"
  echo -e "${GREEN}ok${NC} ${title}"
  echo
}

require_cmd() {
  local cmd="$1"
  local hint="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo -e "${RED}missing:${NC} '$cmd' (${hint})" >&2
    return 1
  fi
}

install_markdownlint() {
  if command -v markdownlint >/dev/null 2>&1; then
    return 0
  fi
  echo "Installing markdownlint-cli..."
  # Try npm install, may require sudo
  if npm install -g markdownlint-cli@0.41.0 2>/dev/null; then
    return 0
  fi
  # If failed, try with sudo
  if sudo npm install -g markdownlint-cli@0.41.0 2>/dev/null; then
    return 0
  fi
  echo -e "${RED}failed to install markdownlint-cli${NC}"
  return 1
}

install_lychee() {
  if command -v lychee >/dev/null 2>&1; then
    return 0
  fi
  echo "Installing lychee..."
  # Check Rust version to pick compatible lychee
  local rust_version
  rust_version=$(rustc --version 2>/dev/null | awk '{print $2}' | cut -d. -f1-2)
  if [[ -n "$rust_version" ]] && [[ "$(printf '%s\n' "$rust_version" "1.88" | sort -V | head -n1)" == "1.88" ]]; then
    # Rust >= 1.88, install latest
    cargo install lychee
  else
    # Rust < 1.88, install older compatible version
    echo "Rust $rust_version detected, installing lychee 0.18.1 (compatible)"
    cargo install lychee --version 0.18.1
  fi
}

run_lychee_docs() {
  local -a base=(
    --verbose
    --no-progress
    --scheme http
    --scheme https
    --exclude "^https://img.shields.io/"
    --exclude "^https://github.com/.+\\.wiki\\.git$"
    "**/*.md"
  )

  if lychee --help | grep -q -- "--exclude-mail"; then
    lychee "${base[@]}" --exclude-mail
  else
    lychee "${base[@]}" --exclude "^mailto:"
  fi
}

run_golden_check() {
  echo -e "${YELLOW}==>${NC} Checking golden outputs (dry-run)"

  # Skip if legacy tupa-cli is not available (Rust-DSL era)
  if [ ! -d "crates/tupa-cli" ]; then
    echo -e "${YELLOW}skip:${NC} golden check (tupa-cli removed — Rust-DSL migration)"
    echo
    return 0
  fi

  local tmp_expected
  tmp_expected="$(mktemp -d)"
  trap "rm -rf $tmp_expected" EXIT

  # Generate goldens into temp directory
  EXPECTED_DIR="$tmp_expected" bash scripts/update-goldens.sh >/dev/null 2>&1

  # Compare with committed expected directory
  if ! diff -r -q "examples/expected" "$tmp_expected" >/dev/null; then
    echo -e "${RED}Golden outputs differ from committed expectations${NC}"
    echo "Run: scripts/update-goldens.sh to regenerate"
    # Show a concise diff summary
    diff -r -u "examples/expected" "$tmp_expected" | head -80 || true
    return 1
  fi

  echo -e "${GREEN}ok${NC} Golden outputs match"
  echo
}

check_commit_convention() {
  # Only check if there are commits not yet pushed or last commit on current branch
  # Simple: check the last commit message
  local last_msg
  last_msg="$(git log -1 --pretty=%s)"
  local pattern='^(feat|fix|docs|refactor|test|ci|chore)(\([a-z0-9._/-]+\))?: .+'
  if echo "$last_msg" | grep -Eq "$pattern"; then
    echo -e "${GREEN}ok${NC} Last commit message follows Conventional Commits"
  else
    echo -e "${RED}Last commit message does not follow Conventional Commits:${NC}"
    echo "  Expected: <type>(<scope>): <summary>"
    echo "  Actual:   $last_msg"
    return 1
  fi
}

check_publish_dry_run() {
  echo -e "${YELLOW}==>${NC} Publish packaging check (workspace, dependency order)"
  # Crates publicáveis em ordem topológica (deps antes dos dependentes).
  # Mantém paridade com [workspace].members do Cargo.toml.
  local crates=(
    tupa-core-macros
    tupa-lints
    tupa-core
    tupa-engine
    tupa-plugin
    tupa-pyffi
    cargo-tupa
  )
  # Limitação do registry: `cargo publish --dry-run` resolve as deps internas
  # (tupa-*) contra crates.io. Ao preparar uma versão nova, essas versões ainda
  # não estão indexadas, então só os crates *folha* (sem deps internas por path)
  # passam por uma verificação completa de build. Para os dependentes, validamos
  # o empacotamento com `cargo package --list` (manifesto + arquivos incluídos),
  # que não toca o registry; o build completo é exercido na publicação real,
  # crate a crate, conforme cada dependência é indexada.
  for crate in "${crates[@]}"; do
    local manifest="crates/$crate/Cargo.toml"
    if [[ ! -f "$manifest" ]]; then
      echo -e "${RED}Failed:${NC} manifest not found for $crate ($manifest)"
      return 1
    fi
    if grep -Eq 'path *= *"\.\./' "$manifest"; then
      echo "Packaging $crate (package --list — deps internas ainda não publicadas)..."
      if ! cargo package -p "$crate" --allow-dirty --list >/dev/null 2>&1; then
        echo -e "${RED}Failed:${NC} cargo package --list for $crate"
        return 1
      fi
    else
      echo "Dry-run $crate (publish --dry-run — build completo)..."
      if ! cargo publish -p "$crate" --locked --allow-dirty --dry-run >/dev/null 2>&1; then
        echo -e "${RED}Failed:${NC} cargo publish dry-run for $crate"
        return 1
      fi
    fi
  done
  echo -e "${GREEN}ok${NC} All workspace crates pass packaging checks"
}

echo -e "${GREEN}Local CI check (CI + Docs Lint)${NC}"
echo

require_cmd cargo "install Rust/Cargo"

# Install optional tools (may need sudo)
install_markdownlint || exit 1
install_lychee || exit 1

require_cmd diff "coreutils (usually pre-installed)"
require_cmd git "install git"

run_step "cargo fmt --all --check" cargo fmt --all --check
run_step "cargo clippy --workspace --all-targets -- -D warnings" cargo clippy --workspace --all-targets -- -D warnings
run_step "TUPA_RUN_E2E=0 cargo test --workspace --locked" env TUPA_RUN_E2E=0 cargo test --workspace --locked
run_step "markdownlint **/*.md" markdownlint "**/*.md"
run_step "scripts/docs-parity-check.sh" ./scripts/docs-parity-check.sh
run_step "scripts/vipertrade-smoke.sh" ./scripts/vipertrade-smoke.sh

if run_lychee_docs; then
  echo -e "${GREEN}ok${NC} lychee docs links"
else
  if [[ "$STRICT_LINKS" == "1" ]]; then
    echo -e "${RED}fail:${NC} lychee docs links (strict mode)"
    exit 1
  fi
  echo -e "${YELLOW}warn:${NC} lychee docs links found issues (non-strict mode)."
  echo -e "      Use CI_LOCAL_STRICT_LINKS=1 to fail on link issues."
fi

echo

# Golden verification (mirrors examples-golden workflow)
if [[ -z "${CI_LOCAL_SKIP_GOLDEN:-}" ]] && [[ "$SKIP_GOLDEN" != "1" ]]; then
  if run_golden_check; then
    echo -e "${GREEN}Golden check passed.${NC}"
  else
    echo -e "${RED}Golden check failed.${NC}"
    echo "Hint: Run 'scripts/update-goldens.sh' to regenerate expectations, then review the changes."
    exit 1
  fi
else
  echo -e "${YELLOW}skip:${NC} golden check (CI_LOCAL_SKIP_GOLDEN=1)"
fi

echo

# Optional: commit message convention (for PR-like validation)
if [[ "${CHECK_COMMIT}" == "1" ]]; then
  if check_commit_convention; then
    echo -e "${GREEN}Commit convention ok.${NC}"
  else
    echo -e "${RED}Commit convention failed.${NC}"
    exit 1
  fi
else
  echo -e "${YELLOW}skip:${NC} commit message convention check (CI_LOCAL_CHECK_COMMIT=1 to enable)"
fi

echo

# Optional: publish dry-run validation (for release preparation)
if [[ "${CHECK_PUBLISH}" == "1" ]]; then
  if check_publish_dry_run; then
    echo -e "${GREEN}Publish dry-run ok.${NC}"
  else
    echo -e "${RED}Publish dry-run failed.${NC}"
    exit 1
  fi
else
  echo -e "${YELLOW}skip:${NC} publish dry-run (CI_LOCAL_CHECK_PUBLISH=1 to enable)"
fi

echo
echo -e "${GREEN}All local CI checks passed.${NC}"
