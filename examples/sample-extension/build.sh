#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "==> Building WASM with wasm-pack..."
wasm-pack build --target web --out-dir web/pkg --no-typescript

echo ""
echo "==> Build complete."
echo "    Serve the extension:  cd web && python3 -m http.server 8080"
echo "    Then configure your Forma extension URL to http://localhost:8080"
