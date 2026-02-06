#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_REPO="https://github.com/marciopaiva/tupalang.wiki.git"
WORKDIR="${REPO_ROOT}/.wiki-tmp"

rm -rf "${WORKDIR}"

git clone "${WIKI_REPO}" "${WORKDIR}"

find "${WORKDIR}" -maxdepth 1 -type f -name "*.md" -delete

cp "${REPO_ROOT}/README.md" "${WORKDIR}/Home.md"
cp "${REPO_ROOT}/docs"/*.md "${WORKDIR}/"
cp "${REPO_ROOT}/CONTRIBUTING.md" "${WORKDIR}/Contributing.md"
cp "${REPO_ROOT}/CODE_OF_CONDUCT.md" "${WORKDIR}/Code-of-Conduct.md"
cp "${REPO_ROOT}/examples/README.md" "${WORKDIR}/Examples.md"

cat <<'EOF' > "${WORKDIR}/_Sidebar.md"
## Português
- [[Home]]
- [[GETTING_STARTED]]
- [[Examples]]
- [[DEV_ENV]]
- [[GLOSSARY]]
- [[ARCHITECTURE]]
- [[STYLE_GUIDE]]
- [[FAQ]]
- [[COMMON_ERRORS]]
- [[ROADMAP]]
- [[EXAMPLES_GUIDE]]
- [[SYNTAX_GLOSSARY]]
- [[TYPES_GLOSSARY]]
- [[ENV_SETUP]]
- [[COMPATIBILITY]]
- [[PROJECT_OVERVIEW]]

## English
- [[Home.en|Home (EN)]]
- [[GETTING_STARTED.en|Getting Started]]
- [[Examples|Examples]]
- [[DEV_ENV.en|Dev Env]]
- [[GLOSSARY.en|Glossary]]
- [[ARCHITECTURE|Architecture]]
- [[STYLE_GUIDE|Style Guide]]
- [[FAQ.en|FAQ]]
- [[COMMON_ERRORS|Common Errors]]
- [[ROADMAP|Roadmap]]
- [[EXAMPLES_GUIDE|Examples Guide]]
- [[SYNTAX_GLOSSARY|Syntax Glossary]]
- [[TYPES_GLOSSARY|Types Glossary]]
- [[ENV_SETUP|Environment Setup]]
- [[COMPATIBILITY|Compatibility]]
- [[PROJECT_OVERVIEW|Project Overview]]

## Reference
- [[SPEC]]
- [[DIAGNOSTICS_CHECKLIST]]
- [[DIAGNOSTICS_GLOSSARY]]
- [[CODEGEN]]

## Plans
- [[MVP_PLAN]]
- [[ADOPTION_PLAN]]
- [[CHANGELOG]]
- [[RELEASE_CHECKLIST]]
- [[RELEASE_GUIDE]]
- [[VERSIONING]]

## Design Notes
- [[DESIGN_NOTES]]

## Contribute
- [[Contributing]]
- [[Code-of-Conduct]]
- [[DOCS_CONTRIBUTING]]
- [[TESTING]]
- [[CI_GUIDE]]
- [[ERROR_MESSAGES]]
- [[GOVERNANCE]]
- [[CONTRIBUTING_FAQ]]
- [[ISSUES_GUIDE]]
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
