#!/usr/bin/env bash

set -eux

PACKAGE="${PACKAGE:-frequency-runtime}" #Need to replicate job for all runtimes
RUNTIME_DIR="${RUNTIME_DIR:-runtime/frequency}"
SRT_TOOL_VERSION="${SRT_TOOL_VERSION:-1.62.0}"

# Enable warnings about unused extern crates
export RUSTFLAGS=" -W unused-extern-crates"

./scripts/init.sh install-toolchain

rustc --version
rustup --version
cargo --version

case $TARGET in
	build-node)
		cargo build --release "$@"
		;;

  build-runtime)
    export RUSTC_VERSION=$SRT_TOOL_VERSION
    echo "Building runtime with rustc version $RUSTC_VERSION"
    docker run --rm -e PACKAGE=$PACKAGE -e RUNTIME_DIR=$RUNTIME_DIR -v $PWD:/build -v /tmp/cargo:/cargo-home paritytech/srtool:$RUSTC_VERSION build
    ;;

  tests)
    cargo test --all-features --workspace --release
    ;;

  lint)
    cargo fmt -- --check
    SKIP_WASM_BUILD=1 cargo clippy --all-targets  -- -A clippy::bool_assert_comparison
    ;;
esac
