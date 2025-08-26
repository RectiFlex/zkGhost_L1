#!/usr/bin/env bash
set -euo pipefail
# Generate raw dev spec
cargo run -p zkghost-node -- build-spec --dev > dev.json
cargo run -p zkghost-node -- build-spec --dev --raw > dev.raw.json
echo "Generated: dev.json, dev.raw.json"
