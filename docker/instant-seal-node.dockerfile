# Docker image for running Frequency parachain node container (with collating)
# locally in instant seal mode. Requires to run from repository root and to copy
# the binary in the build folder.
# This is the build stage for Polkadot. Here we create the binary in a temporary image.
FROM --platform=linux/amd64 ubuntu:focal AS base

LABEL maintainer="Frequency Team"
LABEL description="Frequency collator node in instant seal mode"

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

# This is the 2nd stage: a very small image where we copy the Frequency binary
FROM --platform=linux/amd64 ubuntu:focal

RUN useradd -m -u 1000 -U -s /bin/sh -d /frequency frequency && \
	mkdir -p /chain-data /frequency/.local/share && \
	chown -R frequency:frequency /chain-data && \
	ln -s /chain-data /frequency/.local/share/frequency

USER frequency

COPY --from=base /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
# For local testing only
# COPY --chown=frequency target/release/frequency.amd64 ./frequency/frequency
COPY --chown=frequency target/release/frequency ./frequency/

# 9933 P2P port
# 9944 for RPC call
# 30333 for Websocket
EXPOSE 9933 9944 30333

VOLUME ["/chain-data"]

ENTRYPOINT ["/frequency/frequency", \
	# Required params for starting the chain
	"--dev", \
	"-lruntime=debug", \
	"--instant-sealing", \
	"--wasm-execution=compiled", \
	"--execution=wasm", \
	"--no-telemetry", \
	"--no-prometheus", \
	"--port=30333", \
	"--rpc-port=9933", \
	"--ws-port=9944", \
	"--rpc-external", \
	"--rpc-cors=all", \
	"--ws-external", \
	"--rpc-methods=Unsafe", \
	"--tmp" \
	]

# Params which can be overriden from CLI
# CMD ["", "", ...]
