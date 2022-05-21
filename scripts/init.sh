#!/usr/bin/env bash

set -e

cmd=$1
chain_spec="${RAW_PARACHAIN_CHAIN_SPEC:-./res/genesis/rococo-local-mrc-2000-raw.json}"
# The runtime we want to use
parachain="${PARA_CHAIN_CONFIG:-mrc-local}"
# The parachain Id we want to use
para_id="${PARA_ID:-2000}"
# The tmp base directory
base_dir=/tmp/mrc
# Option to use the Docker image to export state & wasm
docker_onboard="${DOCKER_ONBOARD:-false}"
mrc_docker_image_tag="${PARA_DOCKER_IMAGE_TAG:-mrc-latest}"

chain="${RELAY_CHAIN_SPEC:-./res/rococo-local.json}"

case $cmd in
install-toolchain)
  ./scripts/install_toolchain.sh
  ;;

start-relay-chain)
  echo "Starting local relay chain with Alice and Bob..."
  docker-compose -f ./docker-compose-local-relay.yml up -d
  ;;

stop-relay-chain)
  echo "Stopping relay chain..."
  docker-compose -f ./docker-compose-local-relay.yml down
  ;;

start-mrc-docker)
  echo "Starting mrc container with Alice..."
  docker-compose -f ./docker-compose-local-chain.yml up -d
  ;;

stop-mrc-docker)
  echo "Stopping mrc container with Alice..."
  docker-compose -f ./docker-compose-local-chain.yml down
  ;;

start-mrc)
  printf "\nBuilding mrc with runtime '$parachain' and id '$para_id'...\n"
  cargo build --release

  parachain_dir=$base_dir/parachain/${para_id}
  mkdir -p $parachain_dir;

  if [ "$2" == "purge" ]; then
    echo "purging parachain..."
    rm -rf $parachain_dir
  fi

  ./scripts/run_collator.sh \
    --chain="${chain_spec}" --alice \
    --base-path=$parachain_dir/data \
    --wasm-execution=compiled \
    --execution=wasm \
    --force-authoring \
    --port $((30355 + $para_id)) \
    --rpc-port $((9936 + $para_id)) \
    --ws-port $((9946 + $para_id)) \
    --rpc-external \
    --rpc-cors all \
    --ws-external \
    --rpc-methods=Unsafe \
    --state-cache-size 0 \
  ;;

register-mrc)
  echo "reserving and registering parachain with relay via first available slot..."
  
  cd scripts/js/onboard
  yarn && yarn register "ws://0.0.0.0:9944" "//Alice"
  ;;

onboard-mrc)
  echo "Onboarding parachain with runtime '$parachain' and id '$para_id'..."

   onboard_dir="$base_dir/onboard"
   mkdir -p $onboard_dir

   wasm_location="$onboard_dir/${parachain}-${para_id}.wasm"
    if [ "$docker_onboard" == "true" ]; then
      genesis=$(docker run -it {REPO_NAME}/mrc:${mrc_docker_image_tag} export-genesis-state --chain="${chain_spec}")
      docker run -it {REPO_NAME}/mrc:${mrc_docker_image_tag} export-genesis-wasm --chain="${chain_spec}" > $wasm_location
    else
      genesis=$(./target/release/mrc-collator export-genesis-state --chain="${chain_spec}")
      ./target/release/mrc-collator export-genesis-wasm --chain="${chain_spec}" > $wasm_location
    fi

  echo "WASM path:" "${parachain}-${para_id}.wasm"

  cd scripts/js/onboard
  yarn && yarn onboard "ws://0.0.0.0:9944" "//Alice" ${para_id} "${genesis}" $wasm_location
  ;;

offboard-mrc)
  echo "cleaning up parachain for id '$para_id'..."
  
  cd scripts/js/onboard
  yarn && yarn cleanup "ws://0.0.0.0:9944" "//Alice" ${para_id}
  ;;
esac
