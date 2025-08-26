#!/usr/bin/env bash
set -euo pipefail
BIN=${BIN:-target/release/zkghost-node}
PROFILE=${PROFILE:-release}
if [ ! -x "$BIN" ]; then
  echo "Building node..."; cargo build -p zkghost-node --$PROFILE
  BIN=target/$PROFILE/zkghost-node
fi
exec "$BIN" --dev --unsafe-ws-external --rpc-cors all --rpc-methods safe -linfo
