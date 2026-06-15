#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
CONTRACT_DIR="$ROOT/contracts/ajo"
WASM_PATH="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/ajo_contract.wasm"

echo "=== Ajo Contract Testnet Deploy ==="

if ! command -v soroban &>/dev/null; then
  echo "Error: soroban CLI required. Install: cargo install --locked soroban-cli@20.0.0" >&2
  exit 1
fi

echo "Building contract WASM..."
cd "$CONTRACT_DIR"
cargo build --target wasm32-unknown-unknown --release 2>&1

if [[ ! -f "$WASM_PATH" ]]; then
  echo "Error: WASM not found at $WASM_PATH" >&2
  exit 1
fi

echo "WASM built: $WASM_PATH"
echo ""

echo "Uploading WASM to testnet..."
WASM_HASH=$(soroban contract upload \
  --wasm "$WASM_PATH" \
  --source deployer \
  --network testnet)

echo "WASM Hash: $WASM_HASH"
echo ""

echo "Deploying contract..."
CONTRACT_ID=$(soroban contract deploy \
  --wasm-hash "$WASM_HASH" \
  --source deployer \
  --network testnet)

echo ""
echo "=========================================="
echo "  Contract deployed successfully!"
echo "  CONTRACT_ID: $CONTRACT_ID"
echo "=========================================="
echo ""
echo "Initializing contract with deployer as admin..."

DEPLOYER_ADDRESS=$(soroban keys address deployer)

soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$DEPLOYER_ADDRESS"

echo "Contract initialized."
echo ""
echo "Add to your .env files:"
echo "  CONTRACT_ID=$CONTRACT_ID"
echo "  NEXT_PUBLIC_CONTRACT_ID=$CONTRACT_ID"
