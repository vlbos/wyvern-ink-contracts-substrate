#!/usr/bin/env bash

# set -eu

case "$1" in
"au")  cargo  contract build --manifest-path authenticated_proxy/Cargo.toml ;;
"o") cargo  contract build --manifest-path ownable_delegate_proxy/Cargo.toml ;;
"a") cargo  contract build --manifest-path wyvern_atomicizer/Cargo.toml ;;
"p")  cargo  contract build --manifest-path wyvern_proxy_registry/Cargo.toml ;;
"t") cargo  contract build --manifest-path wyvern_token_transfer_proxy/Cargo.toml ;;
*) cargo  contract build --manifest-path authenticated_proxy/Cargo.toml  && \
cargo  contract build --manifest-path ownable_delegate_proxy/Cargo.toml && \
cargo  contract build --manifest-path wyvern_atomicizer/Cargo.toml && \
cargo  contract build --manifest-path wyvern_proxy_registry/Cargo.toml && \
cargo  contract build --manifest-path wyvern_token_transfer_proxy/Cargo.toml ;;
esac

