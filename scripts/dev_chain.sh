#!/usr/bin/env bash
set -euo pipefail
# Run a single-node dev chain
cargo run -p zkghost-node -- --dev --ws-external --rpc-methods=Unsafe --rpc-cors=all
