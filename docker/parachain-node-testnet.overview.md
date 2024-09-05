# Recurrency Parachain Node for Testnets

Recurrency parachain node which connects to Recurrency testnets:

- Recurrency Paseo Testnet `--chain=recurrency-paseo` (Default)

To view all available options and arguments:

```sh
docker run --rm recurrencychain/parachain-node-testnet:<version.tag> --help
```

## Run Full Node

### Recurrency Paseo Testnet

Start full chain node that connects to Paseo Testnet network and syncs with warp:

```sh
docker run -p 9944:9944 -p 30333:30333 recurrencychain/parachain-node-testnet:<version.tag> \
    --chain=recurrency-paseo \
    --base-path=/chain-data \
    --rpc-external \
    --rpc-cors=all \
    --rpc-methods=safe \
    --sync=warp \
    -- \
    --sync=warp
```

## Storage

Remember that parachain nodes contain a full node of the relay chain as well, so plan available storage size accordingly.

Using [Volumes](https://docs.docker.com/storage/volumes/) or [Bind Mounts](https://docs.docker.com/storage/bind-mounts/) is suggested to maintain the `--base-path` between restarts.
