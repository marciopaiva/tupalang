#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_REPO="https://github.com/marciopaiva/tupalang.wiki.git"
WORKDIR="${REPO_ROOT}/.wiki-tmp"

rm -rf "${WORKDIR}"

git clone "${WIKI_REPO}" "${WORKDIR}"

# Clean existing content (except .git)
find "${WORKDIR}" -maxdepth 1 ! -name ".git" ! -name "." -exec rm -rf {} +

# Copy documentation folders
cp -r "${REPO_ROOT}/docs/en" "${WORKDIR}/en"
cp -r "${REPO_ROOT}/docs/pt-br" "${WORKDIR}/pt-br"
cp -r "${REPO_ROOT}/docs/es" "${WORKDIR}/es"
cp -r "${REPO_ROOT}/docs/shared" "${WORKDIR}/shared"

# Copy project root files
cp "${REPO_ROOT}/CONTRIBUTING.md" "${WORKDIR}/CONTRIBUTING.md"
cp "${REPO_ROOT}/CODE_OF_CONDUCT.md" "${WORKDIR}/CODE_OF_CONDUCT.md"
cp -r "${REPO_ROOT}/examples" "${WORKDIR}/examples"

# Create Home.md
cat <<'EOF' > "${WORKDIR}/Home.md"
# Welcome to Tupã Wiki

Select your language:

- [English Documentation](en/index.md)
- [Documentação em Português](pt-br/index.md)
- [Documentación en Español](es/index.md)

## Useful Links
- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Examples](examples/README.md)
EOF

# Create _Sidebar.md
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
