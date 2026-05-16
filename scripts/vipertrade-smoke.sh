#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# ViperTrade smoke: builds and runs the ViperTradeValidation example.
# This replaces the legacy .tp pipeline check with a Rust-DSL equivalent.

echo "==> Building ViperTrade smoke example (tupa-engine::vipertrade_smoke)"
cargo check -p tupa-engine --example vipertrade_smoke

echo "==> Running ViperTrade smoke example"
cargo run -q -p tupa-engine --example vipertrade_smoke

echo "vipertrade smoke: ok"
