#!/usr/bin/env bash
set -euo pipefail
# Placeholder guidance for generating weights once benchmarking is fully wired
# Build runtime with benchmarking features (may require nightly and proper CLI wiring):
# RUSTFLAGS="-C target-cpu=native" \
# cargo build -p zkghost-runtime --features runtime-benchmarks --release
# Then run node benchmarking subcommand (once implemented) to produce weights.rs.
# For now, update pallets/zkghost/src/weights.rs manually or via CI job once enabled.
echo "Benchmark scaffolding in place. Final wiring to be completed post-CI stabilization."
