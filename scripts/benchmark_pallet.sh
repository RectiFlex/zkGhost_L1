#!/usr/bin/env bash
set -euo pipefail
# Requires nightly toolchain and runtime-benchmarks feature
RUSTFLAGS=${RUSTFLAGS:-""} cargo run --release \
  -p zkghost-node \
  --features runtime-benchmarks \
  -- benchmark pallet \
  --chain dev \
  --execution=wasm \
  --wasm-execution=compiled \
  --log=warn \
  --pallet pallet_zkghost \
  --extrinsic "*" \
  --steps 50 \
  --repeat 20 \
  --output ./pallets/zkghost/src/weights.rs \
  --template ./.maintain/frame-weight-template.hbs || true
