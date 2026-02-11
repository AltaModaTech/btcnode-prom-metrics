# btcnode-prom-metrics

The best Bitcoin node metrics exporter for Prometheus based on [corepc-client](https://crates.io/crates/corepc-client).

## About corepc-client

The [corepc-client](https://crates.io/crates/corepc-client) crate replaces the previous the Bitcoin node RPC crates that were archived on 11/25/2025:

- [bitcoincore-rpc](https://crates.io/crates/bitcoincore-rpc) [GitHub](https://github.com/rust-bitcoin/rust-bitcoincore-rpc)
- [bitcoincore-rpc-json](https://crates.io/crates/) [GitHub](https://github.com/rust-bitcoin/rust-bitcoincore-rpc/tree/master/json)

The Rust Bitcoin Community's public repository for this crate is at [corepc-client](https://github.com/rust-bitcoin/corepc).

## Code structure

This repository's code is separated into:

- _prom-api_ implements the API for Prometheus to call for gathering metrics.
- _btcnode-metrics_ gathers metrics from the Bitcoin node and transforms them into Prometheus format.
