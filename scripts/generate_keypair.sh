#!/usr/bin/env bash
set -euo pipefail

echo "Generating Stellar keypair for testnet deployment..."

if ! command -v soroban &>/dev/null; then
  echo "Error: soroban CLI not found. Install via: cargo install --locked soroban-cli@20.0.0" >&2
  exit 1
fi

KEYPAIR=$(soroban keys generate deployer --network testnet 2>&1 || true)

PUBLIC_KEY=$(soroban keys address deployer 2>/dev/null)

echo ""
echo "============================================"
echo "  Deployer Public Key:  $PUBLIC_KEY"
echo "============================================"
echo ""
echo "Fund this account on testnet:"
echo "  https://friendbot.stellar.org?addr=$PUBLIC_KEY"
echo ""
echo "Or via curl:"
echo "  curl -s \"https://friendbot.stellar.org?addr=$PUBLIC_KEY\" | jq .id"
echo ""
echo "The key is saved under the soroban CLI identity 'deployer'."
