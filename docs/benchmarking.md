# Benchmarking scaffold

Two paths are provided:
- Pallet micro-bench tests: `cargo test -p pallet-zkghost --features runtime-benchmarks -- --nocapture`
- Node-wide benchmarking (to be wired next): `cargo run -p zkghost-node --release --features runtime-benchmarks -- benchmark pallet --pallet pallet-zkghost --execution wasm --wasm-execution compiled --extrinsic="*" --steps 50 --repeat 20`

Weights live at `pallets/zkghost/src/weights.rs` and should be replaced after running benchmarks.
