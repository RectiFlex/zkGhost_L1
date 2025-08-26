#!/usr/bin/env bash
set -euo pipefail
# Dev-only demo sequence (requires node running separately)
# 1) Build (host + wasm)
cargo build --workspace --locked
cargo build -p zkghost-runtime --release --target wasm32-unknown-unknown
# 2) Launch node in another terminal:
#    cargo run -p zkghost-node -- --dev
# 3) Use Polkadot-JS Apps to sudo call set_vk, then submit_proof from Alice.
