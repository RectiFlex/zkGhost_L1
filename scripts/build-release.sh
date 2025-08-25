#!/usr/bin/env bash
set -euo pipefail
export RUSTFLAGS="-C codegen-units=1 -C opt-level=1"
export CARGO_BUILD_JOBS=1
cargo build --release -p zkghost-prover-cli
cargo build --release -p pallet-zkghost --features "std zk-verify"
cargo build --release -p zkghost-runtime --features std
cargo build --release -p zkghost-node
