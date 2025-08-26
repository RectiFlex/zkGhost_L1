#!/usr/bin/env bash
set -euo pipefail
# Run pallet benchmarking (requires runtime-benchmarks feature and proper wiring)
cargo run -p zkghost-node --release --features runtime-benchmarks -- \
  benchmark pallet \
  --pallet pallet-zkghost \
  --execution wasm \
  --wasm-execution compiled \
  --extrinsic "*" \
  --steps 50 \
  --repeat 20
