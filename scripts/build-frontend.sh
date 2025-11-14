#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/app"

if ! cargo leptos --version >/dev/null 2>&1; then
  cargo install cargo-leptos --locked
fi

# Ensure wasm32-unknown-unknown target is installed for WebAssembly compilation
rustup target add wasm32-unknown-unknown

cargo leptos build --features csr

if ! command -v wasm-bindgen &>/dev/null; then
  cargo install wasm-bindgen-cli --version 0.2.105 --force
fi

TARGET_WASM="$ROOT/app/target/front/wasm32-unknown-unknown/debug/smartdoc_app_frontend.wasm"
if [ ! -f "$TARGET_WASM" ]; then
  echo "Wasm artifact not found: $TARGET_WASM" >&2
  exit 1
fi

DIST_DIR="$ROOT/app/dist"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR/pkg"

wasm-bindgen "$TARGET_WASM" --out-dir "$DIST_DIR/pkg" --target web --no-typescript
cp "$ROOT/app/static/index.html" "$DIST_DIR/index.html"
