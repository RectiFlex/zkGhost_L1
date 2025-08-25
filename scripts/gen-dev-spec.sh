#!/usr/bin/env bash
set -euo pipefail
# Placeholder: when node service is wired, use `zkghost-node build-spec` pattern
echo '{"name":"zkGhost Dev","id":"zkghost-dev","chainType":"Development","properties":{"tokenSymbol":"GHOST","tokenDecimals":12}}' > chainspecs/zkghost-dev.json
