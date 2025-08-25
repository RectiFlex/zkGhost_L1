# zkGhost — Substrate L1 scaffold (offline-friendly)

zkGhost is a Substrate-based L1 chain focused on verifying zero-knowledge proofs on-chain. This
repository is an offline-friendly scaffold: it contains a minimal workspace, a Rust-only node stub,
a placeholder runtime template, and a realistic FRAME pallet (pallet-zkghost) as an integration
target. No external crates are fetched here; integrate with Substrate externally.

## Repository layout

- Cargo.toml (workspace)
- node/ — pure Rust stub binary (no Substrate deps)
- runtime/ — placeholder crate; comments indicate how to wire a FRAME runtime externally
- pallets/zkghost/ — FRAME pallet (realistic code; will compile once Substrate deps are present)
- docs/ARCHITECTURE.md
- LICENSE (Apache-2.0)

## Quickstart (integration outside this offline environment)

1. Prepare a Substrate node baseline:
   - Option A: clone the official node-template and add this repo as a sibling.
   - Option B: use an existing node repo and add this as a workspace member.

2. Wire the pallet into your runtime:
   - In `runtime/Cargo.toml`, uncomment relevant FRAME dependencies and add `pallet-zkghost` as a
     dependency (path to `pallets/zkghost`).
   - In `runtime/src/lib.rs`, implement the commented template: define parameter_types! (max sizes,
     per-block cap), implement `pallet_zkghost::Config`, and add the pallet to `construct_runtime!`.

3. Enable Substrate dependencies in the pallet:
   - Open `pallets/zkghost/Cargo.toml`, read the NOTE, and uncomment the FRAME/SP deps.
   - Keep `default-features = false` and feature-gate `std` appropriately.

4. (Later) add ZK verifier via arkworks:
   - Integrate `ark-groth16`, `ark-bls12-381` or the pairing curve you need, and replace the
     placeholder verifier in `pallets/zkghost/src/lib.rs::verify_proof` with actual verification.
   - Consider moving verification-heavy logic to a custom host function or off-chain worker if
     weights are large.

5. Build commands (run in your integrated Substrate repo):
   - `cargo build -p pallet-zkghost`
   - `cargo build -p <your-runtime-crate>`
   - `cargo build -p <your-node-binary>`

6. Runtime plumbing checklist:
   - Add `pallet_zkghost` to `construct_runtime!`.
   - Define `MaxVkLength`, `MaxProofLength`, `MaxPublicInputsLength`, `MaxProofsPerBlock`.
   - Ensure BaseCallFilter allows the pallet or include custom filter.

7. Storage and calls overview:
   - `VerifyingKeys`: map `CircuitId -> BoundedVec<u8>` for serialized VKs.
   - `CircuitMeta`: map `CircuitId -> (vk_hash, last_updated_block)`.
   - `ProofsThisBlock`: reset each block; enforces a per-block submission cap.
   - Calls: `set_vk(Root)`; `submit_proof(Signed)` with bounded sizes and simple input checks.

## Roadmap (high-level)

- Phase 0 (MVP): store VKs; accept proof submissions; placeholder verifier returning false.
- Phase 1: VK governance and upgrade flows; sudo/collective or democracy governance.
- Phase 2: ZK rollup-style validity gating selected pallets (transactions gated by proofs).
- Phase 3: succinct light client proofs; minimal data availability; improved network economics.

## Notes

- Consensus target: BABE/GRANDPA; token: GHOST; networking via libp2p (through Substrate).
- This repo avoids Substrate crates to remain offline-friendly; uncomment deps when integrating.
