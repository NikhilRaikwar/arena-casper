#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONTRACT_DIR="$ROOT_DIR/contracts/arena"
TOOLS_DIR="$ROOT_DIR/.tools/wasm"

export PATH="$CONTRACT_DIR:$TOOLS_DIR/bin:$PATH"

need_cmd() {
  command -v "$1" >/dev/null 2>&1
}

ensure_wasm_tools() {
  if need_cmd wasm-opt && need_cmd wasm-strip; then
    return
  fi

  if [ -f "$CONTRACT_DIR/wasm-opt" ] && [ -f "$CONTRACT_DIR/wasm-strip" ]; then
    chmod +x "$CONTRACT_DIR/wasm-opt" "$CONTRACT_DIR/wasm-strip" 2>/dev/null || true
    return
  fi

  mkdir -p "$TOOLS_DIR/bin"

  if need_cmd apt-get; then
    sudo apt-get update
    sudo apt-get install -y binaryen wabt
  elif need_cmd brew; then
    brew install binaryen wabt
  else
    printf '[arena] Missing wasm-opt and/or wasm-strip.\n' >&2
    printf '[arena] Install binaryen and wabt, or run from WSL with apt-get available.\n' >&2
    exit 1
  fi

  if ! need_cmd wasm-opt || ! need_cmd wasm-strip; then
    printf '[arena] Missing wasm tools after install attempt.\n' >&2
    exit 1
  fi
}

ensure_wasm_tools
chmod +x "$CONTRACT_DIR/wasm-opt" "$CONTRACT_DIR/wasm-strip" 2>/dev/null || true
(cd "$CONTRACT_DIR" && cargo odra build)
