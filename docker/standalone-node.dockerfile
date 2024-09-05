# Docker image for running Recurrency parachain node container (with collating)
# locally as a standalone node. Requires to run from repository root and to copy
# the binary in the build folder.
# This is the build stage for Polkadot. Here we create the binary in a temporary image.
FROM --platform=linux/amd64 ubuntu:22.04 AS base

LABEL maintainer="Recurrency"
LABEL description="Recurrency standalone node"

RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates

# This is the 2nd stage: a very small image where we copy the Recurrency binary
FROM --platform=linux/amd64 ubuntu:22.04

# We want jq and curl in the final image, but we don't need the support files
RUN apt-get update && \
	apt-get install -y jq curl && \
	apt-get clean && \
	rm -rf /usr/share/doc /usr/share/man /usr/share/zsh

RUN useradd -m -u 1000 -U -s /bin/sh -d /recurrency recurrency && \
	mkdir -p /data /recurrency/.local/share && \
	chown -R recurrency:recurrency /data && \
	ln -s /data /recurrency/.local/share/recurrency

USER recurrency

COPY --from=base /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
# For local testing only
# COPY --chown=recurrency target/x86_64-unknown-linux-gnu/debug/recurrency ./recurrency/recurrency
COPY --chown=recurrency target/release/recurrency ./recurrency/
COPY --chown=recurrency docker/recurrency-start.sh ./recurrency/
RUN chmod +x ./recurrency/recurrency ./recurrency/recurrency-start.sh

# 9944 for RPC call
EXPOSE 9944

VOLUME ["/data"]

ENTRYPOINT [ "/recurrency/recurrency-start.sh" ]
