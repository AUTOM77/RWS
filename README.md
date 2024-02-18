# RWS

[![CI Status](https://github.com/AUTOM77/RWS/workflows/ci/badge.svg)](https://github.com/AUTOM77/RWS/actions?query=workflow:ci)
[![Code Size](https://img.shields.io/github/languages/code-size/AUTOM77/RWS)](.)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Open Issues](https://img.shields.io/github/issues/AUTOM77/RWS)](https://github.com/AUTOM77/RWS/issues)

> Unofficial eBPF-based Rust implementation of Cloudflare Warp Proxy with Socks5 support.

🚧 Building .. 🚧

## Features

1. Implementation **Wireguard** protocol in Rust with `eBPF-driven` optimization
2. Asynchronous acquisition for WARP Plus Subscription with `tokio`
3. Apply the `Socks5` proxy protocol with username/password authentication to enhance privacy and bypass firewalls
4. Enable `curve25519` encryption with `x25519_dalek` implementation of x25519 key exchange algorithm

## Usage

`rws-cli -d ${DEVICE_ID} -i ${INTERFACE-NAME} -p ${SOCKS5-PORT} -U ${USER:PASSWD}`

e.g:
`rws-cli -d "rws0xFFFF" -i "warp" -p "9091" -U "automa77:passwd" `

## Notice of Non-Affiliation and Disclaimer

- We are not affiliated, associated, authorized, endorsed by, or in any way officially connected with WireGuard, or any of its subsidiaries or its affiliates. The official WireGuard website can be found at <https://www.wireguard.com/>.

- We are not affiliated, associated, authorized, endorsed by, or in any way officially connected with Cloudflare, or any of its subsidiaries or its affiliates. The official Cloudflare website can be found at <https://www.cloudflare.com>.

## License

RWS is open-sourced under [MIT](./LICENSE) license.