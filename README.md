# zkGhost L1 (scaffold)

This repository contains an offline-friendly scaffold for an L1 blockchain using Substrate with a zk pallet.

Components
- pallets/zkghost: FRAME pallet to store verifying keys and verify Groth16 proofs.
- runtime: Substrate runtime scaffold with commented FRAME wiring.
- node: Node stub. Replace with real sc-service integration.

ZK verification
- The pallet includes a feature-gated arkworks Groth16 BN254 verifier (`zk-verify`).
- Arkworks dependencies are commented to keep offline build safe. Enable and pin when integrating.

Integration steps (polkadot-sdk v1.11)
1) Pin polkadot-sdk crates in runtime and node manifests; uncomment FRAME/wiring.
2) In pallets/zkghost/Cargo.toml, uncomment arkworks deps; build with `--features zk-verify`.
3) Implement chain-spec (dev/test), balances, sudo, tx payment, and launch.
