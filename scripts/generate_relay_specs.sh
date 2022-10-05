#!/usr/bin/env bash

set -e

docker run parity/polkadot:v0.9.29 build-spec --disable-default-bootnode --chain rococo-local --raw > ./resources/rococo-local.json

