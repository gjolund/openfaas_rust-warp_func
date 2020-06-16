OpenFaaS Deno HTTP function
=============================================

An OpenFaaS of-watchdog function written for Deno.

## Installation

```sh
faas template pull https://github.com/austinrivas/rust-warp-template
```

## Create Function

```sh
faas new <name> --lang rust-warp
```

## Testing

```sh
cargo test --manifest-path ./function/Cargo.toml
```

### Format

```sh
cargo fmt --manifest-path ./function/Cargo.toml
```

### Linting

```sh
cargo clippy --manifest-path ./function/Cargo.toml
```

## Deployment

```sh
faas up -f function.yml --gateway $GATEWAY_URL
```

## Remote Dev

This function includes an `okteto.yml` function to facilitate remote dev and debugging.

```bash
cd function
okteto up
 ✓  Development environment activated
 ✓  Files synchronized
    Namespace: austinrivas
    Name:      rust-hello
    Forward:   8080 -> 8080
               9229 -> 9229
okteto> fwatchdog
```

This will compile the function in the remote dev environment. Rust is fast code that compiles slow, so patience is a virtue here. Okteto will cache the build so that later compilations will be much faster.

Okteto will syncronize local changes with the remote environment.

Currently remote debugging is not implemented, however it is a future goal of this project.

## [Template](https://github.com/austinrivas/rust-warp-template)

This function is based on the OpenFaaS [rust-warp-template](https://github.com/austinrivas/rust-warp-template).

This template provides a thin wrapper around the [Rust Warp Server](https://github.com/seanmonstar/warp).

## [Function Handler](https://github.com/austinrivas/openfaas_rust-warp_func/blob/master/function/src/lib.rs)

## Extras

This repo also contains an [Okteto Remote Development Configuration](https://github.com/austinrivas/openfaas_rust-warp_func/blob/master/function/okteto.yml) for use on the [Okteto Platform](https://okteto.com/).

A [github action](https://github.com/austinrivas/openfaas_rust-warp_func/blob/master/.github/workflows/test-deno.yml) is included that will trigger on pull request. This action runs the rust tests / lint / check / format.
