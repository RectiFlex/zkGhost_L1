#!/usr/bin/env bash
set -euo pipefail
# E2E demo scaffold: produce dummy VK/proof/inputs using prover-cli, then print substrate-js snippet to submit.
BIN=${BIN:-target/release/zkghost-node}
PROFILE=${PROFILE:-release}
if [ ! -x "$BIN" ]; then
  echo "Building node..."; cargo build -p zkghost-node --$PROFILE
  BIN=target/$PROFILE/zkghost-node
fi
mkdir -p artifacts
cargo run -p prover-cli --quiet -- \
  --vk-json artifacts/vk.json \
  --proof-json artifacts/proof.json \
  --inputs-json artifacts/inputs.json || true

cat <<'EOF'
To submit on a running dev node via Polkadot.js:
// In browser console with Polkadot.js extension connected to ws://127.0.0.1:9944
const vkId = 1;
const vkMeta = { /* read artifacts/vk.json or compressed bytes */ };
await api.tx.zkghost.setVk(vkId, vkMeta).signAndSend(alice);
const proof = /* from artifacts/proof.json bytes */;
const inputs = /* from artifacts/inputs.json bytes */;
await api.tx.zkghost.submitProof(vkId, proof, inputs).signAndSend(alice);
EOF
