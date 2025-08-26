#!/usr/bin/env bash
set -euo pipefail
PROFILE=${PROFILE:-release}
BIN=target/$PROFILE/zkghost-node
if [ ! -x "$BIN" ]; then
  echo "Building node..."; cargo build -p zkghost-node --$PROFILE
fi
exec "$BIN" --dev --unsafe-ws-external --rpc-cors all --rpc-methods safe -linfo
