#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_REPO="https://github.com/marciopaiva/tupalang.wiki.git"
WORKDIR="${REPO_ROOT}/.wiki-tmp"

rm -rf "${WORKDIR}"

git clone "${WIKI_REPO}" "${WORKDIR}"

cp "${REPO_ROOT}/README.md" "${WORKDIR}/Home.md"
cp "${REPO_ROOT}/docs"/*.md "${WORKDIR}/"
cp "${REPO_ROOT}/CONTRIBUTING.md" "${WORKDIR}/Contributing.md"
cp "${REPO_ROOT}/CODE_OF_CONDUCT.md" "${WORKDIR}/Code-of-Conduct.md"
cp "${REPO_ROOT}/examples/README.md" "${WORKDIR}/Examples.md"

cat <<'EOF' > "${WORKDIR}/_Sidebar.md"
## Start Here
- [[Home]]
- [[GETTING_STARTED]]
- [[Examples]]
- [[DEV_ENV]]

## Reference
- [[SPEC]]
- [[DIAGNOSTICS_CHECKLIST]]
- [[CODEGEN]]

## Plans
- [[MVP_PLAN]]
- [[ADOPTION_PLAN]]

## Design Notes
- [[DESIGN_NOTES]]

## Contribute
- [[Contributing]]
- [[Code-of-Conduct]]
EOF

cd "${WORKDIR}"

git config user.name "github-actions[bot]"
git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

git add -A
if git diff --cached --quiet; then
  echo "No wiki changes to publish."
  exit 0
fi

git commit -m "Sync wiki from docs"
git push
