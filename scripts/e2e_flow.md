# zkGhost E2E Proof Flow (Guide)

1) Launch dev chain:
   ./scripts/dev_chain.sh

2) Set verifying key (as sudo/root):
   - Use Polkadot JS Apps or a simple RPC call to `pallet-zkghost::set_vk` with a BN254 Groth16 VK blob (ark-serialize bytes).

3) Generate proof off-chain (BN254 Groth16) and public inputs, then submit:
   - Build pallet with feature `zk-verify` enabled to verify on-chain, or keep stub for offline.
   - Submit via `pallet-zkghost::submit_proof(circuit_id, proof, public_inputs)` as a signed extrinsic.

Notes:
- `public_inputs` should be concatenated 32-byte little-endian field elements.
- For CI, verification backend is gated by `zk-verify`.
