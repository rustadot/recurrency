UNAME := $(shell uname)

.PHONY: all
all: build

.PHONY: clean
clean:
	cargo clean

.PHONY: start start-recurrency start-recurrency-docker start-manual start-interval start-interval-short start-with-offchain start-recurrency-with-offchain start-manual-with-offchain start-interval-with-offchain
start:
	./scripts/init.sh start-recurrency-instant

start-paseo-relay:
	./scripts/init.sh start-paseo-relay-chain


start-paseo-collator-alice:
	./scripts/init.sh start-paseo-collator-alice

start-paseo-collator-bob:
	./scripts/init.sh start-paseo-collator-bob

start-recurrency:
	./scripts/init.sh start-recurrency

start-recurrency-docker:
	./scripts/init.sh start-recurrency-docker

start-manual:
	./scripts/init.sh start-recurrency-manual

start-interval:
	./scripts/init.sh start-recurrency-interval

start-interval-short:
	./scripts/init.sh start-recurrency-interval 0 1

start-with-offchain:
	./scripts/init.sh start-recurrency-instant with-offchain

start-recurrency-with-offchain:
	./scripts/init.sh start-recurrency with-offchain

start-manual-with-offchain:
	./scripts/init.sh start-recurrency-manual with-offchain

start-interval-with-offchain:
	./scripts/init.sh start-recurrency-interval with-offchain

.PHONY: stop stop-relay stop-recurrency-docker
stop-relay:
	./scripts/init.sh stop-relay-chain

stop-paseo-relay:
	./scripts/init.sh stop-paseo-relay-chain

stop-recurrency-docker:
	./scripts/init.sh stop-recurrency-docker

.PHONY: local-block
local-block:
	curl http://localhost:9944 -H "Content-Type:application/json;charset=utf-8" -d   '{ \
		"jsonrpc":"2.0", \
		"id":1, \
		"method":"engine_createBlock", \
		"params": [true, true] \
		}' | jq

.PHONY: register
register:
	./scripts/init.sh register-recurrency-paseo-local

.PHONY: onboard
onboard:
	./scripts/init.sh onboard-recurrency-paseo-local

.PHONY: offboard
offboard:
	./scripts/init.sh offboard-recurrency-paseo-local

.PHONY: specs-testnet-2000 specs-paseo-local
specs-testnet-2000:
	./scripts/generate_specs.sh 2000 paseo-2000 release

specs-paseo-local:
	./scripts/generate_relay_specs.sh

.PHONY: format
format:
	cargo +nightly-2024-08-01 fmt

.PHONY: lint lint-audit
lint:
	cargo +nightly-2024-08-01 fmt --check
	SKIP_WASM_BUILD=1 env -u RUSTFLAGS cargo clippy --features runtime-benchmarks,recurrency-lint-check -- -D warnings
	RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="--enable-index-page --check -Zunstable-options" cargo doc --no-deps --features recurrency

lint-audit:
	cargo deny check -c deny.toml

.PHONY: format-lint
format-lint: format lint

.PHONY: ci-local
ci-local: check lint lint-audit test js e2e-tests

.PHONY: upgrade-local upgrade-no-relay
upgrade-local:
	./scripts/init.sh upgrade-recurrency-paseo-local

upgrade-no-relay:
	./scripts/init.sh upgrade-recurrency-no-relay


#
# We use hard-coded variables (rather than a pattern) so that smart shells with
# CLI auto-complete for Makefiles will pick up the targets and present as options for auto-complete.
#
# Note: to run benchmarks for > 1 but < all pallets, it is more efficient to call the `run_benchmarks.sh`
#       script directly, as it is able to run the build stage just once for all benchmarks
#
BENCH_TARGETS=\
benchmarks-messages \
benchmarks-msa \
benchmarks-recurrency-tx-payment \
benchmarks-overhead \
benchmarks-schemas \
benchmarks-stateful-storage \
benchmarks-handles \
benchmarks-time-release \
benchmarks-passkey \
benchmarks-pallet_balances \
benchmarks-pallet_collator_selection \
benchmarks-pallet_democracy \
benchmarks-pallet_multisig \
benchmarks-pallet_preimage \
benchmarks-pallet_scheduler \
benchmarks-pallet_session \
benchmarks-pallet_timestamp \
benchmarks-pallet_treasury \
benchmarks-pallet_utility \
benchmarks-pallet_proxy

BENCH_LOCAL_TARGETS=\
benchmarks-messages-local \
benchmarks-msa-local \
benchmarks-overhead-local \
benchmarks-schemas-local \
benchmarks-recurrency-tx-payment-local \
benchmarks-stateful-storage-local \
benchmarks-handles-local \
benchmarks-passkey-local \
benchmarks-time-release-local \
benchmarks-pallet_balances-local \
benchmarks-pallet_collator_selection-local \
benchmarks-pallet_collective-local \
benchmarks-pallet_democracy-local \
benchmarks-pallet_multisig-local \
benchmarks-pallet_preimage-local \
benchmarks-pallet_scheduler-local \
benchmarks-pallet_session-local \
benchmarks-pallet_timestamp-local \
benchmarks-pallet_treasury-local \
benchmarks-pallet_utility-local \
benchmarks-pallet_proxy-local

.PHONY: benchmarks
benchmarks:
	./scripts/run_benchmarks.sh

#
# Target to run benchmarks for local development. Uses the "bench-dev" profile,
# since "release" is unnecessary in local development, and by using "bench-dev"
# (which is just a clone of "release"), we don't overwrite our "release" target used
# for development testing.
.PHONY: benchmarks-local
benchmarks-local:
	./scripts/run_benchmarks.sh -t bench-dev

.PHONY: $(BENCH_TARGETS)
$(BENCH_TARGETS):
	./scripts/run_benchmarks.sh $(@:benchmarks-%=%)

.PHONY: $(BENCH_LOCAL_TARGETS)
$(BENCH_LOCAL_TARGETS):
	./scripts/run_benchmarks.sh -t bench-dev $(@:benchmarks-%-local=%)

#
# benchmarks-multi-* targets are for ease of use in running benchmarks for multiple
# (but not necessarily all) pallets with a single invocation.
#
.PHONY: benchmarks-multi
benchmarks-multi:
	./scripts/run_benchmarks.sh $(PALLETS)

.PHONY: benchmarks-multi-local
benchmarks-multi-local:
	./scripts/run_benchmarks.sh -t bench-dev $(PALLETS)

benchmarks-capacity:
	./scripts/run_benchmark.sh -p capacity

.PHONY: docs
docs:
	RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo doc --no-deps --workspace --features recurrency

# Cleans unused docker resources and artifacts
.PHONY: docs
docker-prune:
	./scripts/prune_all.sh

.PHONY: check check-no-relay check-local check-testnet check-mainnet
check:
	SKIP_WASM_BUILD= cargo check --features runtime-benchmarks,recurrency-lint-check

check-no-relay:
	SKIP_WASM_BUILD= cargo check --features recurrency-no-relay

check-local:
	SKIP_WASM_BUILD= cargo check --features recurrency-paseo-local

check-testnet:
	SKIP_WASM_BUILD= cargo check --features recurrency-testnet

check-mainnet:
	SKIP_WASM_BUILD= cargo check --features recurrency

.PHONY: js
js:
	./scripts/generate_js_definitions.sh

.PHONY: build build-benchmarks build-no-relay build-local build-testnet build-mainnet build-testnet-release build-mainnet-release
build:
	cargo build --features recurrency-no-relay

build-benchmarks:
	cargo build --release --features runtime-benchmarks,recurrency-lint-check --workspace

build-no-relay:
	cargo build --features recurrency-no-relay

build-local:
	cargo build --features recurrency-local

build-testnet:
	cargo build --features recurrency-testnet

build-mainnet:
	cargo build --features recurrency

build-testnet-release:
	cargo build --locked --features recurrency-testnet --release

build-mainnet-release:
	cargo build --locked --features  recurrency --release

.PHONY: test e2e-tests e2e-tests-serial e2e-tests-only e2e-tests-load e2e-tests-load-only e2e-tests-testnet-paseo e2e-tests-paseo-local
test:
	cargo test --workspace --features runtime-benchmarks,recurrency-lint-check

e2e-tests:
	./scripts/run_e2e_tests.sh

e2e-tests-serial:
	./scripts/run_e2e_tests.sh -c serial

e2e-tests-only:
	./scripts/run_e2e_tests.sh -s

e2e-tests-load:
	./scripts/run_e2e_tests.sh load

e2e-tests-load-only:
	./scripts/run_e2e_tests.sh -s load

e2e-tests-testnet-paseo:
	./scripts/run_e2e_tests.sh -c paseo_testnet

e2e-tests-paseo-local:
	./scripts/run_e2e_tests.sh -c paseo_local

check-try-runtime-installed:
	@which try-runtime > /dev/null || (echo "try-runtime is not installed. Please install it" && exit 1)

.PHONY: try-runtime-create-snapshot-mainnet try-runtime-upgrade-mainnet try-runtime-use-snapshot-mainnet try-runtime-create-snapshot-paseo-testnet try-runtime-use-snapshot-paseo-testnet try-runtime-upgrade-paseo-testnet

try-runtime-create-snapshot-paseo-testnet: check-try-runtime-installed
	try-runtime create-snapshot --uri wss://0.rpc.testnet.amplica.io:443 testnet-paseo-all-pallets.state

# mainnet snapshot takes as many as 24 hours to complete
try-runtime-create-snapshot-mainnet: check-try-runtime-installed
	try-runtime create-snapshot --uri wss://1.rpc.recurrency.xyz:443 mainnet-all-pallets.state

try-runtime-upgrade-paseo-testnet: check-try-runtime-installed
	cargo build --release --features recurrency-testnet,try-runtime && \
	try-runtime --runtime ./target/release/wbuild/recurrency-runtime/recurrency_runtime.wasm on-runtime-upgrade live --uri wss://0.rpc.testnet.amplica.io:443

try-runtime-upgrade-mainnet: check-try-runtime-installed
	cargo build --release --features recurrency,try-runtime && \
	try-runtime --runtime ./target/release/wbuild/recurrency-runtime/recurrency_runtime.wasm on-runtime-upgrade live --uri wss://1.rpc.recurrency.xyz:443

try-runtime-use-snapshot-paseo-testnet: check-try-runtime-installed
	cargo build --release --features recurrency-testnet,try-runtime && \
	try-runtime --runtime ./target/release/wbuild/recurrency-runtime/recurrency_runtime.wasm on-runtime-upgrade snap --path testnet-paseo-all-pallets.state

try-runtime-use-snapshot-mainnet: check-try-runtime-installed
	cargo build --release --features recurrency,try-runtime && \
	try-runtime --runtime ./target/release/wbuild/recurrency-runtime/recurrency_runtime.wasm on-runtime-upgrade snap --path mainnet-all-pallets.state

try-runtime-check-migrations-paseo-testnet: check-try-runtime-installed
	cargo build --release --features recurrency-testnet,try-runtime -q --locked && \
	try-runtime --runtime ./target/release/wbuild/recurrency-runtime/recurrency_runtime.wasm on-runtime-upgrade --checks="pre-and-post" --disable-spec-version-check --no-weight-warnings live --uri wss://0.rpc.testnet.amplica.io:443
# Pull the Polkadot version from the polkadot-cli package in the Cargo.lock file.
# This will break if the lock file format changes
POLKADOT_VERSION=$(shell grep -o 'release-polkadot-v[0-9.]*' Cargo.toml | sed 's/release-polkadot-v//' | head -n 1)

.PHONY: version
version:
ifndef v
	@echo "Please set the version with v=#.#.#[-#]"
	@exit 1
endif
ifneq (,$(findstring v,  $(v)))
	@echo "Please don't prefix with a 'v'. Use: v=#.#.#[-#]"
	@exit 1
endif
ifeq (,$(POLKADOT_VERSION))
	@echo "Error: Having trouble finding the Polkadot version. Sorry about that.\nCheck my POLKADOT_VERSION variable command."
	@exit 1
endif
	@echo "Setting the crate versions to "$(v)+polkadot$(POLKADOT_VERSION)
ifeq ($(UNAME), Linux)
	$(eval $@_SED := -i -e)
endif
ifeq ($(UNAME), Darwin)
	$(eval $@_SED := -i '')
endif
	find . -type f -name "Cargo.toml" -print0 | xargs -0 sed $($@_SED) 's/^version = \"0\.0\.0\"/version = \"$(v)+polkadot$(POLKADOT_VERSION)\"/g';
	@echo "Doing cargo check for just examples seems to be the easiest way to update version in Cargo.lock"
	cargo check --examples --quiet
	@echo "All done. Don't forget to double check that the automated replacement worked."

.PHONY: version-polkadot
version-polkadot:
ifeq (,$(POLKADOT_VERSION))
	@echo "Error: Having trouble finding the Polkadot version. Sorry about that.\nCheck my POLKADOT_VERSION variable command."
	@exit 1
endif
	@echo $(POLKADOT_VERSION)

.PHONY: version-reset
version-reset:
	find ./ -type f -name 'Cargo.toml' -exec sed -i '' 's/^version = \".*+polkadot.*\"/version = \"0.0.0\"/g' {} \;

.PHONY: genesis-schemas
genesis-schemas:
	cd tools/genesis-data && \
	npm i && \
	npm run --silent schemas > ../../resources/genesis-schemas.json
