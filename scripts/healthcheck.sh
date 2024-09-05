#!/usr/bin/env bash

set -e

recurrency_rpc_port="${Recurrency_RPC_PORT:-11936}"

node="127.0.0.1"
port="$recurrency_rpc_port"
curl -sS \
    -H 'Content-Type: application/json' \
    --data '{"id":1,"jsonrpc":"2.0","method":"system_health"}' \
    "$node:$port" |\
jq -r '.result'
