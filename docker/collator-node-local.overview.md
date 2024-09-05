# Recurrency Collator Node for Local Relay Chain

Runs one collator node that connects to the local relay chain consisting of two validator nodes.

![](https://github.com/rustadot/recurrency/blob/main/docs/images/local-dev-env-option-2.jpg?raw=true)

## Run

1.  Checkout project and generate local spec

    ```
    git clone git@github.com:rustadot/recurrency.git
    ```

1.  Start relay chain and collator node

    ```sh

    make start-recurrency-docker
    ```

1.  Stop all nodes

    ```sh
    make stop-recurrency-docker
    ```

| **Node**             | **Ports**                       | **Explorer URL**                                                                          |
| -------------------- | ------------------------------- | ----------------------------------------------------------------------------------------- |
| Recurrency Relay Node | ws and rpc: `9944`, p2p:`30333` | [127.0.0.1:9944](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer) |
| Alice Relay Node     | ws and rpc: `9946`, p2p:`30335` | [127.0.0.1:9946](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9946#/explorer) |
| Bob Relay Node       | ws and rpc: `9947`, p2p:`30336` | [127.0.0.1:9947](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9947#/explorer) |

```

```
