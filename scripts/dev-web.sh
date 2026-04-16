#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

export PATH="$HOME/.cargo/bin:$PATH"

echo "==> Building Wasm..."
cd "$REPO_ROOT/crates/plinth-core"
wasm-pack build --target web --out-dir ../../packages/web/plinth-js/wasm

echo "==> Installing dependencies..."
cd "$REPO_ROOT"
pnpm install

echo "==> Building packages..."
pnpm -r build

echo "==> Starting web demo..."
pnpm --filter @wirevice/plinth-dev start
