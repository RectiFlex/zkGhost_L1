# zkGhost E2E Proof Flow (Dev Guide)

This guide shows how to:
1) Launch a local dev node
2) Set a verifying key (VK) for a circuit
3) Submit a proof using the BN254 Groth16 verifier (feature-gated)

Prereqs:
- Build runtime WASM and node
- For on-chain verification enable `--features zk-verify` in `pallet-zkghost` and recompile

Steps:
1. Run node in dev mode
   cargo run -p zkghost-node -- --dev --enable-offchain-indexing 1

2. Set VK (sudo)
   Use Polkadot-JS Apps or subxt; call `zkghost.set_vk(circuit_id, metadata, vk_bytes)` as Root (Alice in dev)

3. Submit proof (signed)
   Call `zkghost.submit_proof(circuit_id, public_inputs, proof_bytes)` from a funded account (e.g., Alice)

Data formats:
- vk_bytes, proof_bytes: ark-serialize (BN254 Groth16)
- public_inputs: concatenation of 32-byte little-endian BN254 field elements

For a reference off-chain prover, see tools/prover-cli (enable arkworks and compile with `--features prover`).
