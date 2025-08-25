#!/usr/bin/env bash
set -euo pipefail
BIN="target/release/zkghost-node"
if [ ! -x "$BIN" ]; then echo "Build the node first: cargo build --release"; exit 1; fi
exec "$BIN" --dev --tmp
