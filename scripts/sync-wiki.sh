#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_REPO="https://github.com/marciopaiva/tupalang.wiki.git"
WORKDIR="${REPO_ROOT}/.wiki-tmp"

rm -rf "${WORKDIR}"

git clone "${WIKI_REPO}" "${WORKDIR}"

# Clean existing content (except .git)
find "${WORKDIR}" -mindepth 1 -maxdepth 1 ! -name ".git" -exec rm -rf {} +

# Copy documentation folders
if [ -d "${REPO_ROOT}/docs/en" ]; then
  cp -r "${REPO_ROOT}/docs/en" "${WORKDIR}/en"
fi
if [ -d "${REPO_ROOT}/docs/pt-br" ]; then
  cp -r "${REPO_ROOT}/docs/pt-br" "${WORKDIR}/pt-br"
fi
if [ -d "${REPO_ROOT}/docs/es" ]; then
  cp -r "${REPO_ROOT}/docs/es" "${WORKDIR}/es"
fi
if [ -d "${REPO_ROOT}/docs/shared" ]; then
  cp -r "${REPO_ROOT}/docs/shared" "${WORKDIR}/shared"
fi

# Copy project root files
cp "${REPO_ROOT}/CONTRIBUTING.md" "${WORKDIR}/CONTRIBUTING.md"
cp "${REPO_ROOT}/CODE_OF_CONDUCT.md" "${WORKDIR}/CODE_OF_CONDUCT.md"
cp -r "${REPO_ROOT}/examples" "${WORKDIR}/examples"

# Sanitize for Wiki:
# 1. Remove BOM from all .md files
# 2. Remove .md extension from internal links (excluding http/https) to prevent raw rendering
find "${WORKDIR}" -name "*.md" -type f -exec sed -i -e '1s/^\xEF\xBB\xBF//' -e 's/(\([^:)]*\)\.md)/(\1)/g' {} +

# Create Home.md (links without .md)
cat <<'EOF' > "${WORKDIR}/Home.md"
# Welcome to Tupã Wiki

Select your language:

- [English Documentation](en/index)
- [Documentação em Português](pt-br/index)
- [Documentación en Español](es/index)

## Useful Links
- [Contributing](CONTRIBUTING)
- [Code of Conduct](CODE_OF_CONDUCT)
- [Examples](examples/README)
EOF

# Create _Sidebar.md (links without .md)
cat <<'EOF' > "${WORKDIR}/_Sidebar.md"
## Documentation

- [English](en/index)
- [Português](pt-br/index)
- [Español](es/index)

## Project

- [Contributing](CONTRIBUTING)
- [Code of Conduct](CODE_OF_CONDUCT)
- [Examples](examples/README)
EOF

cd "${WORKDIR}"

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

git add -A
if git diff --cached --quiet; then
  echo "No wiki changes to publish."
  exit 0
fi

git commit -m "Sync wiki from docs (new structure)"
git push origin HEAD
