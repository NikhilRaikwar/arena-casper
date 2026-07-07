#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONTRACT_DIR="$ROOT_DIR/contracts/arena"
TOOLS_DIR="$ROOT_DIR/.tools/wasm"
KEYS_DIR="$ROOT_DIR/keys"
ENV_FILE="$ROOT_DIR/.env"

export PATH="$CONTRACT_DIR:$TOOLS_DIR/bin:$PATH"

log() {
  printf '\n[arena] %s\n' "$1"
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1
}

ensure_wasm_tools() {
  if need_cmd wasm-opt && need_cmd wasm-strip; then
    log "wasm-opt and wasm-strip found"
    return
  fi

  if [ -f "$CONTRACT_DIR/wasm-opt" ] && [ -f "$CONTRACT_DIR/wasm-strip" ]; then
    chmod +x "$CONTRACT_DIR/wasm-opt" "$CONTRACT_DIR/wasm-strip" 2>/dev/null || true
    log "Using local unoptimized wasm fallback wrappers"
    return
  fi

  mkdir -p "$TOOLS_DIR/bin"

  if need_cmd apt-get; then
    log "Installing binaryen and wabt with apt-get"
    sudo apt-get update
    sudo apt-get install -y binaryen wabt
  elif need_cmd brew; then
    log "Installing binaryen and wabt with brew"
    brew install binaryen wabt
  else
    log "No apt-get or brew found. Install binaryen and wabt, or add wasm-opt and wasm-strip to PATH."
    log "Windows users can run this script from WSL: wsl -d Ubuntu bash scripts/setup-testnet.sh"
  fi

  if ! need_cmd wasm-opt || ! need_cmd wasm-strip; then
    printf '[arena] Missing wasm tools after install attempt.\n' >&2
    printf '[arena] Required: wasm-opt from binaryen and wasm-strip from wabt.\n' >&2
    exit 1
  fi
}

check_keys() {
  local missing=0
  for key in agent-alpha.pem agent-beta.pem verifier.pem; do
    if [ ! -f "$KEYS_DIR/$key" ]; then
      missing=1
      printf '[arena] Missing %s\n' "$KEYS_DIR/$key" >&2
    fi
  done

  if [ "$missing" -ne 0 ]; then
    cat >&2 <<'MSG'
[arena] Create three funded Casper Testnet Ed25519 accounts:
  1. Visit https://testnet.cspr.live and create/download PEM keys.
  2. Put the secret keys here:
       keys/agent-alpha.pem
       keys/agent-beta.pem
       keys/verifier.pem
  3. Fund each account from https://faucet.testnet.casperlabs.io
MSG
    exit 1
  fi
}

update_env_contract_hash() {
  local hash="$1"
  touch "$ENV_FILE"
  if grep -q '^ARENA_CONTRACT_HASH=' "$ENV_FILE"; then
    sed -i.bak "s|^ARENA_CONTRACT_HASH=.*|ARENA_CONTRACT_HASH=$hash|" "$ENV_FILE"
  else
    printf '\nARENA_CONTRACT_HASH=%s\n' "$hash" >> "$ENV_FILE"
  fi
}

parse_contract_hash() {
  sed -nE 's/.*(hash-[0-9a-fA-F]{64}|contract-[0-9a-fA-F]{64}).*/\1/p' | tail -n 1
}

parse_deploy_hash() {
  sed -nE 's/.*([0-9a-fA-F]{64}).*/\1/p' | tail -n 1
}

ensure_wasm_tools
chmod +x "$CONTRACT_DIR/wasm-opt" "$CONTRACT_DIR/wasm-strip" 2>/dev/null || true
check_keys

log "Building Odra contract"
(cd "$CONTRACT_DIR" && cargo odra build)

if ! cargo odra deploy --help >/dev/null 2>&1; then
  cat >&2 <<'MSG'
[arena] This cargo-odra version does not expose `cargo odra deploy`.
[arena] The WASM build is ready. Deploy manually with the Casper/Odra deploy tool available in your environment,
[arena] then paste the resulting hash into ARENA_CONTRACT_HASH in .env.
MSG
  exit 1
fi

log "Deploying Arena contract to Casper Testnet"
DEPLOY_OUTPUT="$(cd "$CONTRACT_DIR" && cargo odra deploy --network testnet --secret-key "$KEYS_DIR/verifier.pem" 2>&1)"
printf '%s\n' "$DEPLOY_OUTPUT"

CONTRACT_HASH="$(printf '%s\n' "$DEPLOY_OUTPUT" | parse_contract_hash)"
DEPLOY_HASH="$(printf '%s\n' "$DEPLOY_OUTPUT" | parse_deploy_hash)"

if [ -z "$CONTRACT_HASH" ]; then
  printf '[arena] Could not parse contract hash from deploy output.\n' >&2
  exit 1
fi

update_env_contract_hash "$CONTRACT_HASH"

log "ARENA_CONTRACT_HASH written to .env: $CONTRACT_HASH"
if [ -n "$DEPLOY_HASH" ]; then
  log "CSPR.live deploy: https://testnet.cspr.live/deploy/$DEPLOY_HASH"
fi
