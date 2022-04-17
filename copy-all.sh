#!/usr/bin/env bash

# set -eu
srcpath=/Users/lisheng/mygit/vlbos/wyvern-ink-contracts-substrate/

destpath=/Users/lisheng/mygit/vlbos/pacific-api-orderbook-backend-contract7/packages/wyvern-js/abisv2/
addrs=(authenticated_proxy ownable_delegate_proxy wyvern_atomicizer wyvern_proxy_registry wyvern_token_transfer_proxy)
# srcpath2=/Users/lisheng/mygit/vlbos/ink/examples/
# ercaddrs=(erc20 erc721)
# for i in 1 2; do cp -f ${srcpath2}${ercaddrs[i-1]}/target/ink/metadata.json ${destpath}/${addrs[i-1]}/ ;cp -f ${srcpath2}${ercaddrs[i-1]}/target/ink/*.wasm ${destpath}/${addrs[i-1]}/ ; cp -f ${srcpath2}${ercaddrs[i-1]}/target/ink/*.contract ${destpath}/${addrs[i-1]}/ ; done    # 为每一个账号输入口令
# for i in 1 2 3 4 5; do mkdir ${destpath}/${addrs[i-1]}; done   
for i in 1 2 3 4 5; do cp -f ${srcpath}${addrs[i-1]}/target/ink/metadata.json ${destpath}/${addrs[i-1]}/ ;cp -f ${srcpath}${addrs[i-1]}/target/ink/*.wasm ${destpath}/${addrs[i-1]}/ ; cp -f ${srcpath}${addrs[i-1]}/target/ink/*.contract ${destpath}/${addrs[i-1]}/ ; done    # 为每一个账号输入口令


# case "$1" in
# "au")  cargo  contract build --manifest-path authenticated_proxy/Cargo.toml ;;
# "o") cargo  contract build --manifest-path ownable_delegate_proxy/Cargo.toml ;;
# "a") cargo  contract build --manifest-path wyvern_atomicizer/Cargo.toml ;;
# "p")  cargo  contract build --manifest-path wyvern_proxy_registry/Cargo.toml ;;
# "t") cargo  contract build --manifest-path wyvern_token_transfer_proxy/Cargo.toml ;;
# *) cargo  contract build --manifest-path authenticated_proxy/Cargo.toml  && \
# cargo  contract build --manifest-path ownable_delegate_proxy/Cargo.toml && \
# cargo  contract build --manifest-path wyvern_atomicizer/Cargo.toml && \
# cargo  contract build --manifest-path wyvern_proxy_registry/Cargo.toml && \
# cargo  contract build --manifest-path wyvern_token_transfer_proxy/Cargo.toml ;;
# esac

