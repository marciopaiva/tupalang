#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PIPELINE_SRC="examples/pipeline/vipertrade_smoke.tp"
PLAN_FILE="vipertrade_smoke.plan.json"

cargo run -q -p tupa-cli -- check "$PIPELINE_SRC" >/dev/null
cargo run -q -p tupa-cli -- codegen --plan-only "$PIPELINE_SRC" >/dev/null

if [[ ! -f "$PLAN_FILE" ]]; then
  echo "ViperTrade smoke failed: plan file not generated"
  exit 1
fi

if ! grep -q '"name"[[:space:]]*:[[:space:]]*"ViperTradeValidation"' "$PLAN_FILE"; then
  echo "ViperTrade smoke failed: expected name ViperTradeValidation in plan"
  cat "$PLAN_FILE"
  exit 1
fi

echo "vipertrade smoke: ok"
# Smoke gate intentionally validates check+plan generation only.
