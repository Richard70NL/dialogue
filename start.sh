#!/bin/sh
# - Set address to 0.0.0.0 (:: for IPv6) to bind to all IP addresses on the
#   host.
# - Set address to 127.0.0.1 (::1 for IPv6) to bind to the loopback address
#   which limits access to the localhost only (best option during development).
# - Or set address to specific IP.
cargo build
sudo cargo run -- start --verbose --address 127.0.0.1:119
