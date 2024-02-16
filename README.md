# RWS

[![CI Status](https://github.com/AUTOM77/RWS/workflows/ci/badge.svg)](https://github.com/AUTOM77/RWS/actions?query=workflow:ci)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/RWS)](.)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/RWS)](https://github.com/AUTOM77/RWS/issues)

> Unofficial Rust implementation of Cloudflare Warp Proxy with Socks5 support.

## Features

1. Implementation of `eBPF-driven` **Wireguard** in Rust
2. Asynchronous acquisition for WARP Plus Subscription with `tokio`
3. Invocation of the `Socks5` proxy protocol with password protection

## Usage

`rws-cli -d ${DEVICE_ID} -i ${INTERFACE-NAME} -p ${SOCKS5-PORT} -U ${USER:PASSWD}`

e.g:
`rws-cli -d "rws0xFFFF" -i "warp" -p "9091" -U "automa77:passwd" `

## Notice of Non-Affiliation and Disclaimer

- We are not affiliated, associated, authorized, endorsed by, or in any way officially connected with WireGuard, or any of its subsidiaries or its affiliates. The official WireGuard website can be found at <https://www.wireguard.com/>.

- We are not affiliated, associated, authorized, endorsed by, or in any way officially connected with Cloudflare, or any of its subsidiaries or its affiliates. The official Cloudflare website can be found at <https://www.cloudflare.com>.

## License

RWS is open-sourced under [MIT](./LICENSE) license.