# zkGhost Architecture (High-level)

zkGhost is a Substrate-based L1 that verifies zero-knowledge proofs on-chain to unlock privacy,
compression, and validity use-cases. This document outlines the intended architecture once the
scaffold is integrated with Substrate in an external environment.

## System Overview

- Consensus: BABE (block production) + GRANDPA (finality).
- Networking: libp2p via Substrate's networking stack (gossip, peer discovery, bootnodes).
- Fees & Staking: native GHOST token (Balances + Staking pallets in a full runtime).
- Execution: FRAME-based runtime with pallets; `pallet-zkghost` is the ZK verification surface.

## ZK Flow (MVP)

1. Governance or Root sets verifying keys (VK) for circuits via `set_vk(circuit_id, vk_bytes)`.
2. Users call `submit_proof(circuit_id, proof_bytes, public_inputs_bytes)`.
3. Runtime checks lengths, ensures circuit exists, applies per-block proof cap, and verifies.
4. On success, runtime emits `ProofVerified` and downstream pallets may trust the result.

Notes:
- The scaffold's verifier is a placeholder that returns `false`. Replace with arkworks
  (`ark-groth16`, appropriate curve) in integration.
- For heavy verification costs, consider precompiles, host functions, or off-chain workers.

## Runtime Composition (target)

- Core: System, Timestamp, Balances, Transaction Payment.
- Consensus support: BABE, GRANDPA, Authority Discovery (as needed).
- Governance (Phase 1): Sudo/Collective/Democracy for VK management and upgrades.
- zkGhost: the verification pallet (`pallet-zkghost`).

## Networking

- libp2p under Substrate: secure multiplexed streams, gossip message propagation.
- Bootnodes configured per network (dev/testnet/mainnet).
- Optional telemetry for network health and observability.

## Roadmap

- Phase 0 (MVP):
  - On-chain VK storage and proof submission API.
  - Placeholder verifier; correctness tested with mocked results.

- Phase 1 (VK Governance):
  - Governance-managed VK lifecycle (add/upgrade/deprecate).
  - On-chain metadata (hashes, circuit params, auditing).

- Phase 2 (Validity-Gated Execution):
  - Rollup-style gating for selected pallets/transactions by proof validity.
  - Economic incentives using GHOST; metering based on proof complexity.

- Phase 3 (Succinct Light Client):
  - Succinct proofs of chain state for light clients.
  - Data availability improvements and minimized on-chain footprint.

## Security Considerations

- VK provenance and governance are critical; restrict `set_vk` initially to Root/sudo.
- Input size bounds and per-block caps mitigate DoS vectors.
- When integrating arkworks, audit circuits and pairing libraries; pin versions and run fuzzing.
