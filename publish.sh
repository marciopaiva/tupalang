#!/bin/bash
# Prepare and publish TupaLang v0.8.2 to crates.io
# Run from tupalang/ directory

set -e

echo "=== Preparing TupaLang v0.8.2 for crates.io ==="

# Backup original Cargo.toml files
for crate in crates/*/Cargo.toml; do
    cp "$crate" "$crate.bak"
done

# Remove path dependencies (they must use version on crates.io)
for crate in crates/*/Cargo.toml; do
    sed -i 's|path = "../[^"]*", ||g; s|path = "../[^"]*"||g' "$crate"
done

echo "Building to verify..."
cargo build --release

echo "Running tests..."
cargo test --workspace

echo "Ready to publish. Run: cargo publish -p tupa-lexer && cargo publish -p tupa-parser ..."

# Restore backups
for crate in crates/*/Cargo.toml; do
    mv "$crate.bak" "$crate"
done

echo "Restored path dependencies. Ready for local development."