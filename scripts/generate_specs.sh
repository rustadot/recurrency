#!/usr/bin/env bash

set -e

parachain_id=$1
build_step=$2
profile=$3
if [[ $parachain_id == "" ]]; then
  echo "Chain Name or Parachain ID argument not provided"
  exit 1
fi

BUILT_TARGET=./target/$profile/recurrency
if [ ! -x "$BUILT_TARGET" ]; then
    echo "FATAL: $BUILT_TARGET does not exist, or not executable, rebuild binary to continue"
    exit 1
fi
case $build_step in
  paseo-2000)
    mkdir -p ./res/genesis/local
    echo "Building Spec for recurrency testnet paseo localnet paraid=2000"
    $PWD/target/$profile/recurrency build-spec --disable-default-bootnode --chain=recurrency-paseo-local > ./res/genesis/local/recurrency-spec-paseo.json
    sed -i.bu "s/\"parachainId\": 2000/\"parachainId\": $parachain_id/g" ./res/genesis/local/recurrency-spec-paseo.json
    sed -i.bu "s/\"para_id\": 2000/\"para_id\": $parachain_id/g" ./res/genesis/local/recurrency-spec-paseo.json
    $PWD/target/$profile/recurrency build-spec --raw --disable-default-bootnode --chain ./res/genesis/local/recurrency-spec-paseo.json > ./res/genesis/local/paseo-local-recurrency-2000-raw.json
    rm ./res/genesis/local/recurrency-spec-paseo.json.bu

    echo "Exporting state and wasm for recurrency testnet paseo localnet paraid=2000"
    $PWD/target/$profile/recurrency export-genesis-state --chain ./res/genesis/local/paseo-local-recurrency-2000-raw.json > ./res/genesis/local/recurrency-paseo-genesis-state
    $PWD/target/$profile/recurrency export-genesis-wasm --chain ./res/genesis/local/paseo-local-recurrency-2000-raw.json > ./res/genesis/local/recurrency-paseo-genesis-wasm
    ;;
  *)
    echo "Unknown build step: $build_step"
    exit 1
    ;;

esac
